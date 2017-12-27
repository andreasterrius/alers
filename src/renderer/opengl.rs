use gl;
use gl::types::*;
use std::ffi::CString;
use std::{ptr, mem};
use std::os::raw::c_void;
use std::collections::HashMap;
use image::{self, DynamicImage};
use math::Transform2D;
use cgmath::prelude::*;
use cgmath::{self, Vector2, Vector3, Vector4, Matrix4};
use renderer::job::{RenderJob, SpriteRenderable};
use std::str;

#[allow(non_snake_case)]
pub struct OpenGLRenderer {
    pub sprite_vao : GLuint,
    pub sprite_vbo : GLuint,
    pub sprite_ebo : GLuint,

    pub projection_2d : Matrix4<f32>,

    pub shaders : HashMap<String, GLuint>,
    pub textures : HashMap<String, GLuint>
}

impl OpenGLRenderer {
    pub fn new(screen_width : u32, screen_height : u32) -> OpenGLRenderer {
        unsafe {
            let (sprite_vao, sprite_vbo, sprite_ebo) = OpenGLRenderer::create_sprite_buffer();

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            let projection_2d = cgmath::ortho(0.0, screen_width as f32,screen_height as f32, 0.0, -0.1, 0.1);

            OpenGLRenderer {
                sprite_vao,
                sprite_vbo,
                sprite_ebo,

                projection_2d,
                shaders : HashMap::new(),
                textures : HashMap::new(),
            }
        }
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

    pub fn clear(&mut self){
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
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

            for sprite in sprites {

                let(transform2d, renderable_2d) = sprite;
                let shader = self.shaders.get(renderable_2d.get_shader_key()).unwrap();

                gl::UseProgram(*shader);

                let textures_keys = renderable_2d.get_texture_keys();
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
                let color = renderable_2d.get_sprite_color();
                gl::Uniform4f(color_loc, color.x, color.y, color.z, color.w);

                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            }
        }
    }

    pub fn render_particles(&mut self, particlejobs : Vec<(Transform2D, SpriteRenderable)>){
        unsafe {
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
            gl::BindVertexArray(self.sprite_vao);

            for particlejob in particlejobs {

                let(transform2d, particle) = particlejob;
                let shader = self.shaders.get(particle.get_shader_key()).unwrap();

                gl::UseProgram(*shader);

                let textures_keys = particle.get_texture_keys();
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
                let color = particle.get_sprite_color();
                gl::Uniform4f(color_loc, color.x, color.y, color.z, color.w);

                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            }

            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)
        }
    }

    pub fn render(&mut self, renderjobs : Vec<RenderJob>){

        use renderer::job::RenderJob::*;

        let mut sprites = vec!();
        let mut particles = vec!();

        for job in renderjobs {
            match job {
                Sprite(transform2d, sprite) => sprites.push((transform2d, sprite)),
                Particle(transform2d, particle) => particles.push((transform2d, particle))
            };
        }
        
        self.render_sprite(sprites);
        self.render_particles(particles);
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
}
