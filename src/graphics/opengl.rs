use gl;
use gl::types::*;
use std::ffi::{CString, CStr};
use std::{ptr, mem};
use std::os::raw::c_void;
use std::collections::{HashMap, BTreeMap};
use image::{self, DynamicImage};
use math::Transform2D;
use cgmath::prelude::*;
use cgmath::{self, Vector2, Vector3, Vector4, Matrix4};
use graphics::shader::CustomShaderUniform;
use graphics::job::{RenderJob, ParticleRenderable, SpriteRenderable, TextRenderable};
use resource::ResourceManager;
use std::str;
use graphics::font::{GlyphData, FontManager};
use rusttype::{self, FontCollection, Font, Scale, point, PositionedGlyph};
use rusttype::{GlyphId, Codepoint, CodepointOrGlyphId};

#[allow(non_snake_case)]
pub struct OpenGLRenderer {
    render_height : GLuint,
    render_width : GLuint,

    sprite_vao : GLuint,
    sprite_vbo : GLuint,
    sprite_ebo : GLuint,

    projection_2d : Matrix4<f32>,

    preprocess_shader_key : Option<String>,
    main_fbo : GLuint,
    ms_fbo : GLuint,
    render_texture : GLuint,

    shaders : HashMap<String, GLuint>,
    textures : HashMap<String, GLuint>,
    glyphs : HashMap<String, GlyphData>
}

impl OpenGLRenderer  {

    pub fn new(screen_width : u32, screen_height : u32) -> OpenGLRenderer {
        unsafe {
            let (sprite_vao, sprite_vbo, sprite_ebo) = OpenGLRenderer::create_sprite_buffer();
            let (render_texture, ms_fbo, main_fbo) = OpenGLRenderer::initialize_postprocessing(screen_width, screen_height);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            let projection_2d = cgmath::ortho(0.0, screen_width as f32,screen_height as f32, 0.0, -0.1, 0.1);

            let mut renderer = OpenGLRenderer {
                render_height : screen_height,
                render_width : screen_width,

                sprite_vao,
                sprite_vbo,
                sprite_ebo,

                main_fbo,
                ms_fbo,
                render_texture,
                preprocess_shader_key: None,

                projection_2d,
                shaders : HashMap::new(),
                textures : HashMap::new(),
                glyphs : HashMap::new(),
            };

            return renderer;
        }
    }

