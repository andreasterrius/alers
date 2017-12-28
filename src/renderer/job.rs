use math::{Transform2D, Lerpable};
use cgmath::{Vector3, Vector4};

pub enum RenderJob {
    Sprite(Transform2D, SpriteRenderable),
    Particle(Transform2D, ParticleRenderable, SpriteRenderable)
}

#[derive(Clone, Debug)]
pub struct SpriteRenderable {
    pub color : Vector4<f32>,
    pub custom_shader_key: String,
    pub texture_keys : Vec<String>
}

impl SpriteRenderable {

    pub fn new (color : Vector4<f32>,
                shader_key : String,
                texture_keys : Vec<String>) -> SpriteRenderable {
        SpriteRenderable {
            color,
            custom_shader_key: shader_key,
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
}

#[derive(Clone,Debug)]
pub struct ParticleRenderable {
    pub life : f32
}