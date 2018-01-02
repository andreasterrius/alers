use math::{Transform2D, Lerpable};
use cgmath::{Vector3, Vector4};
use renderer::shader::CustomShaderUniform;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum RenderJob {
    Sprite(Transform2D, SpriteRenderable),
    Particle(Transform2D, ParticleRenderable, SpriteRenderable),
    Text(Transform2D, TextRenderable)
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

pub struct TextRenderable {
    pub font_key : String,
    pub font_size : String,
}