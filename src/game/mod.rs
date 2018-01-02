use cgmath::{Vector2, Vector3, Matrix4};
use cgmath::prelude::*;
use math::Transform2D;
use fisika::{self, BoxCollider2D, CircleCollider2D};
use ale::input::Input;
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
use audio::AudioPlayer;

mod block;
mod ball;
mod paddle;
mod postprocess;
mod powerup;

use self::block::Block;
use self::ball::Ball;
use self::paddle::Paddle;

pub struct Game {
    blocks : Vec<Block>,
    paddle : Paddle,
    ball : Ball,

    ball_particle : ParticleEmitter,
    postprocess : PostProcess,

    powerups : Vec<Powerup>,
    arena_width : u32,
    arena_height : u32,
    start_time : i64 //game start time

}

impl Game  {
    pub fn new(arena_width : u32, arena_height : u32, idgen : &mut TimestampIdGenerator) -> Game  {

        let blocks = Block::arena(arena_width, arena_height, idgen);
        let paddle = Paddle::new(arena_width, arena_height, idgen.next());
        let ball = Ball::new(arena_width, arena_height, &paddle, idgen.next());
        let ball_particle = ParticleEmitter::new(500, 1.0, idgen);
        let postprocess = PostProcess::new();

        Game {
            blocks,
            paddle,
            ball,

            ball_particle,
            postprocess,
            powerups : vec!(),
            
            arena_width,
            arena_height,
            start_time : time::get_millisecond_epoch()
        }
    }

    pub fn load_resources(&self, resources : &mut ResourceManager) {
        //shader
        resources.load_glsl("sprite", "shaders/sprite.vs", "shaders/sprite.fs");
        resources.load_glsl("particle", "shaders/particle.vs", "shaders/particle.fs");
        resources.load_glsl("postprocess", "shaders/postprocess.vs", "shaders/postprocess.fs");

        //sprites
        resources.load_image("ball", "resources/ball.png");
        resources.load_image("block", "resources/block.png");
        resources.load_image("block_solid", "resources/block_solid.png");
        resources.load_image("paddle", "resources/paddle.png");
        resources.load_image("ballparticle", "resources/particle.png");

        resources.load_image("powerup_chaos", "resources/powerup_chaos.png");
        resources.load_image("powerup_confuse", "resources/powerup_confuse.png");
        resources.load_image("powerup_increase", "resources/powerup_increase.png");
        resources.load_image("powerup_passthrough", "resources/powerup_passthrough.png");
        resources.load_image("powerup_speed", "resources/powerup_speed.png");
        resources.load_image("powerup_sticky", "resources/powerup_sticky.png");

        //audio
        resources.load_audio("bgm", "resources/breakout.ogg");
        resources.load_audio("bounce", "resources/bleep.ogg");
        resources.load_audio("bleep", "resources/bleep2.wav");
        resources.load_audio("solid", "resources/solid.wav");
        resources.load_audio("powerup", "resources/powerup.wav");

    }

    pub fn configure_renderer(&self, resources : &ResourceManager, renderer: &mut OpenGLRenderer){
        let particle_shader = resources.get_glsl("particle").unwrap();
        let postprocess_shader = resources.get_glsl("postprocess").unwrap();
        let sprite_shader = resources.get_glsl("sprite").unwrap();

        renderer.register_shader("particle", &particle_shader.vertex_shader, &particle_shader.fragment_shader);
        renderer.register_shader("postprocess", &postprocess_shader.vertex_shader, &postprocess_shader.fragment_shader);
        renderer.register_shader("sprite", &sprite_shader.vertex_shader, &sprite_shader.fragment_shader);

        renderer.register_image("ball", &resources.get_image("ball").unwrap().image);
        renderer.register_image("block", &resources.get_image("block").unwrap().image);
        renderer.register_image("block_solid", &resources.get_image("block_solid").unwrap().image);
        renderer.register_image("paddle", &resources.get_image("paddle").unwrap().image);
        renderer.register_image("ballparticle", &resources.get_image("ballparticle").unwrap().image);

        renderer.register_image("powerup_chaos", &resources.get_image("powerup_chaos").unwrap().image);
        renderer.register_image("powerup_confuse", &resources.get_image("powerup_confuse").unwrap().image);
        renderer.register_image("powerup_increase", &resources.get_image("powerup_increase").unwrap().image);
        renderer.register_image("powerup_passthrough", &resources.get_image("powerup_passthrough").unwrap().image);
        renderer.register_image("powerup_speed", &resources.get_image("powerup_speed").unwrap().image);
        renderer.register_image("powerup_sticky", &resources.get_image("powerup_sticky").unwrap().image);

        renderer.register_preprocessor("postprocess");
        renderer.register_uniforms("postprocess", &self.get_postprocess_uniforms());
    }

