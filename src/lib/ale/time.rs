use time;
use std::collections::HashMap;

pub struct TimerManager {
    timers: HashMap<String, Timer>,
}

impl TimerManager {
    pub fn new() -> TimerManager {
        TimerManager { timers: HashMap::new() }
    }

    pub fn fixed_tick(&mut self, dt: f32) {
        for (key, timer) in &mut self.timers {
            timer.fixed_tick(dt);
        }
    }

    pub fn register_timer(&mut self, key: &str, timer: Timer) {
        self.timers.insert(String::from(key), timer);
    }


    pub fn has_completed_timer(&mut self, key: &str) -> Option<f32> {
        match self.timers.get(key) {
            None => None,
            Some(timer) => timer.has_completed(),
        }
    }

    pub fn is_exist_timer(&self, key: &str) -> bool {
        self.timers.get(key).is_some()
    }

    pub fn reset_timer(&mut self, key: &str) {
        self.timers.get_mut(key).unwrap().reset();
    }

    pub fn destroy_timer(&mut self, key: &str) {
        self.timers.remove(key);
    }

    pub fn destroy_all_timer(&mut self) {
        self.timers.clear();
    }
}

pub struct Timer {
    interval_sec: f32,
    curr_interval_sec: f32,
}

impl Timer {
    pub fn new(interval_sec: f32) -> Timer {
        Timer {
            interval_sec,
            curr_interval_sec: interval_sec,
        }
    }

    fn fixed_tick(&mut self, dt: f32) {
        self.curr_interval_sec -= dt;
    }

    fn has_completed(&self) -> Option<f32> {
        if self.curr_interval_sec < 0.0 {
            return Some(self.curr_interval_sec);
        }

        None
    }

    fn reset(&mut self) {
        self.curr_interval_sec = self.interval_sec;
    }
}

//returns current time in microsecond
pub fn get_nanosecond_epoch() -> i64 {
    let time = time::now().to_timespec();
    time.sec * 1000000000 + time.nsec as i64
}

pub fn get_millisecond_epoch() -> i64 {
    self::get_nanosecond_epoch() / 1000000
}

pub fn get_second_epoch() -> i64 {
    time::now().to_timespec().sec
}
