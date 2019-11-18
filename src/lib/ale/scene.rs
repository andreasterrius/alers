use graphics::job::RenderJob;
use ale::input::InputManager;
use ale::time::TimerManager;
use audio::AudioManager;
use graphics::shader::CustomShaderUniform;
use resource::ResourceManager;
use std::collections::{HashMap, BTreeMap};
use graphics::opengl::OpenGLRenderer;
use ale::idgen::TimestampIdGenerator;

pub type SceneId = String;

pub trait Scene {
    fn get_scene_id(&self) -> SceneId;

    fn get_renderables(&self) -> BTreeMap<i64, RenderJob>;

    fn fixed_tick(&mut self, dt: f32, input_manager: &InputManager, audio_manager: &AudioManager, timer_manager: &mut TimerManager, idgen: &mut TimestampIdGenerator);

    fn get_postprocess_uniforms(&self) -> CustomShaderUniform;

    fn get_postprocess_tick(&self) -> CustomShaderUniform;

    fn load_resources(&self, resources: &mut ResourceManager);

    fn configure_renderer(&self, resources: &ResourceManager, renderer: &mut OpenGLRenderer);

    fn configure_audio(&self, resources: &ResourceManager, audio_manager: &mut AudioManager);
}

pub struct SceneLoader {
    active_scene: SceneId,
    scenes: HashMap<SceneId, Box<Scene>>,
}

impl SceneLoader {
    pub fn new() -> SceneLoader {
        SceneLoader {
            active_scene: String::new(),
            scenes: HashMap::new(),
        }
    }

    pub fn register_scene(&mut self, scene: Box<Scene>) {
        self.scenes.insert(scene.get_scene_id(), scene);
    }

    pub fn switch_to_scene(&mut self, key: SceneId) {
        self.active_scene = key;
    }

    pub fn get_active_scene(&mut self) -> &mut Box<Scene> {
        self.scenes.get_mut(&self.active_scene).unwrap()
    }
}
