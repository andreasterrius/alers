use std::borrow::Cow;
use ale_resources::resources::Resources;
use ale_resources::shader::{LoadError, WGSLShader};
use thiserror::Error;
use wgpu::{BufferDescriptor, TextureFormat};
use ale_math::{Vector3, Vector4};

use crate::backend::{Graphics};
use crate::render_pipeline::{RenderLabel, RenderPipeline, RenderPipelineOpts};

const MAX_SPRITE_SUPPORTED : u64 = 30000;

pub struct Flat {
  position : [Vector3<f32>; 4],
  color : Vector4<f32>
}

pub struct SpriteRenderPipeline {
  render_pipeline: wgpu::RenderPipeline,
}

impl SpriteRenderPipeline {
  pub fn new(
    graphics: &mut Graphics,
    resources: &mut Resources,
  ) -> Result<SpriteRenderPipeline, SpriteRendererError> {
    let shader_key = resources.shaders.wgsl_stash.load("shaders/2d")?.remove(0);
    let shader = resources.shaders.wgsl_stash.get(shader_key).unwrap();

    let label = RenderLabel("sprite renderer".to_owned());
    let rd = graphics.render_device.as_mut().expect("no surface has been created yet");

    rd.device.create_buffer(&BufferDescriptor{
      label: label.name("vertex buffer"),
      size: MAX_SPRITE_SUPPORTED * (std::mem::size_of::<Flat>() as u64),
      usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false
    });

    let pipeline_layout = rd.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: None,
      bind_group_layouts: &[],
      push_constant_ranges: &[],
    });

    let shader_module = rd.device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: None,
      source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&shader.source)),
    });

    let render_pipeline = rd.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: None,
      layout: Some(&pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader_module,
        entry_point: &shader.vert_entry,
        buffers: &[],
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader_module,
        entry_point: &shader.frag_entry,
        targets: &[Some(wgpu::ColorTargetState {
          format: TextureFormat::Rgb10a2Unorm,
          blend: Some(wgpu::BlendState::ALPHA_BLENDING),
          write_mask: wgpu::ColorWrites::default(),
        })],
      }),
      primitive: wgpu::PrimitiveState::default(),
      depth_stencil: None,
      multisample: wgpu::MultisampleState::default(),
      multiview: None,
    });


    Ok(SpriteRenderPipeline { render_pipeline })
  }

  pub fn render<'a>(&'a self,
                view : &'a wgpu::TextureView,
                encoder : &'a mut wgpu::CommandEncoder) -> wgpu::RenderPass {
    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: None,
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view,
        resolve_target: None,
        ops: wgpu::Operations {
          load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
          store: true,
        },
      })],
      depth_stencil_attachment: None,
    });
    rpass.set_pipeline(&self.render_pipeline);
    rpass.draw(0..3, 0..1);
    rpass
  }
}

#[derive(Error, Debug)]
pub enum SpriteRendererError {
  #[error("(SpriteRendererError::ShaderNotFound) {}", .0)]
  ShaderNotFound(#[from] LoadError),
}
