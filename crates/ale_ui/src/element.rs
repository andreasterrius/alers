use ale_data::alevec::AleVec;
use crate::text::Text;

pub enum Element {
    Elements(Elements),
    Text(Text)
}

pub struct Elements (pub AleVec<Element>);