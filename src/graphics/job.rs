use math::{Transform2D, Lerpable};
use cgmath::{Vector3, Vector4};
use graphics::shader::CustomShaderUniform;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum RenderJob {
    Sprite(Transform2D, SpriteRenderable),
    Particle(Transform2D, ParticleRenderable, SpriteRenderable),
    Text(Transform2D, TextRenderable, String)
}

#[derive(Clone, Debug)]
pub struct SpriteRenderable {
    pub color : Vector4<f32>,
    pub custom_shader_key: String,
    pub custom_shader_uniform: Option<CustomShaderUniform>,
    pub texture_keys : Vec<String>,
}

impl SpriteRenderable {

    pub fn new (color : Vector4<f32>,
                shader_key : String,
                shader_uniforms : Option<CustomShaderUniform>,
                texture_keys : Vec<String>) -> SpriteRenderable {
        SpriteRenderable {
            color,
            custom_shader_key: shader_key,
            custom_shader_uniform: shader_uniforms,
            texture_keys
        }
    }

    pub fn get_sprite_color(&self) -> &Vector4<f32>{
        &self.color
    }

    pub fn get_shader_key(&self) -> &str {
        &self.custom_shader_key
    }

    pub fn get_texture_keys(&self) -> &Vec<String> {
        &self.texture_keys
    }

    pub fn get_shader_uniforms(&self) -> &Option<CustomShaderUniform> {
        &self.custom_shader_uniform
    }
}

#[derive(Clone,Debug)]
pub struct ParticleRenderable {
    pub life : f32
}

#[derive(Clone,Debug)]
pub struct TextRenderable {
    pub shader_key : String,
    pub font_key : String,
    pub font_size : u32,
}

impl TextRenderable {
    pub fn new(shader_key : String, font_key : String, font_size : u32) -> TextRenderable {
        TextRenderable {
            shader_key,
            font_key,
            font_size
        }
    }

    pub fn get_shader_key(&self) -> &str {
        &self.shader_key
    }

    pub fn get_font_key(&self) -> &str {
        &self.font_key
    }

    pub fn get_font_size(&self) -> u32 {
        self.font_size
    }
}