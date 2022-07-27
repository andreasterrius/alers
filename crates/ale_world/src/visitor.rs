use crate::components;

pub trait Visitor<T : ?Sized> {
    fn visit(&mut self, component : &T);
}

pub trait VisitorMut<T: ?Sized>{
    fn visit(&mut self, component : &mut T);
}