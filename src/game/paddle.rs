pub struct Paddle  {
    transform2d : Transform2D,
    renderable : Renderable2D,

    speed : f32,
    moving_right : f32
}

impl Paddle {
    fn do_move(&mut self, dt : f32, input : f32){
        self.transform2d.position += Vector2::new(input * self.speed, 0.0);
    }
}

impl BoxCollider2D for Paddle {
    fn get_world_position(&self) -> Vector2<f32> {
        self.transform2d.position
    }

    fn get_size(&self) -> Vector2<f32> {
        self.transform2d.size
    }
}
