use crate::OnEnterResult::{
  ArgumentPrint, ArgumentSet, ArgumentsFailToParse, ArgumentsNumberDiffer, CommandEmpty, CommandNotFound,
};
use ale_input::Input;
use ale_input::Input::Key;
use ale_math::num_traits::clamp;
use ale_math::{Vector3, Vector4};
use ale_variable::{ale_variable_name_get, ale_variable_value_to_string, Variable};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::process::Command;
use std::str::FromStr;
use std::sync::mpsc::TryRecvError::Empty;

pub struct Console {
  // All lines that will be rendered
  pub lines: VecDeque<String>,

  // All commands from user
  pub commands: VecDeque<String>,

  pub line_buffer: String,
  pub max_lines: usize,

  pub variables: HashMap<String, Variable>,
  pub lookup: usize,

  pub has_focus: bool,
}

pub fn ale_console_new(max_lines: usize) -> Console {
  Console {
    lines: VecDeque::new(),
    commands: VecDeque::new(),
    line_buffer: "".to_string(),
    max_lines,
    has_focus: false,
    variables: HashMap::new(),
    lookup: 0,
  }
}

pub fn ale_console_input(console: &mut Console, input: &Input) {
  match input {
    Input::Key(key, scancode, action, modifier) => {
      if action == &ale_input::Action::Press {
        if key == &ale_input::Key::Enter {
          match intern_on_enter(console) {
            CommandNotFound(cmd) => intern_add_output_line(console, &format!("{} is an invalid command", cmd)),
            ArgumentsNumberDiffer(cmd, actual, expected) => intern_add_output_line(
              console,
              &format!("{} has {} argument(s), need {}", cmd, actual, expected),
            ),
            ArgumentsFailToParse(cmd, arg) => {
              intern_add_output_line(console, &format!("{} fail to parse {}, type mismatch", cmd, arg))
            }
            CommandEmpty => {}
            ArgumentSet => {}
            ArgumentPrint(cmd, val) => {
              intern_add_output_line(console, &format!("{}", val));
            }
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
  console.variables.insert(ale_variable_name_get(&variable), variable);
}

pub fn ale_console_command_register(console: &mut Console) {}

fn intern_on_enter(console: &mut Console) -> OnEnterResult {
  // If the console has too many lines already, we pop the earliest one
  if console.lines.len() > console.max_lines {
    console.lines.pop_front();
  }
  if console.commands.len() > console.max_lines {
    console.commands.pop_front();
  }

  // Find out what is the entered line
  let entered_line = console.line_buffer.clone();
  intern_add_command_line(console, &entered_line);
  console.line_buffer.clear();

  let words: Vec<&str> = entered_line.split_whitespace().collect();

  if words.is_empty() {
    return CommandEmpty;
  }

  let variable = console.variables.get_mut(&words[0].to_owned());

  match variable {
    None => return CommandNotFound(entered_line.to_owned()),
    Some(variable) => {
      if words.len() == 1 {
        return ArgumentPrint(words[0].to_owned(), ale_variable_value_to_string(variable));
      }

      match variable {
        Variable::F32_1(_, v) => {
          let k: Vec<f32> = match intern_parse_words_len(&words, 1) {
            Ok(v) => v,
            Err(e) => return e,
          };
          *v = k[0];
        }
        Variable::F32_3(_, ref mut v) => {
          let k: Vec<f32> = match intern_parse_words_len(&words, 3) {
            Ok(v) => v,
            Err(e) => return e,
          };
          v.x = k[0];
          v.y = k[1];
          v.z = k[2];
        }
        Variable::F32_4(_, ref mut v) => {
          let k: Vec<f32> = match intern_parse_words_len(&words, 4) {
            Ok(v) => v,
            Err(e) => return e,
          };
          v.x = k[0];
          v.y = k[1];
          v.z = k[2];
          v.w = k[3];
        }
      }
    }
  };

  OnEnterResult::ArgumentSet
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

fn intern_add_output_line(console: &mut Console, cmd: &str) {
  console.lines.push_back(format!(">> {}", cmd));
}

pub enum OnEnterResult {
  CommandEmpty,
  CommandNotFound(String /* cmd */),
  ArgumentsNumberDiffer(String /* cmd */, /* actual */ i32, /* expected */ i32),
  ArgumentsFailToParse(String /* cmd */, String /* arg */),
  ArgumentSet,
  ArgumentPrint(String /* cmd */, String /* value */),
}
