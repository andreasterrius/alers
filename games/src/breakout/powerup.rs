use alexyt::graphics::job::RenderJob;
use alexyt::math::Transform2D;
use alexyt::cgmath::{Vector2, Vector4};
use alexyt::cgmath::prelude::*;
use alexyt::graphics::job::SpriteRenderable;
use alexyt::rand::{self, Rng};
use alexyt::fisika::BoxCollider2D;

pub struct Powerup {
    pub id : i64,
    pub transform2d : Transform2D,
    pub velocity : Vector2<f32>,
    pub pu_type : PowerupType,
    pub is_alive : bool
}

pub enum PowerupType {
    Speed,
    Sticky,
    PassThrough,
    PadSize,
    Confuse,
    Chaos
}

impl PowerupType {
    fn get_sprite(&self) -> SpriteRenderable {
        use self::PowerupType::*;
        match self {
            &Speed => {
                SpriteRenderable {
                    color: Vector4::from_value(1.0),
                    custom_shader_uniform: None,
                    custom_shader_key: String::from("sprite"),
                    texture_keys: vec!(String::from("powerup_speed")),
                }
            },
            &Sticky => {
                SpriteRenderable {
                    color: Vector4::from_value(1.0),
                    custom_shader_uniform: None,
                    custom_shader_key: String::from("sprite"),
                    texture_keys: vec!(String::from("powerup_sticky")),
                }
            },
            &PassThrough => {
                SpriteRenderable {
                    color: Vector4::from_value(1.0),
                    custom_shader_uniform: None,
                    custom_shader_key: String::from("sprite"),
                    texture_keys: vec!(String::from("powerup_passthrough")),
                }
            },
            &PadSize => {
                SpriteRenderable {
                    color: Vector4::from_value(1.0),
                    custom_shader_uniform: None,
                    custom_shader_key: String::from("sprite"),
                    texture_keys: vec!(String::from("powerup_increase")),
                }
            },
            &Confuse => {
                SpriteRenderable {
                    color: Vector4::new(1.0, 0.7, 0.7, 1.0),
                    custom_shader_uniform: None,
                    custom_shader_key: String::from("sprite"),
                    texture_keys: vec!(String::from("powerup_confuse")),
                }
            },
            &Chaos => {
                SpriteRenderable {
                    color: Vector4::new(1.0, 0.7, 0.7, 1.0),
                    custom_shader_uniform: None,
                    custom_shader_key: String::from("sprite"),
                    texture_keys: vec!(String::from("powerup_chaos")),
                }
            }
        }
    }
}

impl Powerup {

    fn new(location : Vector2<f32>, pu_type : PowerupType, id : i64 ) -> Powerup {
        Powerup {
            id,
            transform2d: Transform2D {
                position: location,
                size: Vector2::new(80.0, 30.0),
                depth: 1.0,
            },
            velocity: Vector2::new(0.0, 90.0),
            pu_type,
            is_alive: true,
        }
    }

    pub fn probably_spawn(location : Vector2<f32>, id : i64) -> Option<Powerup> {

        let mut rng = rand::thread_rng();
        let dice = rng.gen_range(0, 200);

        if dice < 15 {
            return Some(Powerup::new(location, PowerupType::Speed, id));
        }
        else if dice > 15 && dice < 30 {
            return Some(Powerup::new(location, PowerupType::Sticky, id));
        }
        else if dice > 30 && dice < 45 {
            return Some(Powerup::new(location, PowerupType::PassThrough, id));
        }
        else if dice > 45 && dice < 60 {
            return Some(Powerup::new(location, PowerupType::PadSize, id));
        }
        else if dice > 60 && dice < 75 {
            return Some(Powerup::new(location, PowerupType::Confuse, id));
        }
        else if dice > 75 && dice < 90 {
            return Some(Powerup::new(location, PowerupType::Chaos, id));
        }

        None
    }

    pub fn fixed_tick(&mut self, dt : f32) {
        self.transform2d.position += self.velocity * dt;
    }

    pub fn get_renderable(&self) -> RenderJob {
        RenderJob::Sprite(self.transform2d.clone(), self.pu_type.get_sprite())
    }

    pub fn destroy(&mut self) {
        self.is_alive = false;
    }
}

impl BoxCollider2D for Powerup {
    fn worldpos(&self) -> Vector2<f32> {
        self.transform2d.position
    }

    fn size(&self) -> Vector2<f32> {
        self.transform2d.size
    }
}