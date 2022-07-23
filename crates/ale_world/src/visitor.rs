use crate::components;

pub trait RenderableVisitor {
    fn visit(&mut self, renderable : &mut dyn components::Renderable);
}

pub trait CameraVisitor {
    fn visit(&mut self, camera : &mut dyn components::Camera);
}

pub trait Visitor<'a, T : ?Sized> {
    fn visit(&mut self, component : &T);
}

pub trait VisitorMut<'a, T: ?Sized + 'a> {
    fn visit(&mut self, component : &mut T);
}