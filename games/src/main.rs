extern crate alexyt;
use alexyt::ale::scene::SceneLoader;

mod breakout;
use breakout::breakout::BreakoutScene;
use alexyt::ale::scene::Scene;

/* Create an instance of breakout game */
fn play_breakout(){
    alexyt::start_engine(|scene_loader, idgen,
                          arena_width,  arena_height| 
    {
        let game_scene = BreakoutScene::new(arena_width, arena_height, idgen);
        scene_loader.switch_to_scene(game_scene.get_scene_id());
        scene_loader.register_scene(Box::<BreakoutScene>::new(game_scene));
    });
}

fn main() {
    play_breakout();
}
