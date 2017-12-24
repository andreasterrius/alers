use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;
use std::fmt;
use image;

pub struct ResourceManager {
    images : HashMap<String, ResourceImageFile>,
    glsl : HashMap<String, ResourceGlslFile>
}

impl ResourceManager {

    pub fn new() -> ResourceManager {
        ResourceManager{
            images: HashMap::new(),
            glsl: HashMap::new()
        }
    }

    pub fn load_image(&mut self, key : &str, path : &str){

        let img_path = PathBuf::from(path);
        let img = image::open(path)
            .expect(&format!("Image not found, {:?}", path));

        self.images.insert(String::from(key), ResourceImageFile{
            image : img
        });
    }

    pub fn get_image(&self, key: &str) -> Option<&ResourceImageFile> {
        self.images.get(key)
    }

    pub fn load_glsl(&mut self,
                 key: &str,
                 vertex_shader_path: &str,
                 fragment_shader_path: &str)
    {
        let vertex_shader_pathbuf = PathBuf::from(vertex_shader_path);
        let mut vertex_shader = String::new();
        File::open(vertex_shader_pathbuf)
            .expect(&format!("Vertex shader file not found, {:?}", vertex_shader_path))
            .read_to_string(&mut vertex_shader);

        let fragment_shader_pathbuf = PathBuf::from(fragment_shader_path);
        let mut fragment_shader = String::new();
        File::open(fragment_shader_pathbuf)
            .expect(&format!("Fragment shader file not found. {:?}", fragment_shader_path))
            .read_to_string(&mut fragment_shader);

        self.glsl.insert(String::from(key), ResourceGlslFile{
            vertex_shader,
            fragment_shader,
        });
    }

    pub fn get_glsl(&self, key : &str) -> Option<&ResourceGlslFile> {
        self.glsl.get(key)
    }
}

pub struct ResourceGlslFile {
    pub vertex_shader : String,
    pub fragment_shader : String
}

pub struct ResourceImageFile {
    pub image : image::DynamicImage
}