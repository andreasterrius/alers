use ale_input::Input;
use ale_input::Input::Key;
use std::collections::VecDeque;

pub struct Console {
  pub lines: VecDeque<String>,
  pub line_buffer: String,
  pub max_lines: usize,

  pub has_focus: bool,
}

pub fn ale_console_new(max_lines: usize) -> Console {
  Console {
    lines: VecDeque::new(),
    line_buffer: "".to_string(),
    max_lines,
    has_focus: false,
  }
}

pub fn ale_console_input(console: &mut Console, input: &Input) {
  match input {
    Input::Key(key, scancode, action, modifier) => {
      if action == &ale_input::Action::Press {
        if key == &ale_input::Key::Enter {
          console.lines.push_back(console.line_buffer.clone());
          console.line_buffer.clear();
        } else if key == &ale_input::Key::GraveAccent {
          console.has_focus = !console.has_focus;
        } else if key == &ale_input::Key::Backspace {
          console.line_buffer.pop();
        }
      }
    }
    Input::Char(char) => {
      if console.has_focus {
        console.line_buffer.push(char.clone());
      }
    }
    _ => {}
  }
}
