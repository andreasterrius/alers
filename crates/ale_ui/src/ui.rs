use ale_data::alevec::AleVec;
use crate::element::{Element, Elements};

pub struct Root {
    elements : Elements
}

impl Root {
    pub fn new() -> Root {
        Root { elements: Elements(AleVec::new())}
    }

    pub fn add_element(&mut self, element : Element) {
        self.elements.0.push(element);
    }
}
