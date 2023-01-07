use ale_data::alevec::Key;
use ale_math::color::Color;
use ale_math::Vector2;
use ale_resources::texture::Texture;

pub enum RenderTask {
  StaticMesh(StaticMesh),
  Sprite(Sprite),
}

pub struct StaticMesh {}

pub struct Sprite {
  pub texture_sprite: Option<Key<Texture>>,
  pub color : Color,
  pub position : Vector2<f32>,
  pub size : Vector2<f32>,
}