    pub fn configure_audio(&self, resources : &ResourceManager, audio_player : &mut AudioPlayer){
        audio_player.register_audio("bgm", resources.get_audio("bgm").unwrap().audio.clone());
        audio_player.register_audio("bounce", resources.get_audio("bounce").unwrap().audio.clone());
        audio_player.register_audio("bleep", resources.get_audio("bleep").unwrap().audio.clone());
        audio_player.register_audio("solid", resources.get_audio("solid").unwrap().audio.clone());
        audio_player.register_audio("powerup", resources.get_audio("powerup").unwrap().audio.clone());

        audio_player.play_audio_simple("bgm", true);
    }

    pub fn get_postprocess_uniforms(&self) -> CustomShaderUniform {

        let mut shader = CustomShaderUniform {
            uniforms: HashMap::new(),
        };

        let offset = 1.0 / 300.0;
        let offsets = vec![
            ( -offset,  offset  ),  // top-left
            (  0.0,     offset  ),  // top-center
            (  offset,  offset  ),  // top-right
            ( -offset,  0.0    ),  // center-left
            (  0.0,     0.0    ),  // center-center
            (  offset,  0.0    ),  // center - right
            ( -offset, -offset  ),  // bottom-left
            (  0.0,    -offset  ),  // bottom-center
            (  offset, -offset  )
        ];
        shader.uniforms.insert(String::from("offsets"), ShaderUniform::Float2vArray(offsets));
        
        let edge_kernel = vec!(
            -1, -1, -1,
            -1,  8, -1,
            -1, -1, -1
        );
        shader.uniforms.insert(String::from("edge_kernel"), ShaderUniform::Integer1vArray(edge_kernel));

        let blur_kernel  = vec!(
            1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0,
            2.0 / 16.0, 4.0 / 16.0, 2.0 / 16.0,
            1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0
        );
        shader.uniforms.insert(String::from("blur_kernel"), ShaderUniform::Float1vArray(blur_kernel));

        shader
    }

    pub fn get_postprocess(&self) -> CustomShaderUniform {

        let mut uniforms = HashMap::new();
        uniforms.insert(String::from("shake"), ShaderUniform::Boolean(self.postprocess.shake));
        uniforms.insert(String::from("confuse"), ShaderUniform::Boolean(self.postprocess.confuse));
        uniforms.insert(String::from("chaos"), ShaderUniform::Boolean(self.postprocess.chaos));

        let t = (time::get_millisecond_epoch() - self.start_time) as f32 / 100.0;
        uniforms.insert(String::from("time"), ShaderUniform::Float(t));

        CustomShaderUniform
            { uniforms }
    }

    pub fn get_renderables(&self) -> BTreeMap<i64, RenderJob> {
        let mut renderjobs = BTreeMap::new();
        for block in &self.blocks {
            if !block.is_alive { continue; }
            renderjobs.insert(block.id, RenderJob::Sprite(block.transform2d.clone(), block.sprite.clone()));
        }

        renderjobs.insert(self.paddle.id, RenderJob::Sprite(self.paddle.transform2d.clone(), self.paddle.sprite.clone()));
        renderjobs.insert(self.ball.id, RenderJob::Sprite(self.ball.transform2d.clone(), self.ball.sprite.clone()));
        renderjobs.extend(self.ball_particle.get_renderables());

        for powerup in &self.powerups {
            if !powerup.is_alive { continue; }
            renderjobs.insert(powerup.id, powerup.get_renderable());
        }

        renderjobs
    }

