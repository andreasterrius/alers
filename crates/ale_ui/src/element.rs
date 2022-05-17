use ale_data::alevec::AleVec;
use crate::button::Button;
use crate::text::Text;

pub enum Element {
    Elements(Elements),
    Button(Button),
    Text(Text),
}

pub struct Elements(pub AleVec<Element>);