use cgmath::Vector2;
use std::collections::HashMap;
use rusttype::{self, FontCollection, Font, Scale, point, PositionedGlyph};
use rusttype::{GlyphId, Codepoint, CodepointOrGlyphId};

type FontSizeId = String;

pub struct FontManager <'font> {
    pub font_data : HashMap<FontSizeId, Font<'font>>
}

impl <'font> FontManager <'font> {
    pub fn new() -> FontManager <'font> {
        FontManager {
            font_data : HashMap::new()
        }
    }

    pub fn register_font(&mut self, font_key : &str, font_bytes : Vec<u8>) {
        let font = FontCollection::from_bytes(font_bytes).into_font().unwrap();
        self.font_data.insert(String::from(font_key), font);
    }

    pub fn get_font(&mut self, font_key : &str) -> Option<&Font> {
        self.font_data.get(font_key)
    }
}

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