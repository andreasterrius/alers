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
use ale::scene::Scene;

use game::*;
use game::block::Block;
use game::ball::Ball;
use game::paddle::Paddle;

pub struct BreakoutScene {
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

impl BreakoutScene {
    pub fn new(arena_width : u32, arena_height : u32, idgen : &mut TimestampIdGenerator) -> BreakoutScene  {

        let blocks = Block::arena(arena_width, arena_height, idgen);
        let paddle = Paddle::new(arena_width, arena_height, idgen.next());
        let ball = Ball::new(arena_width, arena_height, &paddle, idgen.next());
        let ball_particle = ParticleEmitter::new(500, 1.0, idgen);
        let postprocess = PostProcess::new();

        BreakoutScene {
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
}

impl Scene for BreakoutScene  {

    fn get_renderables(&self) -> BTreeMap<i64, RenderJob> {
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

    fn reset_game(&mut self, idgen : &mut TimestampIdGenerator, timer_ticker : &mut TimerManager){
        self.blocks = Block::arena(self.arena_width, self.arena_height, idgen);
        self.paddle = Paddle::new(self.arena_width, self.arena_height, idgen.next());
        self.ball = Ball::new(self.arena_width, self.arena_height, &self.paddle, idgen.next());
        self.postprocess = PostProcess::new();
        self.powerups.clear();

        timer_ticker.destroy_all_timer();
    }

    fn fixed_tick(&mut self, dt : f32,
                      input_manager : &InputManager,
                      timer_manager : &mut TimerManager,
                      audio_manager : &AudioManager,
                      idgen : &mut TimestampIdGenerator) {

        /* Handle input */
        let move_right = {
            let mut kk = 0.0;

            kk += input_manager.get_key(&Key::Right)
                .map_or(0.0, | action | {
                    match *action {
                        Action::Press => 1.0,
                        Action::Repeat => 1.0,
                        _ => 0.0
                    }
                });
            kk += input_manager.get_key(&Key::Left)
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

        if let Some(action) = input_manager.get_key(&Key::Space){
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

                    audio_manager.play_audio_simple("bounce", false);

                    if let Some(powerup) = Powerup::probably_spawn(block.worldpos(), idgen.next()) {
                        self.powerups.push(powerup);
                    }
                }
                    else {
                        audio_manager.play_audio_simple("solid", false);

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

            audio_manager.play_audio_simple("bleep", false);
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

                audio_manager.play_audio_simple("powerup", false);
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
