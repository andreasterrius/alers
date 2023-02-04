use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::hash::{Hash, Hasher};

use futures::executor::block_on;
use log::info;
use raw_window_handle::{
  HasRawDisplayHandle, HasRawWindowHandle,
};
use wgpu::{BufferDescriptor, CommandEncoder, CompositeAlphaMode, PipelineLayout, Surface, TextureFormat, TextureView};
use wgpu::util::BufferInitDescriptor;

use ale_data::alevec::Key;
use ale_data::entity::entry::Traitcast;
use ale_data::indexmap::{AleIndexMap, Id, IndexMap};
use ale_math::rect::Rect;
use ale_resources::resources::Resources;
use ale_resources::shader::{GLSLShader, WGSLShader};
use ale_variable::Variable;

use crate::render_pipeline::{RenderPipeline, RenderPipelineOpts};
use crate::sprite_renderer::SpriteRenderPipeline;

pub struct Graphics {
  instance: wgpu::Instance,

  pub render_device: Option<RenderDevice>,
  pub surfaces: AleIndexMap<wgpu::Surface>,

}

impl Graphics {
  pub fn new() -> Graphics {
    Graphics {
      instance: wgpu::Instance::new(wgpu::Backends::all()),
      render_device: None,
      surfaces: AleIndexMap::new(),
    }
  }

  pub fn create_surface<W>(&mut self, w: &W, dimension: Rect) -> Id<wgpu::Surface>
    where W: HasRawWindowHandle + HasRawDisplayHandle
  {
    let surface = unsafe { self.instance.create_surface(w) };
    let adapter = block_on(async {
      self.instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
      }).await.unwrap()
    });

    match &self.render_device {
      None => {
        let (device, queue) = block_on(async {
          adapter.request_device(
            &wgpu::DeviceDescriptor {
              label: None,
              features: wgpu::Features::empty(),
              limits: wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            }, None).await.unwrap()
        });
        self.render_device = Some(RenderDevice::new(device, adapter, queue));
      }
      Some(rd) => {
        if rd.adapter.get_info() != adapter.get_info() {
          panic!("adapter is not the same for a newly created surface");
        }
      }
    };

    let device = &self.render_device.as_mut().unwrap().device;
    let mut config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: TextureFormat::Rgb10a2Unorm,
      width: dimension.size.x,
      height: dimension.size.y,
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode: CompositeAlphaMode::Auto,
    };
    surface.configure(device, &config);

    self.surfaces.insert(surface)
  }

  pub fn create_render_pipeline(&mut self,
                                shader: &WGSLShader,
                                opts : RenderPipelineOpts) -> RenderPipeline {
    let rd = &mut self.render_device.as_mut().expect("no surface has been created yet");

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

    RenderPipeline::new(render_pipeline)
  }

  pub fn execute<F: FnMut(&TextureView, &mut CommandEncoder)>(&mut self, mut func: F) {
    let rd = self.render_device.as_mut().unwrap();
    for (id, surface) in &self.surfaces.inner {
      let frame = surface.get_current_texture().expect("failed to acquire next swap chain texture");
      let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
      let mut encoder = rd.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

      func(&view, &mut encoder);

      rd.queue.submit(Some(encoder.finish()));
      frame.present();
    }
  }
}

pub struct RenderDevice {
  pub device: wgpu::Device,
  pub adapter: wgpu::Adapter,
  pub queue: wgpu::Queue,
}

impl RenderDevice {
  pub fn new(device: wgpu::Device, adapter: wgpu::Adapter, queue: wgpu::Queue) -> RenderDevice {
    RenderDevice {
      device,
      adapter,
      queue,
    }
  }
}

