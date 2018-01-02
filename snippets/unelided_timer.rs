pub struct GameLoop {
    flag: bool,
}

impl GameLoop {
    pub fn tick<'a, 'b: 'a>(&'b mut self, timers: &mut Vec<Timer<'a>>) {
        let callback = Box::new(move || {
            self.flag = true;
        });
        timers.push(Timer{ callback });
    }
}

pub struct Timer<'a> {
    callback: Box<FnMut() + 'a>,
}

fn main() {
    let mut game = GameLoop { flag: false };
    let mut timers = vec![];
    game.tick(&mut timers);

    for timer in &mut timers {
        (timer.callback)()
    }
}

