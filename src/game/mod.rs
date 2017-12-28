use cgmath::{Vector2, Vector3, Matrix4};
use cgmath::prelude::*;
use math::Transform2D;
use fisika::{self, BoxCollider2D, CircleCollider2D};
use ale::input::Input;
use glfw::{Key, Action};
use resource::*;
use renderer::job::RenderJob;
use renderer::opengl::*;
use ale::idgen::TimestampIdGenerator;
use std::collections::HashMap;
use fisika::BoxGeneralArea;
use ale::particle::{ParticleEmitter, Particle};

mod block;
mod ball;
mod paddle;

use self::block::Block;
use self::ball::Ball;
use self::paddle::Paddle;

pub struct Game {
    blocks : Vec<Block>,
    paddle : Paddle,
    ball : Ball,

    ball_particle : ParticleEmitter
}

impl Game  {
    pub fn new(arena_width : u32, arena_height : u32) -> Game  {

        let mut idgen = TimestampIdGenerator::new();

        let blocks = Block::arena(arena_width, arena_height, &mut idgen);
        let paddle = Paddle::new(arena_width, arena_height, idgen.next());
        let ball = Ball::new(arena_width, arena_height, &paddle, idgen.next());
        let ball_particle = ParticleEmitter::new(500, 1.0, &mut idgen);

        Game {
            blocks,
            paddle,
            ball,

            ball_particle
        }
    }

    pub fn load_resources(&self, resources : &mut ResourceManager) {
        resources.load_glsl("particle", "shaders/particle.vs", "shaders/particle.fs");
        resources.load_image("ball", "resources/ball.png");
        resources.load_image("block", "resources/block.png");
        resources.load_image("paddle", "resources/paddle.png");
        resources.load_image("ballparticle", "resources/particle.png");
    }

    pub fn configure_renderer(&self, resources : &ResourceManager, renderer: &mut OpenGLRenderer){
        let particle_shader = resources.get_glsl("particle").unwrap();
        renderer.register_shader("particle", &particle_shader.vertex_shader, &particle_shader.fragment_shader);

        renderer.register_image("ball", &resources.get_image("ball").unwrap().image);
        renderer.register_image("block", &resources.get_image("block").unwrap().image);
        renderer.register_image("paddle", &resources.get_image("paddle").unwrap().image);
        renderer.register_image("ballparticle", &resources.get_image("ballparticle").unwrap().image);
    }

    pub fn get_renderables(&self) -> HashMap<i64, RenderJob> {
        let mut renderjobs = HashMap::new();
        for block in &self.blocks {
            if !block.is_alive { continue; }
            renderjobs.insert(block.id, RenderJob::Sprite(block.transform2d.clone(), block.sprite.clone()));
        }

        renderjobs.insert(self.paddle.id, RenderJob::Sprite(self.paddle.transform2d.clone(), self.paddle.sprite.clone()));
        renderjobs.insert(self.ball.id, RenderJob::Sprite(self.ball.transform2d.clone(), self.ball.sprite.clone()));
        renderjobs.extend(self.ball_particle.get_renderables());

        renderjobs
    }

    pub fn fixed_tick(&mut self, dt : f32, input : &Input) {

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

        /* Physics tick */
        self.ball.do_move(dt);
        self.ball_particle.fixed_tick(fisika::get_center_pos_circle(&self.ball), dt);
        self.paddle.do_move(dt, move_right);

        for block in &mut self.blocks {
            if !block.is_alive() { continue }

            let closest_point_opt = fisika::aabb_collision_box_circle(block, &self.ball);

            if let Some(closest_point) = closest_point_opt {
                let area_opt = fisika::determine_point_in_box_general(block, closest_point);
                if let Some(area) = area_opt {
                    self.ball.bounce(&area, Vector2::from_value(0.0));
                    block.destroy();
                    break;
                }
            }
        }

        let closest_point_opt2 = fisika::aabb_collision_box_circle(&self.paddle, &self.ball);
        if let Some(closest_point) = closest_point_opt2 {
            let area_opt = fisika::determine_point_in_box_general(&self.paddle, closest_point);

            if let Some(area) = area_opt {
                self.ball.bounce(&area, self.paddle.get_velocity(move_right));

                let x_mult = {
                    let center = fisika::get_center_pos_box(&self.paddle);
                    let diff = closest_point.x / self.paddle.size().x;
                    (diff - 0.5) * 2.0
                };

                self.ball.multiply_speed(x_mult + x_mult + x_mult + x_mult);
            }
        }

    }

}