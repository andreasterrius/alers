use crate::components;

pub trait RenderableVisitor {
    fn visit(&mut self, renderable : &mut dyn components::Renderable);
}

pub trait CameraVisitor {
    fn visit(&mut self, camera : &mut dyn components::Camera);
}

