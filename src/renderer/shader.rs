use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct CustomShaderUniform {
    pub uniforms : HashMap<String, ShaderUniform>
}

#[derive(Debug, Clone)]
pub enum ShaderUniform {
    Float1vArray(Vec<f32>),
    Float2vArray(Vec<(f32, f32)>),
    Integer1vArray(Vec<i32>),
    Boolean(bool),
    Double(f64),
    Float(f32)
}