    pub fn reset_game(&mut self, idgen : &mut TimestampIdGenerator, timer_ticker : &mut TimerManager){
        self.blocks = Block::arena(self.arena_width, self.arena_height, idgen);
        self.paddle = Paddle::new(self.arena_width, self.arena_height, idgen.next());
        self.ball = Ball::new(self.arena_width, self.arena_height, &self.paddle, idgen.next());
        self.postprocess = PostProcess::new();
        self.powerups.clear();
        
        timer_ticker.destroy_all_timer();
    }

    pub fn fixed_tick(&mut self, dt : f32,
                      input : &Input,
                      timer_manager : &mut TimerManager,
                      audio_player : &AudioPlayer,
                      idgen : &mut TimestampIdGenerator) {

        /* Handle input */
        let move_right = {
            let mut kk = 0.0;

            kk += input.get_key(&Key::Right)
                .map_or(0.0, | action | {
                    match *action {
                        Action::Press => 1.0,
                        Action::Repeat => 1.0,
                        _ => 0.0
                    }
                });
            kk += input.get_key(&Key::Left)
                .map_or(0.0, | action | {
                    match *action {
                        Action::Press => -1.0,
                        Action::Repeat => -1.0,
                        _ => 0.0
                    }
                });

            kk
        };

        /* Get past y = 0 */
        if self.ball.transform2d.position.y > self.arena_height as f32 {
            self.reset_game(idgen, timer_manager);
        }

        if let Some(action) = input.get_key(&Key::Space){
            match action {
                &Action::Release => {},
                &Action::Press => { self.ball.is_sticky = false; },
                &Action::Repeat => { self.ball.is_sticky = false; },
            }
        };

        /* Physics tick */
        self.ball_particle.fixed_tick(fisika::get_center_pos_circle(&self.ball), dt);
        self.paddle.do_move(dt, move_right);

        /* Ball sticky check */
        if self.ball.is_sticky {
            self.ball.transform2d.position.x += self.paddle.get_velocity(move_right).x;
            self.ball.transform2d.position.y = self.paddle.transform2d.position.y - self.ball.transform2d.size.y;
        } else {
            self.ball.do_move(dt);
        }

        /* Blocks collision check */
        for block in &mut self.blocks {
            if !block.is_alive() { continue }
            if let Some((closest_point, diff)) = fisika::aabb_collision_box_circle(block, &self.ball) {
                let area = fisika::determine_point_in_box_general(block, closest_point);

                if !timer_manager.is_exist_timer("passthrough") {
                    match area {
                        BoxGeneralArea::Top => {
                            self.ball.velocity.y = -self.ball.velocity.y;
                            self.ball.transform2d.position.y = block.transform2d.position.y + block.transform2d.size.y;
                        },
                        BoxGeneralArea::Bottom => {
                            self.ball.velocity.y = -self.ball.velocity.y;
                            self.ball.transform2d.position.y = block.transform2d.position.y - self.ball.transform2d.size.y;
                        },
                        BoxGeneralArea::Left => {
                            self.ball.velocity.x = -self.ball.velocity.x;
                            self.ball.transform2d.position.x = block.transform2d.position.x - self.ball.transform2d.size.x;
                        },
                        BoxGeneralArea::Right => {
                            self.ball.velocity.x = -self.ball.velocity.x;
                            self.ball.transform2d.position.x = block.transform2d.position.x + block.transform2d.size.x;
                        },
                    }
                }

                if !block.is_solid {
                    block.destroy();

                    audio_player.play_audio_simple("bounce", false);

                    if let Some(powerup) = Powerup::probably_spawn(block.worldpos(), idgen.next()) {
                        self.powerups.push(powerup);
                    }
                }
                else {
                    audio_player.play_audio_simple("solid", false);

                    self.postprocess.shake = true;
                    timer_manager.register_timer("shake", Timer::new(0.1));
                }
                break;
            }
        }

        /* Paddle collision check */
        if let Some((closest_point, diff)) = fisika::aabb_collision_box_circle(&self.paddle, &self.ball) {

            let area = fisika::determine_point_in_box_general(&self.paddle, closest_point);
            match area {
                BoxGeneralArea::Top => {
                    self.ball.velocity.y = -(self.ball.velocity.y.abs());
                },
                BoxGeneralArea::Bottom => {
                    self.ball.velocity.y = -(self.ball.velocity.y.abs());
                },
                BoxGeneralArea::Left => {
                    self.ball.velocity.y = -(self.ball.velocity.y.abs());
                    self.ball.velocity.x = -self.ball.velocity.x;
                },
                BoxGeneralArea::Right => {
                    self.ball.velocity.y = -(self.ball.velocity.y.abs());
                    self.ball.velocity.x = -self.ball.velocity.x;
                },
            }

            let x_mult = closest_point.x / (self.paddle.size().x / 2.0);
            self.ball.multiply_speed(2.0 * x_mult);

            audio_player.play_audio_simple("bleep", false);
            if timer_manager.is_exist_timer("sticky") {
                self.ball.is_sticky = true;
            }
        }

        /* Powerup tick & collision check */
        let pu_delete = vec!();
        for i in 0..self.powerups.len() {
            let pwup = &mut self.powerups[i];
            if !pwup.is_alive { continue; }
            pwup.fixed_tick(dt);

            if fisika::aabb_collision_box_box(pwup, &self.paddle){
                use self::powerup::PowerupType::*;

                match pwup.pu_type {
                    Speed => {
                        if !timer_manager.is_exist_timer("speed") {
                            self.ball.velocity *= 2.0;
                        }
                        timer_manager.register_timer("speed", Timer::new(5.0));
                    },
                    Sticky => {
                        timer_manager.register_timer("sticky", Timer::new(5.0));
                    },
                    PassThrough => {
                        timer_manager.register_timer("passthrough", Timer::new(5.0));
                    },
                    PadSize => {
                        if !timer_manager.is_exist_timer("padsize") {
                            self.paddle.transform2d.size.x *= 2.0;
                        }
                        timer_manager.register_timer("padsize", Timer::new(5.0));
                    },
                    Confuse => {
                        if !timer_manager.is_exist_timer("confuse"){
                            self.postprocess.confuse = true;
                        }
                        timer_manager.register_timer("confuse", Timer::new(3.0));
                    },
                    Chaos => {
                        if !timer_manager.is_exist_timer("chaos"){
                            self.postprocess.chaos = true;
                        }
                        timer_manager.register_timer("chaos", Timer::new(3.0));
                    }
                }

                audio_player.play_audio_simple("powerup", false);
                pwup.destroy();
            }
        }
        
        for index in pu_delete {
            self.powerups.remove(index);
        }

        /* Timer check */
        if let Some(diff) = timer_manager.has_completed_timer("shake") {
            self.postprocess.shake = false;
            timer_manager.destroy_timer("shake");
        }

        if let Some(diff) = timer_manager.has_completed_timer("speed") {
            self.ball.velocity /= 2.0;
            timer_manager.destroy_timer("speed");
        }

        if let Some(diff) = timer_manager.has_completed_timer("sticky") {
            timer_manager.destroy_timer("sticky");
        }

        if let Some(diff) = timer_manager.has_completed_timer("passthrough") {
            timer_manager.destroy_timer("passthrough");
        }

        if let Some(diff) = timer_manager.has_completed_timer("padsize") {
            self.paddle.transform2d.size.x /= 2.0;
            timer_manager.destroy_timer("padsize");
        }

        if let Some(diff) = timer_manager.has_completed_timer("chaos") {
            self.postprocess.chaos = false;
            timer_manager.destroy_timer("chaos");
        }

        if let Some(diff) = timer_manager.has_completed_timer("confuse") {
            self.postprocess.confuse = false;
            timer_manager.destroy_timer("confuse");
        }
    }
}