pub mod opengl;
use cgmath::{Vector3};

#[derive(Clone)]
pub struct Renderable2D {
    color : Vector3<f32>,
    shader_key : String,
    texture_keys : Vec<String>
}

impl Renderable2D {

    pub fn new (color : Vector3<f32>,
               shader_key : String,
               texture_keys : Vec<String>) -> Renderable2D {
        Renderable2D {
            color,
            shader_key,
            texture_keys
        }
    }

    pub fn get_sprite_color(&self) -> &Vector3<f32>{
        &self.color
    }

    pub fn get_shader_key(&self) -> &str {
        &self.shader_key
    }

    pub fn get_texture_keys(&self) -> &Vec<String> {
        &self.texture_keys
    }
}