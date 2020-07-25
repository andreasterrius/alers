use glfw::{Action, Key};

use crate::input;

pub fn translate_key(glfw_key: glfw::Key) -> input::Key {
  match glfw_key {
    Key::Space => input::Key::Space,
    Key::Apostrophe => input::Key::Apostrophe,
    Key::Comma => input::Key::Comma,
    Key::Minus => input::Key::Minus,
    Key::Period => input::Key::Period,
    Key::Slash => input::Key::Slash,
    Key::Num0 => input::Key::Num0,
    Key::Num1 => input::Key::Num1,
    Key::Num2 => input::Key::Num2,
    Key::Num3 => input::Key::Num3,
    Key::Num4 => input::Key::Num4,
    Key::Num5 => input::Key::Num5,
    Key::Num6 => input::Key::Num6,
    Key::Num7 => input::Key::Num7,
    Key::Num8 => input::Key::Num8,
    Key::Num9 => input::Key::Num9,
    Key::Semicolon => input::Key::Semicolon,
    Key::Equal => input::Key::Equal,
    Key::A => input::Key::A,
    Key::B => input::Key::B,
    Key::C => input::Key::C,
    Key::D => input::Key::D,
    Key::E => input::Key::E,
    Key::F => input::Key::F,
    Key::G => input::Key::G,
    Key::H => input::Key::H,
    Key::I => input::Key::I,
    Key::J => input::Key::J,
    Key::K => input::Key::K,
    Key::L => input::Key::L,
    Key::M => input::Key::M,
    Key::N => input::Key::N,
    Key::O => input::Key::O,
    Key::P => input::Key::P,
    Key::Q => input::Key::Q,
    Key::R => input::Key::R,
    Key::S => input::Key::S,
    Key::T => input::Key::T,
    Key::U => input::Key::U,
    Key::V => input::Key::V,
    Key::W => input::Key::W,
    Key::X => input::Key::X,
    Key::Y => input::Key::Y,
    Key::Z => input::Key::Z,
    Key::LeftBracket => input::Key::LeftBracket,
    Key::Backslash => input::Key::Backslash,
    Key::RightBracket => input::Key::RightBracket,
    Key::GraveAccent => input::Key::GraveAccent,
    Key::World1 => input::Key::World1,
    Key::World2 => input::Key::World2,
    Key::Escape => input::Key::Escape,
    Key::Enter => input::Key::Enter,
    Key::Tab => input::Key::Tab,
    Key::Backspace => input::Key::Backspace,
    Key::Insert => input::Key::Insert,
    Key::Delete => input::Key::Delete,
    Key::Right => input::Key::Right,
    Key::Left => input::Key::Left,
    Key::Down => input::Key::Down,
    Key::Up => input::Key::Up,
    Key::PageUp => input::Key::PageUp,
    Key::PageDown => input::Key::PageDown,
    Key::Home => input::Key::Home,
    Key::End => input::Key::End,
    Key::CapsLock => input::Key::CapsLock,
    Key::ScrollLock => input::Key::ScrollLock,
    Key::NumLock => input::Key::NumLock,
    Key::PrintScreen => input::Key::PrintScreen,
    Key::Pause => input::Key::Pause,
    Key::F1 => input::Key::F1,
    Key::F2 => input::Key::F2,
    Key::F3 => input::Key::F3,
    Key::F4 => input::Key::F4,
    Key::F5 => input::Key::F5,
    Key::F6 => input::Key::F6,
    Key::F7 => input::Key::F7,
    Key::F8 => input::Key::F8,
    Key::F9 => input::Key::F9,
    Key::F10 => input::Key::F10,
    Key::F11 => input::Key::F11,
    Key::F12 => input::Key::F12,
    Key::F13 => input::Key::F13,
    Key::F14 => input::Key::F14,
    Key::F15 => input::Key::F15,
    Key::F16 => input::Key::F16,
    Key::F17 => input::Key::F17,
    Key::F18 => input::Key::F18,
    Key::F19 => input::Key::F19,
    Key::F20 => input::Key::F20,
    Key::F21 => input::Key::F21,
    Key::F22 => input::Key::F22,
    Key::F23 => input::Key::F23,
    Key::F24 => input::Key::F24,
    Key::F25 => input::Key::F25,
    Key::Kp0 => input::Key::Kp0,
    Key::Kp1 => input::Key::Kp1,
    Key::Kp2 => input::Key::Kp2,
    Key::Kp3 => input::Key::Kp3,
    Key::Kp4 => input::Key::Kp4,
    Key::Kp5 => input::Key::Kp5,
    Key::Kp6 => input::Key::Kp6,
    Key::Kp7 => input::Key::Kp7,
    Key::Kp8 => input::Key::Kp8,
    Key::Kp9 => input::Key::Kp9,
    Key::KpDecimal => input::Key::KpDecimal,
    Key::KpDivide => input::Key::KpDivide,
    Key::KpMultiply => input::Key::KpMultiply,
    Key::KpSubtract => input::Key::KpSubtract,
    Key::KpAdd => input::Key::KpAdd,
    Key::KpEnter => input::Key::KpEnter,
    Key::KpEqual => input::Key::KpEqual,
    Key::LeftShift => input::Key::LeftShift,
    Key::LeftControl => input::Key::LeftControl,
    Key::LeftAlt => input::Key::LeftAlt,
    Key::LeftSuper => input::Key::LeftSuper,
    Key::RightShift => input::Key::RightShift,
    Key::RightControl => input::Key::RightControl,
    Key::RightAlt => input::Key::RightAlt,
    Key::RightSuper => input::Key::RightSuper,
    Key::Menu => input::Key::Menu,
    Key::Unknown => input::Key::Unknown,
  }
}

pub fn translate_scancode(glfw_scancode: glfw::Scancode) -> input::Scancode {
  glfw_scancode
}

pub fn translate_action(glfw_action: glfw::Action) -> input::Action {
  match glfw_action {
    Action::Release => input::Action::Release,
    Action::Press => input::Action::Press,
    Action::Repeat => input::Action::Repeat,
  }
}

pub fn translate_modifier(glfw_modifier: glfw::Modifiers) -> input::Modifier {
  let mut modifier = input::Modifier::empty();
  if glfw_modifier.contains(glfw::Modifiers::Shift) {
    modifier.insert(input::Modifier::SHIFT);
  }
  if glfw_modifier.contains(glfw::Modifiers::Control) {
    modifier.insert(input::Modifier::CONTROL);
  }
  if glfw_modifier.contains(glfw::Modifiers::Alt) {
    modifier.insert(input::Modifier::ALT);
  }
  if glfw_modifier.contains(glfw::Modifiers::Super) {
    modifier.insert(input::Modifier::SUPER);
  }
  if glfw_modifier.contains(glfw::Modifiers::CapsLock) {
    modifier.insert(input::Modifier::CAPSLOCK);
  }
  if glfw_modifier.contains(glfw::Modifiers::NumLock) {
    modifier.insert(input::Modifier::NUMLOCK);
  }
  modifier
}
