use cgmath::{Vector2, Vector3, Matrix4};
use cgmath::prelude::*;
use math::Transform2D;
use fisika::{self, BoxCollider2D, CircleCollider2D};
use ale::input::InputManager;
use glfw::{Key, Action};
use resource::*;
use renderer::job::RenderJob;
use renderer::opengl::*;
use renderer::shader::{CustomShaderUniform, ShaderUniform};
use std::collections::{HashMap, BTreeMap};
use fisika::BoxGeneralArea;
use ale::particle::{ParticleEmitter, Particle};
use game::postprocess::PostProcess;
use ale::time::{self, TimerManager, Timer};
use ale::idgen::TimestampIdGenerator;
use game::powerup::Powerup;
use audio::AudioManager;
use ale::scene::SceneLoader;

mod block;
mod ball;
mod paddle;
mod postprocess;
mod powerup;
mod breakout;
mod breakoutmenu;

use self::block::Block;
use self::ball::Ball;
use self::paddle::Paddle;
use self::breakout::BreakoutScene;

//pub fn create_scene(scene_loader : &mut SceneLoader) {
//
//}
//
//pub fn load_resources(resources : &mut ResourceManager) {
//    //shader
//    resources.load_glsl("sprite", "shaders/sprite.vs", "shaders/sprite.fs");
//    resources.load_glsl("particle", "shaders/particle.vs", "shaders/particle.fs");
//    resources.load_glsl("postprocess", "shaders/postprocess.vs", "shaders/postprocess.fs");
//
//    //sprites
//    resources.load_image("ball", "resources/ball.png");
//    resources.load_image("block", "resources/block.png");
//    resources.load_image("block_solid", "resources/block_solid.png");
//    resources.load_image("paddle", "resources/paddle.png");
//    resources.load_image("ballparticle", "resources/particle.png");
//
//    resources.load_image("powerup_chaos", "resources/powerup_chaos.png");
//    resources.load_image("powerup_confuse", "resources/powerup_confuse.png");
//    resources.load_image("powerup_increase", "resources/powerup_increase.png");
//    resources.load_image("powerup_passthrough", "resources/powerup_passthrough.png");
//    resources.load_image("powerup_speed", "resources/powerup_speed.png");
//    resources.load_image("powerup_sticky", "resources/powerup_sticky.png");
//
//    //audio
//    resources.load_audio("bgm", "resources/breakout.ogg");
//    resources.load_audio("bounce", "resources/bleep.ogg");
//    resources.load_audio("bleep", "resources/bleep2.wav");
//    resources.load_audio("solid", "resources/solid.wav");
//    resources.load_audio("powerup", "resources/powerup.wav");
//
//}

//pub fn configure_renderer(resources : &ResourceManager, renderer: &mut OpenGLRenderer){
//    let particle_shader = resources.get_glsl("particle").unwrap();
//    let postprocess_shader = resources.get_glsl("postprocess").unwrap();
//    let sprite_shader = resources.get_glsl("sprite").unwrap();
//
//    renderer.register_shader("particle", &particle_shader.vertex_shader, &particle_shader.fragment_shader);
//    renderer.register_shader("postprocess", &postprocess_shader.vertex_shader, &postprocess_shader.fragment_shader);
//    renderer.register_shader("sprite", &sprite_shader.vertex_shader, &sprite_shader.fragment_shader);
//
//    renderer.register_image("ball", &resources.get_image("ball").unwrap().image);
//    renderer.register_image("block", &resources.get_image("block").unwrap().image);
//    renderer.register_image("block_solid", &resources.get_image("block_solid").unwrap().image);
//    renderer.register_image("paddle", &resources.get_image("paddle").unwrap().image);
//    renderer.register_image("ballparticle", &resources.get_image("ballparticle").unwrap().image);
//
//    renderer.register_image("powerup_chaos", &resources.get_image("powerup_chaos").unwrap().image);
//    renderer.register_image("powerup_confuse", &resources.get_image("powerup_confuse").unwrap().image);
//    renderer.register_image("powerup_increase", &resources.get_image("powerup_increase").unwrap().image);
//    renderer.register_image("powerup_passthrough", &resources.get_image("powerup_passthrough").unwrap().image);
//    renderer.register_image("powerup_speed", &resources.get_image("powerup_speed").unwrap().image);
//    renderer.register_image("powerup_sticky", &resources.get_image("powerup_sticky").unwrap().image);
//
//    renderer.register_preprocessor("postprocess");
//    renderer.register_uniforms("postprocess", get_postprocess_uniforms());
//}
//
//pub fn configure_audio(resources : &ResourceManager, audio_manager : &mut AudioManager){
//    audio_manager.register_audio("bgm", resources.get_audio("bgm").unwrap().audio.clone());
//    audio_manager.register_audio("bounce", resources.get_audio("bounce").unwrap().audio.clone());
//    audio_manager.register_audio("bleep", resources.get_audio("bleep").unwrap().audio.clone());
//    audio_manager.register_audio("solid", resources.get_audio("solid").unwrap().audio.clone());
//    audio_manager.register_audio("powerup", resources.get_audio("powerup").unwrap().audio.clone());
//
//    audio_manager.play_audio_simple("bgm", true);
//}

//pub fn get_postprocess_uniforms() -> CustomShaderUniform {
//
//    let mut shader = CustomShaderUniform {
//        uniforms: HashMap::new(),
//    };
//
//    let offset = 1.0 / 300.0;
//    let offsets = vec![
//        ( -offset,  offset  ),  // top-left
//        (  0.0,     offset  ),  // top-center
//        (  offset,  offset  ),  // top-right
//        ( -offset,  0.0    ),  // center-left
//        (  0.0,     0.0    ),  // center-center
//        (  offset,  0.0    ),  // center - right
//        ( -offset, -offset  ),  // bottom-left
//        (  0.0,    -offset  ),  // bottom-center
//        (  offset, -offset  )
//    ];
//    shader.uniforms.insert(String::from("offsets"), ShaderUniform::Float2vArray(offsets));
//
//    let edge_kernel = vec!(
//        -1, -1, -1,
//        -1,  8, -1,
//        -1, -1, -1
//    );
//    shader.uniforms.insert(String::from("edge_kernel"), ShaderUniform::Integer1vArray(edge_kernel));
//
//    let blur_kernel  = vec!(
//        1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0,
//        2.0 / 16.0, 4.0 / 16.0, 2.0 / 16.0,
//        1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0
//    );
//    shader.uniforms.insert(String::from("blur_kernel"), ShaderUniform::Float1vArray(blur_kernel));
//
//    shader
//}
//
//pub fn get_postprocess() -> CustomShaderUniform {
//
//    let mut uniforms = HashMap::new();
//    uniforms.insert(String::from("shake"), ShaderUniform::Boolean(self.postprocess.shake));
//    uniforms.insert(String::from("confuse"), ShaderUniform::Boolean(self.postprocess.confuse));
//    uniforms.insert(String::from("chaos"), ShaderUniform::Boolean(self.postprocess.chaos));
//
//    let t = (time::get_millisecond_epoch() - self.start_time) as f32 / 100.0;
//    uniforms.insert(String::from("time"), ShaderUniform::Float(t));
//
//    CustomShaderUniform
//        { uniforms }
//}