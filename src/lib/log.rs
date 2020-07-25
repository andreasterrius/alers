use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn init() {
  let now_ms = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
  CombinedLogger::init(vec![
    TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed).unwrap(),
    WriteLogger::new(
      LevelFilter::Info,
      Config::default(),
      File::create(format!("alers-{}.log", now_ms)).unwrap(),
    ),
  ])
  .unwrap();
}

pub fn init_test() {
  TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap();
}