    pub fn register_font(&mut self, font_key : &str, font_size : u32, font : &Font<'static>){

        unsafe { gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1); }
        let scale = Scale { x : font_size as f32, y : font_size as f32};
        let v_metrics = font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);
        let generated_string = String::from(r#"ABCDEFGHIJKLMNOPQRSTUVWXYZ
            abcdefghijklmnopqrstuvwxyz123456789!:., "#);
        //let generated_string = String::from("This is");
        for c in generated_string.chars() {

            let glyph = font.glyph(c).unwrap().scaled(scale).positioned(rusttype::Point{x: 0.0, y: 0.0});
            let gid = match c.into() {
                CodepointOrGlyphId::Codepoint(Codepoint(c)) => c,
                CodepointOrGlyphId::GlyphId(GlyphId(gid)) => gid
            };

            let key = OpenGLRenderer::get_glyph_texture_key(font_key, font_size, gid);

            //rasterize glyph
            if let Some(bb) = glyph.pixel_bounding_box(){
                let mut pixels : Vec<u8> = vec![0; (bb.width() * bb.height()) as usize];
                glyph.draw(|x, y, v| {
                    let v = ((v * 255.0) + 0.5).floor().max(0.0).min(255.0) as u8;
                    pixels[(x + y * bb.width() as u32) as usize] = v;
                });


                //uncomment for debug
//                for i in 0..pixels.len() {
//                    if pixels[i] <= 64 {
//                        print!(" ");
//                    }
//                        else { print!("b"); }
//                    if i != 0 && (i as i32 % bb.width() as i32) == 0 {
//                        print!("\n");
//                    }
//                }

                let glyph = glyph.unpositioned();
                self.glyphs.insert(key.clone(), GlyphData::new(
                    gid,
                    Some(key.clone()),
                    Vector2::new(bb.width() as f32, bb.height() as f32),
                    Vector2::new(glyph.h_metrics().left_side_bearing,
                                 v_metrics.ascent),
                    glyph.h_metrics().advance_width,
                ));
                self.register_image_byte(&key, bb.width() as u32, bb.height() as u32, pixels.as_slice());
            } else {

                //Have no image (maybe a space
                self.glyphs.insert(key.clone(), GlyphData::new(
                    gid,
                    None,
                    Vector2::new(0.0, 0.0),
                    Vector2::new(glyph.unpositioned().h_metrics().left_side_bearing, v_metrics.ascent),
                    glyph.unpositioned().h_metrics().advance_width,
                ));
            }

        }

        unsafe { gl::PixelStorei(gl::PACK_ALIGNMENT, 1); }
    }

    pub fn register_shader(&mut self, key : &str, vertex_shader_source : &str, fragment_shader_source : &str) {

        unsafe {
            // vertex shader
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            // check for shader compile errors
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&info_log));
            }

            // fragment shader
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            // check for shader compile errors
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&info_log));
            }

            // link shaders
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            // check for linking errors
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&info_log));
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            self.shaders.insert(String::from(key), shader_program);
        }
    }

    fn get_glyph_texture_key(font_key : &str, font_size : u32, id : u32) -> String {
        format!("__{}_{}_{}", font_key, font_size, id)
    }

    pub fn register_uniforms(&self, shader_key: &str, custom_uniforms : &CustomShaderUniform){
        let shader_id = self.shaders.get(shader_key).unwrap();

        unsafe {
            gl::UseProgram(*shader_id);
            self.pass_uniforms(*shader_id, custom_uniforms);
        }
    }

    unsafe fn pass_uniforms(&self, shader_id : GLuint, custom_uniforms : &CustomShaderUniform) {
        use graphics::shader::ShaderUniform::*;

        for (key, uniform) in &custom_uniforms.uniforms {
            match uniform {
                &Float1vArray(ref floats) => {
                    gl::Uniform1fv(gl::GetUniformLocation(shader_id, CString::new(key.as_str()).unwrap().as_ptr()),
                                   floats.len() as i32, floats.as_ptr());
                },
                &Float2vArray(ref floats) => {
                    gl::Uniform2fv(gl::GetUniformLocation(shader_id, CString::new(key.as_str()).unwrap().as_ptr()),
                                   floats.len() as i32, floats.as_ptr() as *const f32);
                },
                &Integer1vArray(ref integers) => {
                    gl::Uniform1iv(gl::GetUniformLocation(shader_id, CString::new(key.as_str()).unwrap().as_ptr()),
                                   integers.len() as i32, integers.as_ptr());
                }
                &Boolean(ref boolean) => {
                    gl::Uniform1i(gl::GetUniformLocation(shader_id, CString::new(key.as_str()).unwrap().as_ptr()),
                                    if *boolean { 1 } else { 0 });
                }
                &Double(ref double) => {
                    gl::Uniform1d(gl::GetUniformLocation(shader_id, CString::new(key.as_str()).unwrap().as_ptr()),
                                    *double);
                }
                &Float(ref float) => {
                    gl::Uniform1f(gl::GetUniformLocation(shader_id, CString::new(key.as_str()).unwrap().as_ptr()),
                                  *float);
                }
            }
        }
    }

    pub fn register_preprocessor(&mut self, key : &str){
        self.preprocess_shader_key = Some(String::from(key));
    }

    pub fn register_image_byte(&mut self, key : &str, width : u32, height : u32, data : &[u8]) {
        unsafe {
            let mut texture = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RED as i32,
                           width as i32,
                           height as i32,
                           0,
                           gl::RED,
                           gl::UNSIGNED_BYTE,
                           &data[0] as *const u8 as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);

            self.textures.insert(String::from(key), texture);
        }
    }

    pub fn register_image(&mut self, key: &str, img : &DynamicImage){
        unsafe {
            let mut texture = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            let data = img.raw_pixels();

            match img {
                &image::ImageLuma8(_) => unimplemented!(),
                &image::ImageLumaA8(_) => unimplemented!(),
                &image::ImageRgb8(ref img) => {
                    gl::TexImage2D(gl::TEXTURE_2D,
                                   0,
                                   gl::RGB as i32,
                                   img.width() as i32,
                                   img.height() as i32,
                                   0,
                                   gl::RGB,
                                   gl::UNSIGNED_BYTE,
                                   &data[0] as *const u8 as *const c_void);
                    gl::GenerateMipmap(gl::TEXTURE_2D);

                    self.textures.insert(String::from(key), texture);
                },
                &image::ImageRgba8(ref img) => {
                    gl::TexImage2D(gl::TEXTURE_2D,
                                   0,
                                   gl::RGBA as i32,
                                   img.width() as i32,
                                   img.height() as i32,
                                   0,
                                   gl::RGBA,
                                   gl::UNSIGNED_BYTE,
                                   &data[0] as *const u8 as *const c_void);
                    gl::GenerateMipmap(gl::TEXTURE_2D);

                    self.textures.insert(String::from(key), texture);
                }
            }
        }
    }

    pub fn delete_buffers(&mut self){
        unsafe {
            gl::DeleteVertexArrays(1, &self.sprite_vao);
            gl::DeleteBuffers(1, &self.sprite_vbo);
            gl::DeleteBuffers(1, &self.sprite_ebo);
        }
    }

    pub fn render_sprite(&mut self, sprites : Vec<(Transform2D, SpriteRenderable)>){
        unsafe {
            gl::BindVertexArray(self.sprite_vao);

            for (transform2d, sprite) in sprites {

                let shader = self.shaders.get("sprite").unwrap();
                gl::UseProgram(*shader);

                let textures_keys = sprite.get_texture_keys();
                for i in 0..textures_keys.len() {
                    let texture = self.textures.get(&textures_keys[i]).unwrap();
                    gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                    gl::BindTexture(gl::TEXTURE_2D, *texture);
                }

                let transform_loc = gl::GetUniformLocation(*shader, CString::new("projection").unwrap().as_ptr());
                gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, self.projection_2d.as_ptr());

                let model_loc = gl::GetUniformLocation(*shader, CString::new("model").unwrap().as_ptr());
                gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, transform2d.get_matrix().as_ptr());

                let color_loc = gl::GetUniformLocation(*shader, CString::new("color").unwrap().as_ptr());
                let color = sprite.get_sprite_color();
                gl::Uniform4f(color_loc, color.x, color.y, color.z, color.w);

                if let &Some(ref uniforms) = sprite.get_shader_uniforms() {
                    self.pass_uniforms(*shader, &uniforms);
                }

                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            }
        }
    }

    pub fn render_particles(&mut self, particlejobs : Vec<(Transform2D, ParticleRenderable, SpriteRenderable)>){
        unsafe {
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
            gl::BindVertexArray(self.sprite_vao);

            for particlejob in particlejobs {

                let(transform2d, particle, sprite) = particlejob;
                let shader = self.shaders.get(sprite.get_shader_key()).unwrap();

                gl::UseProgram(*shader);

                let textures_keys = sprite.get_texture_keys();
                for i in 0..textures_keys.len() {
                    let texture = self.textures.get(&textures_keys[i]).unwrap();
                    gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                    gl::BindTexture(gl::TEXTURE_2D, *texture);
                }

                let transform_loc = gl::GetUniformLocation(*shader, CString::new("projection").unwrap().as_ptr());
                gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, self.projection_2d.as_ptr());

                let model_loc = gl::GetUniformLocation(*shader, CString::new("model").unwrap().as_ptr());
                gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, transform2d.get_matrix().as_ptr());

                let offset_loc = gl::GetUniformLocation(*shader, CString::new("offset").unwrap().as_ptr());
                let offset = Vector2::from_value(0.0);
                gl::Uniform2f(offset_loc, offset.x, offset.y);

                let color_loc = gl::GetUniformLocation(*shader, CString::new("color").unwrap().as_ptr());
                let color = sprite.get_sprite_color();
                gl::Uniform4f(color_loc, color.x, color.y, color.z, particle.life);

                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            }

            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)
        }
    }

    pub fn render_text(&mut self, textjobs : Vec<(Transform2D, TextRenderable, String)>){
        use rusttype::{CodepointOrGlyphId, Codepoint, GlyphId};

        unsafe {
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
            gl::BindVertexArray(self.sprite_vao);

            for textjob in textjobs {

                let(mut transform2d, textrenderable, string) = textjob;
                let shader = self.shaders.get(textrenderable.get_shader_key()).unwrap();

                gl::UseProgram(*shader);

                let transform_loc = gl::GetUniformLocation(*shader, CString::new("projection").unwrap().as_ptr());
                gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, self.projection_2d.as_ptr());

                let color_loc = gl::GetUniformLocation(*shader, CString::new("color").unwrap().as_ptr());
                let color = Vector4::from_value(1.0);
                gl::Uniform4f(color_loc, color.x, color.y, color.z, color.w);

                let mut adv = 0.0;
                for c in string.chars() {
                    //grab the glyph info first
                    let gid = match c.into() {
                        CodepointOrGlyphId::Codepoint(Codepoint(c)) => c,
                        CodepointOrGlyphId::GlyphId(GlyphId(gid)) => gid
                    };

                    let key = OpenGLRenderer::get_glyph_texture_key(textrenderable.get_font_key(), textrenderable.get_font_size(), gid);
                    let glyph_data = self.glyphs.get(&key).unwrap();

                    if let &Some(ref texture_key) = glyph_data.get_texture_key() {
                        let model_loc = gl::GetUniformLocation(*shader, CString::new("model").unwrap().as_ptr());
                        let positioned = Transform2D::new(
                    Vector2::new(
                                transform2d.position.x + adv + glyph_data.get_bearing().x,
                                transform2d.position.y - glyph_data.get_size().y),
                        Vector2::new(
                                glyph_data.get_size().x,
                                glyph_data.get_size().y
                            ),
                            0.0
                        );

                        gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, positioned.get_matrix().as_ptr());

                        //grab the texture
                        let texture = self.textures.get(texture_key.as_str()).unwrap();
                        //let texture = self.textures.get("ball").unwrap();
                        gl::ActiveTexture(gl::TEXTURE0);
                        gl::BindTexture(gl::TEXTURE_2D, *texture);

                        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
                    }

                    adv += glyph_data.get_advance();
                }
            }

            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)
        }
    }

    pub fn render(&mut self, renderjobs : Vec<RenderJob>, postproces_shader_uniforms : &CustomShaderUniform){
        use graphics::job::RenderJob::*;

        if self.preprocess_shader_key.is_some() {
            unsafe {
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                gl::ClearColor(0.2, 0.2, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                gl::BindFramebuffer(gl::FRAMEBUFFER, self.ms_fbo);
                gl::ClearColor(0.2, 0.2, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
        }

        let mut sprites = vec!();
        let mut particles = vec!();
        let mut texts = vec!();

        for job in renderjobs {
            match job {
                Sprite(transform2d, sprite) => sprites.push((transform2d, sprite)),
                Particle(transform2d, particle, sprite) => particles.push((transform2d, particle, sprite)),
                Text(transform2d, text, string) => texts.push((transform2d, text, string))
            };
        }
        
        //self.render_sprite(sprites);
        //self.render_particles(particles);
        self.render_text(texts);

        if let Some(ref postprocess_shader_key) = self.preprocess_shader_key {
            unsafe {
                gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.ms_fbo);
                gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.main_fbo);
                gl::BlitFramebuffer(0, 0,
                                    self.render_width as i32, self.render_height as i32,
                                    0, 0,
                                    self.render_width as i32, self.render_height as i32,
                                    gl::COLOR_BUFFER_BIT, gl::NEAREST);

                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

                let postprocess_shader = self.shaders.get(postprocess_shader_key).unwrap();
                gl::UseProgram(*postprocess_shader);
                self.pass_uniforms(*postprocess_shader, postproces_shader_uniforms);

                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, self.render_texture);

                gl::BindVertexArray(self.sprite_vao);
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            }
        }
    }

    unsafe fn create_sprite_buffer() -> (GLuint, GLuint, GLuint) {
        let vertices: [f32; 20] = [
            // positions (3)   // texture coords (2)
            0.0, 1.0, 0.0, 0.0, 1.0,
            1.0, 0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            1.0, 1.0, 0.0, 1.0, 1.0
        ];

        let indices: [i32; 6] = [
            0, 1, 2,
            0, 3, 1
        ];

        let (mut sprite_vbo, mut sprite_vao, mut sprite_ebo) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut sprite_vao);
        gl::GenBuffers(1, &mut sprite_vbo);
        gl::GenBuffers(1, &mut sprite_ebo);

        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(sprite_vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, sprite_vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, sprite_ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (indices.len() * mem::size_of::<GLint>()) as GLsizeiptr,
                       &indices[0] as *const i32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 5 * mem::size_of::<GLfloat>() as GLsizei,
                                (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);

        // note that this is allowed, the call to gl::VertexAttribPointer registered sprite_vbo as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the sprite_vao afterwards so other sprite_vao calls won't accidentally modify this sprite_vao, but this rarely happens. Modifying other
        // sprite_vaos requires a call to glBindVertexArray anyways so we generally don't unbind sprite_vaos (nor sprite_vbos) when it's not directly necessary.
        gl::BindVertexArray(0);

        (sprite_vao, sprite_vbo, sprite_ebo)
    }

    unsafe fn initialize_postprocessing(render_width : GLuint, render_height : GLuint) -> (GLuint, GLuint, GLuint) {

        let mut fbo = 0;
        let mut msfbo = 0;
        let mut rbo = 0;

        gl::GenFramebuffers(1, &mut msfbo);
        gl::GenFramebuffers(1, &mut fbo);
        gl::GenRenderbuffers(1, &mut rbo);

        //Create a multisampled framebuffer with an attached renderbuffer
        gl::BindFramebuffer(gl::FRAMEBUFFER, msfbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorageMultisample(gl::RENDERBUFFER, 8, gl::RGB, render_width as i32, render_height as i32);
        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::RENDERBUFFER, rbo);

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR:POSTPROCESSOR: Failed to initialize MSFBO");
        }

        //Create texture to be attached to COLOR_ATTACHMENT_0
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        let mut texture = 0;
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, render_width as i32, render_height as i32, 0,
                       gl::RGB, gl::UNSIGNED_BYTE, ptr::null());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture, 0);

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR:POSTPROCESSOR: Failed to create a color FBO");
        }
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);


        (texture, msfbo, fbo)
    }
}
