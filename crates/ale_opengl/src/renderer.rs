use ale_render::renderer;
use crate::render_frame::ale_opengl_render_frame_capture;
use crate::text::{ale_opengl_text_render, OpenGLTextFontContext};

pub struct Renderer {
    opengl_text_context : OpenGLTextFontContext
}

impl renderer::Renderer for Renderer {
    fn render_text() {
    }

    fn render_pbr_mesh() {
        todo!()
    }

    fn render_wire_mesh() {
        todo!()
    }

    fn render_textured_plane() {
        todo!()
    }

    fn render_debug_line3d() {
        todo!()
    }

    fn render_debug_point3d() {
        todo!()
    }
}