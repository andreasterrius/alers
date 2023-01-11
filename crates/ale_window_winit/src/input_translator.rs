// use glfw::{Action, Key, MouseButton};
//
// pub fn translate_key(glfw_key: glfw::Key) -> ale_input::Key {
//   match glfw_key {
//     Key::Space => ale_input::Key::Space,
//     Key::Apostrophe => ale_input::Key::Apostrophe,
//     Key::Comma => ale_input::Key::Comma,
//     Key::Minus => ale_input::Key::Minus,
//     Key::Period => ale_input::Key::Period,
//     Key::Slash => ale_input::Key::Slash,
//     Key::Num0 => ale_input::Key::Num0,
//     Key::Num1 => ale_input::Key::Num1,
//     Key::Num2 => ale_input::Key::Num2,
//     Key::Num3 => ale_input::Key::Num3,
//     Key::Num4 => ale_input::Key::Num4,
//     Key::Num5 => ale_input::Key::Num5,
//     Key::Num6 => ale_input::Key::Num6,
//     Key::Num7 => ale_input::Key::Num7,
//     Key::Num8 => ale_input::Key::Num8,
//     Key::Num9 => ale_input::Key::Num9,
//     Key::Semicolon => ale_input::Key::Semicolon,
//     Key::Equal => ale_input::Key::Equal,
//     Key::A => ale_input::Key::A,
//     Key::B => ale_input::Key::B,
//     Key::C => ale_input::Key::C,
//     Key::D => ale_input::Key::D,
//     Key::E => ale_input::Key::E,
//     Key::F => ale_input::Key::F,
//     Key::G => ale_input::Key::G,
//     Key::H => ale_input::Key::H,
//     Key::I => ale_input::Key::I,
//     Key::J => ale_input::Key::J,
//     Key::K => ale_input::Key::K,
//     Key::L => ale_input::Key::L,
//     Key::M => ale_input::Key::M,
//     Key::N => ale_input::Key::N,
//     Key::O => ale_input::Key::O,
//     Key::P => ale_input::Key::P,
//     Key::Q => ale_input::Key::Q,
//     Key::R => ale_input::Key::R,
//     Key::S => ale_input::Key::S,
//     Key::T => ale_input::Key::T,
//     Key::U => ale_input::Key::U,
//     Key::V => ale_input::Key::V,
//     Key::W => ale_input::Key::W,
//     Key::X => ale_input::Key::X,
//     Key::Y => ale_input::Key::Y,
//     Key::Z => ale_input::Key::Z,
//     Key::LeftBracket => ale_input::Key::LeftBracket,
//     Key::Backslash => ale_input::Key::Backslash,
//     Key::RightBracket => ale_input::Key::RightBracket,
//     Key::GraveAccent => ale_input::Key::GraveAccent,
//     Key::World1 => ale_input::Key::World1,
//     Key::World2 => ale_input::Key::World2,
//     Key::Escape => ale_input::Key::Escape,
//     Key::Enter => ale_input::Key::Enter,
//     Key::Tab => ale_input::Key::Tab,
//     Key::Backspace => ale_input::Key::Backspace,
//     Key::Insert => ale_input::Key::Insert,
//     Key::Delete => ale_input::Key::Delete,
//     Key::Right => ale_input::Key::Right,
//     Key::Left => ale_input::Key::Left,
//     Key::Down => ale_input::Key::Down,
//     Key::Up => ale_input::Key::Up,
//     Key::PageUp => ale_input::Key::PageUp,
//     Key::PageDown => ale_input::Key::PageDown,
//     Key::Home => ale_input::Key::Home,
//     Key::End => ale_input::Key::End,
//     Key::CapsLock => ale_input::Key::CapsLock,
//     Key::ScrollLock => ale_input::Key::ScrollLock,
//     Key::NumLock => ale_input::Key::NumLock,
//     Key::PrintScreen => ale_input::Key::PrintScreen,
//     Key::Pause => ale_input::Key::Pause,
//     Key::F1 => ale_input::Key::F1,
//     Key::F2 => ale_input::Key::F2,
//     Key::F3 => ale_input::Key::F3,
//     Key::F4 => ale_input::Key::F4,
//     Key::F5 => ale_input::Key::F5,
//     Key::F6 => ale_input::Key::F6,
//     Key::F7 => ale_input::Key::F7,
//     Key::F8 => ale_input::Key::F8,
//     Key::F9 => ale_input::Key::F9,
//     Key::F10 => ale_input::Key::F10,
//     Key::F11 => ale_input::Key::F11,
//     Key::F12 => ale_input::Key::F12,
//     Key::F13 => ale_input::Key::F13,
//     Key::F14 => ale_input::Key::F14,
//     Key::F15 => ale_input::Key::F15,
//     Key::F16 => ale_input::Key::F16,
//     Key::F17 => ale_input::Key::F17,
//     Key::F18 => ale_input::Key::F18,
//     Key::F19 => ale_input::Key::F19,
//     Key::F20 => ale_input::Key::F20,
//     Key::F21 => ale_input::Key::F21,
//     Key::F22 => ale_input::Key::F22,
//     Key::F23 => ale_input::Key::F23,
//     Key::F24 => ale_input::Key::F24,
//     Key::F25 => ale_input::Key::F25,
//     Key::Kp0 => ale_input::Key::Kp0,
//     Key::Kp1 => ale_input::Key::Kp1,
//     Key::Kp2 => ale_input::Key::Kp2,
//     Key::Kp3 => ale_input::Key::Kp3,
//     Key::Kp4 => ale_input::Key::Kp4,
//     Key::Kp5 => ale_input::Key::Kp5,
//     Key::Kp6 => ale_input::Key::Kp6,
//     Key::Kp7 => ale_input::Key::Kp7,
//     Key::Kp8 => ale_input::Key::Kp8,
//     Key::Kp9 => ale_input::Key::Kp9,
//     Key::KpDecimal => ale_input::Key::KpDecimal,
//     Key::KpDivide => ale_input::Key::KpDivide,
//     Key::KpMultiply => ale_input::Key::KpMultiply,
//     Key::KpSubtract => ale_input::Key::KpSubtract,
//     Key::KpAdd => ale_input::Key::KpAdd,
//     Key::KpEnter => ale_input::Key::KpEnter,
//     Key::KpEqual => ale_input::Key::KpEqual,
//     Key::LeftShift => ale_input::Key::LeftShift,
//     Key::LeftControl => ale_input::Key::LeftControl,
//     Key::LeftAlt => ale_input::Key::LeftAlt,
//     Key::LeftSuper => ale_input::Key::LeftSuper,
//     Key::RightShift => ale_input::Key::RightShift,
//     Key::RightControl => ale_input::Key::RightControl,
//     Key::RightAlt => ale_input::Key::RightAlt,
//     Key::RightSuper => ale_input::Key::RightSuper,
//     Key::Menu => ale_input::Key::Menu,
//     Key::Unknown => ale_input::Key::Unknown,
//   }
// }
//
// pub fn translate_scancode(glfw_scancode: glfw::Scancode) -> ale_input::Scancode {
//   glfw_scancode
// }
//
// pub fn translate_action(glfw_action: glfw::Action) -> ale_input::Action {
//   match glfw_action {
//     Action::Release => ale_input::Action::Release,
//     Action::Press => ale_input::Action::Press,
//     Action::Repeat => ale_input::Action::Repeat,
//   }
// }
//
// pub fn translate_modifier(glfw_modifier: glfw::Modifiers) -> ale_input::Modifier {
//   let mut modifier = ale_input::Modifier::empty();
//   if glfw_modifier.contains(glfw::Modifiers::Shift) {
//     modifier.insert(ale_input::Modifier::SHIFT);
//   }
//   if glfw_modifier.contains(glfw::Modifiers::Control) {
//     modifier.insert(ale_input::Modifier::CONTROL);
//   }
//   if glfw_modifier.contains(glfw::Modifiers::Alt) {
//     modifier.insert(ale_input::Modifier::ALT);
//   }
//   if glfw_modifier.contains(glfw::Modifiers::Super) {
//     modifier.insert(ale_input::Modifier::SUPER);
//   }
//   if glfw_modifier.contains(glfw::Modifiers::CapsLock) {
//     modifier.insert(ale_input::Modifier::CAPSLOCK);
//   }
//   if glfw_modifier.contains(glfw::Modifiers::NumLock) {
//     modifier.insert(ale_input::Modifier::NUMLOCK);
//   }
//   modifier
// }
//
// pub fn translate_mousebutton(glfw_mouse: glfw::MouseButton) -> ale_input::MouseButton {
//   match glfw_mouse {
//     MouseButton::Button1 => ale_input::MouseButton::ButtonLeft,
//     MouseButton::Button2 => ale_input::MouseButton::ButtonRight,
//     MouseButton::Button3 => ale_input::MouseButton::ButtonMiddle,
//     MouseButton::Button4 => ale_input::MouseButton::Button4,
//     MouseButton::Button5 => ale_input::MouseButton::Button5,
//     MouseButton::Button6 => ale_input::MouseButton::Button6,
//     MouseButton::Button7 => ale_input::MouseButton::Button7,
//     MouseButton::Button8 => ale_input::MouseButton::Button8,
//   }
// }
