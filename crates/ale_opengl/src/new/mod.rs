use std::collections::HashMap;
use ale_font::FontTextureKey;
use crate::mesh::OpenGLMesh;
use crate::shader::OpenGLShader;
use crate::texture::OpenGLTexture;

pub mod text;

pub struct TextRenderer {
    text_2d_shader: OpenGLShader,
    plane_opengl_mesh: OpenGLMesh,
    glyph_texture: HashMap<FontTextureKey, OpenGLTexture>,
}

// impl TextRenderer {
//     pub fn new() -> TextRenderer {
//         let text_2d_shader = ale_opengl_shader_new(&ale_shader_new(
//             include_str!("../../../resources/shaders/text_2d.vert").to_owned(),
//             include_str!("../../../resources/shaders/text_2d.frag").to_owned(),
//         ))
//             .unwrap();
//
//         let plane_opengl_mesh = ale_opengl_mesh_new(&ale_mesh_plane_new()).unwrap();
//
//         OpenGLTextFontContext {
//             text_2d_shader,
//             plane_opengl_mesh,
//             glyph_texture: Default::default(),
//         }
//     }
// }