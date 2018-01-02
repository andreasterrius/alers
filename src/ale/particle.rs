use cgmath::{Vector2, Vector3, Vector4};
use cgmath::prelude::*;
use renderer::job::{RenderJob, ParticleRenderable, SpriteRenderable};
use ale::idgen::TimestampIdGenerator;
use std::collections::{HashMap, BTreeMap};
use math::*;
use rand;
use rand::distributions::{IndependentSample, Range};

pub struct ParticleEmitter {
    pub id : i64,
    pub last_used_particle : i32,
    pub particles : Vec<Particle>,
    pub particle_life : f32,

    pub sprite_renderable : SpriteRenderable
}

impl ParticleEmitter {
    pub fn new(count : i32, life : f32, idgen : &mut TimestampIdGenerator) -> ParticleEmitter {

        let mut particles = vec!();
        for i in 0..count {
            particles.push(Particle{
                id: idgen.next(),
                transform2d: Transform2D {
                    position : Vector2::from_value(0.0),
                    size : Vector2::from_value(10.0),
                    depth: 0.0,
                },
                velocity: Vector2::from_value(0.0),
                life: 0.0,
                color: Vector4::from_value(0.0),
            });
        }

        ParticleEmitter {
            last_used_particle: 0,
            particles,
            particle_life: life,
            id : idgen.next(),
            sprite_renderable: SpriteRenderable{
                color: Vector4::from_value(1.0),
                custom_shader_key: String::from("particle"),
                custom_shader_uniform : None,
                texture_keys: vec![String::from("ballparticle")],
            }
        }
    }

    pub fn get_renderables(&self) -> BTreeMap<i64, RenderJob> {
        let mut renderables = BTreeMap::new();
        for particle in &self.particles {
            renderables.insert(particle.id, RenderJob::Particle(particle.transform2d.clone(),
                                                                particle.get_renderable(),
                                                                self.sprite_renderable.clone()));
        }

        renderables
    }

    pub fn fixed_tick(&mut self, position : Vector2<f32>, dt : f32){

        let unused_particle_idx =
            if self.last_used_particle >= (self.particles.len()-1) as i32 {
                self.last_used_particle = 0;
                self.last_used_particle as usize
            } else {
                self.last_used_particle += 1;
                self.last_used_particle as usize
            };

        let id = self.particles[unused_particle_idx].id;

        let mut rng = rand::thread_rng();
        let between = Range::new(-50.0, 50.0);

        self.particles[unused_particle_idx] = Particle{
            id,
            transform2d : Transform2D {
                position : Vector2::new(position.x - 5.0, position.y - 5.0),
                size: Vector2::from_value(10.0),
                depth: 0.0,
            },
            velocity: Vector2::new(between.ind_sample(&mut rng), between.ind_sample(&mut rng)),
            life: 1.0,
            color: Vector4::from_value(1.0),
        };

        for particle in &mut self.particles {
            particle.life -= dt;
            if particle.life > 0.0 {
                particle.transform2d.position += particle.velocity * dt;
                particle.color.w -= dt * 2.5;
            }
        }
    }
}

#[derive(Debug)]
pub struct Particle {
    pub id : i64,
    pub transform2d : Transform2D,
    pub velocity : Vector2<f32>,
    pub life : f32,
    pub color : Vector4<f32>
}

impl Particle {
    fn get_renderable(&self) -> ParticleRenderable {
        ParticleRenderable {
            life : self.life
        }
    }
}
