use math::Transform2D;
use renderer::job::{SpriteRenderable};
use cgmath::prelude::*;
use cgmath::{Vector2, Vector3, Vector4};
use fisika::BoxCollider2D;

pub struct Paddle  {
    pub id : i64,
    pub transform2d : Transform2D,
    pub sprite: SpriteRenderable,

    pub speed : f32,
    pub moving_right : f32
}

impl Paddle {

    pub fn new(arena_width : u32, arena_height : u32, id : i64) -> Paddle {
        let size = Vector2::new(95.0, 25.0);
        let position = Vector2::new(
            arena_width as f32 / 2.0 - size.x / 2.0,
            arena_height as f32 - size.y
        );

        let transform2d = Transform2D {
            position,
            size
        };

        Paddle {
            id,
            transform2d,
            sprite: SpriteRenderable::new(
                Vector4::from_value(1.0),
                String::from("sprite"),
                vec!(String::from("paddle")),
            ),
            speed : 10.0,
            moving_right: 0.0,
        }
    }

    pub fn do_move(&mut self, dt : f32, input : f32){
        self.transform2d.position += self.get_velocity(input);
    }

    pub fn get_velocity(&self, input : f32) -> Vector2<f32> {
        Vector2::new(input * self.speed, 0.0)
    }
}

impl BoxCollider2D for Paddle {
    fn worldpos(&self) -> Vector2<f32> {
        self.transform2d.position
    }

    fn size(&self) -> Vector2<f32> {
        self.transform2d.size
    }
}
