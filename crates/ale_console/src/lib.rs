mod backend;

use crate::ConsoleEvent::{Print, Set};
use crate::OnEnterResult::{ArgumentsFailToParse, ArgumentsNumberDiffer, CommandEmpty, CommandNotFound, EventQueued};
use ale_input::Input;
use ale_input::Input::Key;
use ale_math::num_traits::clamp;
use ale_math::{Vector3, Vector4};
use ale_variable::Variable;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::process::Command;
use std::str::FromStr;
use std::sync::mpsc::TryRecvError::Empty;

pub struct Console {
  // All lines that will be rendered
  pub lines: VecDeque<String>,

  // All commands from user, and capabillity to look it up
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

impl Console {
  pub fn new(max_lines: usize) -> Console {
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

  pub fn handle_input(&mut self, input: &Input) {
    match input {
      Input::Key(key, scancode, action, modifier) => {
        if action == &ale_input::Action::Press {
          if key == &ale_input::Key::Enter {
            match self.on_enter() {
              CommandNotFound(cmd) => self.print_output(&format!("{} is an invalid command", cmd)),
              ArgumentsNumberDiffer(cmd, actual, expected) => {
                self.print_output(&format!("{} has {} argument(s), need {}", cmd, actual, expected))
              }
              ArgumentsFailToParse(cmd, arg) => {
                self.print_output(&format!("{} fail to parse {}, type mismatch", cmd, arg))
              }
              CommandEmpty => {}
              EventQueued => {}
            };
          } else if key == &ale_input::Key::GraveAccent {
            self.has_focus = !self.has_focus;
          } else if key == &ale_input::Key::Backspace {
            self.line_buffer.pop();
          }

          if key == &ale_input::Key::Up {
            if !self.commands.is_empty() {
              self.lookup = clamp(self.lookup, 0, self.commands.len() - 1);
              self.line_buffer = self.commands.iter().rev().nth(self.lookup).unwrap().clone();
              self.lookup = clamp(self.lookup + 1, 0, self.commands.len() - 1);
            }
          } else if key == &ale_input::Key::Down {
            if !self.commands.is_empty() && self.lookup > 0 {
              self.lookup = clamp(self.lookup - 1, 0, self.commands.len() - 1);
              self.line_buffer = self.commands.iter().rev().nth(self.lookup).unwrap().clone();
            } else {
              self.line_buffer.clear();
            }
          } else {
            // Any other key
            self.lookup = 0;
          }
        }
      }
      Input::Char(char) => {
        if self.has_focus && char != &'`' {
          self.line_buffer.push(char.clone());
        }
      }
      _ => {}
    }
  }

  pub fn variable_register(&mut self, variable: Variable) {
    self.registered_commands.insert(variable.name_str(), variable);
  }

  pub fn variable_handle_event<T: From<Variable>>(&mut self, variable: Variable) -> T {
    match self.events.remove(&variable.name_str()).unwrap_or(ConsoleEvent::None) {
      Print => self.print_output(&variable.value_str()),
      Set(x) => return x.into(),
      ConsoleEvent::None => {}
    }

    return variable.into();
  }

  pub fn variable_has_event(&self) -> bool {
    !self.events.is_empty()
  }

  pub fn print_output(&mut self, cmd: &str) {
    self.lines.push_back(format!(">> {}", cmd));
  }

  fn on_enter(&mut self) -> OnEnterResult {
    // If the console has too many lines already, we pop the earliest one
    if self.lines.len() > self.max_lines {
      self.lines.pop_front();
    }
    if self.commands.len() > self.max_lines {
      self.commands.pop_front();
    }

    // Get the entered line, add it to console buffer and clear line buffer
    let entered_line = self.line_buffer.clone();
    self.add_command_line(&entered_line);
    self.line_buffer.clear();

    // Process the arguments
    let words: Vec<&str> = entered_line.split_whitespace().collect();
    if words.is_empty() {
      return CommandEmpty;
    }

    let variable = self.registered_commands.get_mut(&words[0].to_owned());

    match variable {
      None => return CommandNotFound(entered_line.to_owned()),
      Some(variable) => {
        if words.len() == 1 {
          self.events.insert(words[0].to_owned(), Print);
          return OnEnterResult::EventQueued;
        }

        match variable {
          Variable::F32_1(key, v) => {
            let val: Vec<f32> = match parse_words_len(&words, 1) {
              Ok(v) => v,
              Err(e) => return e,
            };
            *v = val[0];
          }
          Variable::F32_3(key, ref mut v) => {
            let val: Vec<f32> = match parse_words_len(&words, 3) {
              Ok(v) => v,
              Err(e) => return e,
            };
            v.x = val[0];
            v.y = val[1];
            v.z = val[2];
          }
          Variable::F32_4(key, ref mut v) => {
            let val: Vec<f32> = match parse_words_len(&words, 4) {
              Ok(v) => v,
              Err(e) => return e,
            };
            v.x = val[0];
            v.y = val[1];
            v.z = val[2];
            v.w = val[3];
          }
          Variable::Bool(key, ref mut v) => {
            let val: Vec<bool> = match parse_words_len(&words, 1) {
              Ok(v) => v,
              Err(e) => return e,
            };
            *v = val[0];
          }
          Variable::Void(_) => {}
        }
        self.events.insert(words[0].to_owned(), Set(variable.clone()))
      }
    };

    OnEnterResult::EventQueued
  }

  fn add_command_line(&mut self, cmd: &str) {
    self.lines.push_back(cmd.to_owned());
    if !cmd.is_empty() {
      self.commands.push_back(cmd.to_owned());
    }
  }
}

fn parse_words_len<T: FromStr>(words: &[&str], len: usize) -> Result<Vec<T>, OnEnterResult> {
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
