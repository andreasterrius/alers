use cgmath::{Vector2, Vector3, Matrix4};
use cgmath::prelude::*;

pub trait Lerpable {
    fn lerp(&self, other: &Self, t: f32) -> Self;
}

impl Lerpable for f32 {
    fn lerp(&self, other: &f32, t: f32) -> f32 {
        (1.0 - t) * self + t * other
    }
}

impl Lerpable for Vector2<f32> {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        let vec = Vector2::new(self.x.lerp(&other.x, t), self.y.lerp(&other.y, t));

        vec
    }
}

#[derive(Debug, Clone)]
pub struct Transform2D {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub depth: f32,
}

impl Transform2D {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>, depth: f32) -> Transform2D {
        Transform2D {
            position,
            size,
            depth,
        }
    }

    pub fn get_matrix(&self) -> Matrix4<f32> {
        let mut transform: Matrix4<f32> = Matrix4::identity();
        transform = transform *
            Matrix4::from_translation(Vector3::<f32>::new(
                self.position.x,
                self.position.y,
                self.depth,
            ));
        transform = transform * Matrix4::from_nonuniform_scale(self.size.x, self.size.y, 1.0);

        return transform;
    }
}

impl Lerpable for Transform2D {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Transform2D {
            position: self.position.lerp(other.position, t),
            size: self.size.lerp(other.size, t),
            depth: 0.0,
        }

    }
}

pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    return self::max(min, self::min(val, max));
}

pub fn max<T: PartialOrd>(val: T, max: T) -> T {
    return if val > max { val } else { max };
}

pub fn min<T: PartialOrd>(val: T, min: T) -> T {
    return if val < min { val } else { min };
}

pub fn abs_vec2(val: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(val.x.abs(), val.y.abs())
}
