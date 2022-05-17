use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

use ale_input::Input;
use ale_math::num_traits::clamp;
use ale_variable::Variable;

use crate::ConsoleEvent::{Print, Set};
use crate::OnEnterResult::{ArgumentsFailToParse, ArgumentsNumberDiffer, CommandEmpty, CommandNotFound, EventQueued};

pub struct Console {
  // All lines that will be rendered
  pub lines: VecDeque<String>,

  // All commands from user, and capability to look it up
  pub(crate) commands: VecDeque<String>,
  pub(crate) lookup: usize,

  pub line_buffer: String,
  pub(crate) max_lines: usize,

  pub(crate) registered_commands: HashMap<String, Variable>,
  pub(crate) events: HashMap<String, ConsoleEvent>,

  pub has_focus: bool,
}

pub enum ConsoleEvent {
  Print,
  Set(Variable),
  None,
}

enum OnEnterResult {
  CommandEmpty,
  CommandNotFound(String /* cmd */),
  ArgumentsNumberDiffer(String /* cmd */, /* actual */ i32, /* expected */ i32),
  ArgumentsFailToParse(String /* cmd */, String /* arg */),
  EventQueued,
}

pub fn ale_console_new(max_lines: usize) -> Console {
  Console {
    lines: VecDeque::new(),
    commands: VecDeque::new(),
    line_buffer: "".to_string(),
    max_lines,
    has_focus: false,
    //variables: HashMap::new(),
    lookup: 0,
    events: HashMap::new(),
    registered_commands: HashMap::new(),
  }
}

pub fn ale_console_input(console: &mut Console, input: &Input) {
  match input {
    Input::Key(key, scancode, action, modifier) => {
      if action == &ale_input::Action::Press {
        if key == &ale_input::Key::Enter {
          match intern_on_enter(console) {
            CommandNotFound(cmd) => ale_console_print_output(console, &format!("{} is an invalid command", cmd)),
            ArgumentsNumberDiffer(cmd, actual, expected) => ale_console_print_output(
              console,
              &format!("{} has {} argument(s), need {}", cmd, actual, expected),
            ),
            ArgumentsFailToParse(cmd, arg) => {
              ale_console_print_output(console, &format!("{} fail to parse {}, type mismatch", cmd, arg))
            }
            CommandEmpty => {}
            EventQueued => {}
          };
        } else if key == &ale_input::Key::GraveAccent {
          console.has_focus = !console.has_focus;
        } else if key == &ale_input::Key::Backspace {
          console.line_buffer.pop();
        }

        if key == &ale_input::Key::Up {
          if !console.commands.is_empty() {
            console.lookup = clamp(console.lookup, 0, console.commands.len() - 1);
            console.line_buffer = console.commands.iter().rev().nth(console.lookup).unwrap().clone();
            console.lookup = clamp(console.lookup + 1, 0, console.commands.len() - 1);
          }
        } else if key == &ale_input::Key::Down {
          if !console.commands.is_empty() && console.lookup > 0 {
            console.lookup = clamp(console.lookup - 1, 0, console.commands.len() - 1);
            console.line_buffer = console.commands.iter().rev().nth(console.lookup).unwrap().clone();
          } else {
            console.line_buffer.clear();
          }
        } else {
          // Any other key
          console.lookup = 0;
        }
      }
    }
    Input::Char(char) => {
      if console.has_focus && char != &'`' {
        console.line_buffer.push(char.clone());
      }
    }
    _ => {}
  }
}

pub fn ale_console_variable_register(console: &mut Console, variable: Variable) {
  console.registered_commands.insert(variable.name_str(), variable);
}

pub fn ale_console_variable_event_handle<T: From<Variable>>(console: &mut Console, variable: Variable) -> T {
  match console
    .events
    .remove(&variable.name_str())
    .unwrap_or(ConsoleEvent::None)
  {
    Print => ale_console_print_output(console, &variable.value_str()),
    Set(x) => return x.into(),
    ConsoleEvent::None => {}
  }

  return variable.into();
}

pub fn ale_console_variable_has_event(console: &Console) -> bool {
  !console.events.is_empty()
}

pub fn ale_console_print_output(console: &mut Console, cmd: &str) {
  console.lines.push_back(format!(">> {}", cmd));
}

fn intern_on_enter(console: &mut Console) -> OnEnterResult {
  // If the console has too many lines already, we pop the earliest one
  if console.lines.len() > console.max_lines {
    console.lines.pop_front();
  }
  if console.commands.len() > console.max_lines {
    console.commands.pop_front();
  }

  // Get the entered line, add it to console buffer and clear line buffer
  let entered_line = console.line_buffer.clone();
  intern_add_command_line(console, &entered_line);
  console.line_buffer.clear();

  // Process the arguments
  let words: Vec<&str> = entered_line.split_whitespace().collect();
  if words.is_empty() {
    return CommandEmpty;
  }

  let variable = console.registered_commands.get_mut(&words[0].to_owned());

  match variable {
    None => return CommandNotFound(entered_line.to_owned()),
    Some(variable) => {
      if words.len() == 1 {
        console.events.insert(words[0].to_owned(), Print);
        return OnEnterResult::EventQueued;
      }

      match variable {
        Variable::F32_1(key, v) => {
          let val: Vec<f32> = match intern_parse_words_len(&words, 1) {
            Ok(v) => v,
            Err(e) => return e,
          };
          *v = val[0];
        }
        Variable::F32_3(key, ref mut v) => {
          let val: Vec<f32> = match intern_parse_words_len(&words, 3) {
            Ok(v) => v,
            Err(e) => return e,
          };
          v.x = val[0];
          v.y = val[1];
          v.z = val[2];
        }
        Variable::F32_4(key, ref mut v) => {
          let val: Vec<f32> = match intern_parse_words_len(&words, 4) {
            Ok(v) => v,
            Err(e) => return e,
          };
          v.x = val[0];
          v.y = val[1];
          v.z = val[2];
          v.w = val[3];
        }
        Variable::Bool(key, ref mut v) => {
          let val: Vec<bool> = match intern_parse_words_len(&words, 1) {
            Ok(v) => v,
            Err(e) => return e,
          };
          *v = val[0];
        }
        Variable::Void(_) => {}
        Variable::F32_4_4(_, _) => {}
      }
      console.events.insert(words[0].to_owned(), Set(variable.clone()))
    }
  };

  OnEnterResult::EventQueued
}

fn intern_parse_words_len<T: FromStr>(words: &[&str], len: usize) -> Result<Vec<T>, OnEnterResult> {
  if words.len() - 1 != len {
    return Err(OnEnterResult::ArgumentsNumberDiffer(
      words[0].to_owned(),
      (words.len() - 1) as i32,
      len as i32,
    ));
  }

  let mut v: Vec<T> = vec![];
  for i in 1..words.len() {
    let p = words[i].parse::<T>();
    match p {
      Ok(val) => v.push(val),
      Err(err) => return Err(ArgumentsFailToParse(words[0].to_owned(), words[i].to_owned())),
    }
  }
  return Ok(v);
}

fn intern_add_command_line(console: &mut Console, cmd: &str) {
  console.lines.push_back(cmd.to_owned());
  if !cmd.is_empty() {
    console.commands.push_back(cmd.to_owned());
  }
}
