extern crate alexyt;
use alexyt::ale::scene::SceneLoader;

mod breakout;

/* Create an instance of breakout game */
fn play_breakout(){
    alexyt::start_engine(|&mut scene_loader| {
    
    });
}

fn main() {
    play_breakout();
}
