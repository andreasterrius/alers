use crate::ui::button::Button;
use crate::ui::panel::Panel;
use crate::ui::text::Text;

pub mod button;
pub mod panel;
pub mod text;

pub enum UI {
  Button(Button),
  Panel(Panel),
  Text(Text),
}