extern crate alexyt;
use alexyt::ale::scene::SceneLoader;

fn main() {
    let mut scene_loader = SceneLoader::new();
    alexyt::start_engine(scene_loader);
}
