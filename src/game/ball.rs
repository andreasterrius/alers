use game::paddle::Paddle;
use math::Transform2D;
use renderer::job::{SpriteRenderable};
use cgmath::prelude::*;
use cgmath::{Vector2, Vector3, Vector4};
use fisika::{CircleCollider2D, BoxGeneralArea};

pub struct Ball  {
    pub id : i64,
    pub transform2d : Transform2D,
    pub sprite: SpriteRenderable,
    pub velocity : Vector2<f32>,

    pub is_sticky : bool
}

impl Ball {

    pub fn new(arena_width : u32, arena_height : u32, paddle : &Paddle, id : i64) -> Ball {
        let size = Vector2::new(30.0, 30.0);
        let position = Vector2::new(
            arena_width as f32 / 2.0 - size.x / 2.0,
            arena_height as f32 - size.y - paddle.transform2d.size.y
        );

        let transform2d = Transform2D {
            position,
            size,
            depth: 0.0,
        };

        Ball {
            id,
            transform2d,
            sprite: SpriteRenderable::new(
                Vector4::from_value(1.0),
                String::from("sprite"),
                None,
                vec!(String::from("ball")),
            ),
            velocity: Vector2::new(0.0, -500.0),
            is_sticky : true,
        }
    }

    pub fn do_move(&mut self, dt : f32) {

        self.transform2d.position += self.velocity * dt;

        if self.transform2d.position.x <= 0.0 {
            self.velocity.x = -self.velocity.x;
            self.transform2d.position.x = 0.0
        }
            else if self.transform2d.position.x + self.transform2d.size.x >= 800.0 {
                self.velocity.x = - self.velocity.x;
                self.transform2d.position.x = 800.0 - self.transform2d.size.x;
            }

        if self.transform2d.position.y <= 0.0
            {
                self.velocity.y = -self.velocity.y;
                self.transform2d.position.y = 0.0;
            }
    }


    pub fn multiply_speed(&mut self, x_mult : f32){
        self.velocity.x = x_mult * 100.0;
        if self.velocity.y < 0.0 {
            self.velocity.y -= 12.0;
        } else {
            self.velocity.y += 12.0;
        }
    }
}

impl CircleCollider2D for Ball {
    fn worldpos(&self) -> Vector2<f32> {
        self.transform2d.position
    }

    fn radius(&self) -> f32 {
        self.transform2d.size.x/2.0
    }
}