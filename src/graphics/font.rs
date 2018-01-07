extern crate rusttype;
use cgmath::Vector2;

pub struct GlyphData {
    id : u32,
    texture_key : String,
    size : Vector2<f32>,
    bearing : Vector2<f32>,
    advance : f32
}

impl GlyphData {
    pub fn new(id : u32,
               texture_key: String,
               size : Vector2<f32>,
               bearing : Vector2<f32>,
               advance : f32) -> GlyphData {
        GlyphData {
            id,
            texture_key,
            size,
            bearing,
            advance,
        }
    }

    pub fn get_texture_key(&self) -> &str {
        &self.texture_key
    }

    pub fn get_size(&self) -> &Vector2<f32> {
        &self.size
    }

    pub fn get_bearing(&self) -> &Vector2<f32> {
        &self.bearing
    }

    pub fn get_advance(&self) -> f32 {
        self.advance
    }
}