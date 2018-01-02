use renderer::job::RenderJob;
use ale::input::InputManager;
use ale::time::TimerManager;
use std::collections::{HashMap, BTreeMap};

type SceneId = String;

trait Scene {
    fn get_scene_id(&self) -> SceneId;

    fn get_renderables(&self) -> BTreeMap<i64, RenderJob>;

    fn fixed_tick(dt : f32,
                  input_manager: &InputManager,
                  audio_manager : &AudioManager,
                  timer_manager : &mut TimerManager);

    fn get_postprocess_uniforms(&self) -> CustomShaderUniform;

    fn get_postprocess_tick(&self) -> CustomShaderUniform;

    fn load_resources(resources : &mut ResourceManager);

    fn configure_renderer(resources : &ResourceManager, renderer: &mut OpenGLRenderer);

    fn configure_audio(resources : &ResourceManager, audio_manager : &mut AudioManager);
}

pub struct SceneLoader {
    active_scene : SceneId,
    scenes : HashMap<SceneId, Box<Scene>>
}

impl SceneLoader {
    pub fn new() -> SceneLoader {
        SceneLoader {
            active_scene: String::new(),
            scenes: HashMap::new(),
        }
    }

    pub fn register_scene(&mut self, scene : Scene){
        self.scenes.insert(scene.get_scene_id(), Box::new(scene));
    }

    pub fn switch_to_scene(&mut self, key : SceneId) {
        self.active_scene = key;
    }
}