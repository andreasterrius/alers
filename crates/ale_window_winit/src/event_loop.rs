use winit::event::Event;
use winit::event_loop::EventLoopWindowTarget;

pub struct EventLoop(winit::event_loop::EventLoop<()>);

impl EventLoop {
  pub fn new() -> EventLoop {
    EventLoop(winit::event_loop::EventLoop::new())
  }

  pub fn run<Fn: FnMut(&EventLoopWindowTarget<()>)+ 'static>(self, mut tick: Fn) {
    self.0.run(move |event, window_target, control_flow| match event {
      Event::NewEvents(_) => {}
      Event::WindowEvent { .. } => {}
      Event::DeviceEvent { .. } => {}
      Event::UserEvent(_) => {}
      Event::Suspended => {}
      Event::Resumed => {}
      Event::MainEventsCleared => {
        tick(window_target);
      }
      Event::RedrawRequested(_) => {}
      Event::RedrawEventsCleared => {}
      Event::LoopDestroyed => {}
    });
  }
}