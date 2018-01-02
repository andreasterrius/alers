use time;

pub struct TimestampIdGenerator {
    last_id : i64
}

impl TimestampIdGenerator {

    pub fn new() -> TimestampIdGenerator {
        TimestampIdGenerator{
            last_id: 0
        }
    }

    pub fn next(&mut self) -> i64 {
        let time = time::now().to_timespec();
        let mut id = time.sec * 1000000000 + time.nsec as i64;
        if id <= self.last_id { id = self.last_id + 1; }

        self.last_id = id;
        id
    }
}