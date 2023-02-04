use wgpu::Label;
use ale_variable::Variable;

pub struct RenderPipeline {
  pub render_pipeline : wgpu::RenderPipeline
}

impl RenderPipeline {
  pub fn new(render_pipeline : wgpu::RenderPipeline) -> RenderPipeline {
    RenderPipeline {
      render_pipeline
    }
  }
}

pub struct RenderPipelineOpts<'a> {
  unique_label : String,

  pub vertex_buffer_descriptor : Option<wgpu::BufferDescriptor<'a>>,
  pub index_buffer_descriptor : Option<wgpu::BufferDescriptor<'a>>,


  //uniform_layouts : Vec<Variable>,
}

impl <'a> RenderPipelineOpts<'a> {
  pub fn label(&self, name : &str) -> String {
    format!("{} {}", self.unique_label, name)
  }
}

pub struct RenderLabel(pub String);
impl <'a> RenderLabel {
  pub fn name(&'a self, n : &str) -> Option<&'a str> {
      Some(&format!("{} {}", self.0, n))
  }
}