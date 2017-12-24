pub struct Ball  {
    transform2d : Transform2D,
    renderable : Renderable2D,
    velocity : Vector2<f32>,
    is_on_paddle : bool,
    is_colliding : bool
}

impl Ball {
    fn do_move(&mut self, dt : f32) {

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

    fn bounce(&mut self, flip_x : bool, flip_y : bool){

        if flip_x { self.velocity.x *= -1.0 }
        if flip_y { self.velocity.y *= -1.0 }
    }
}

impl BoxCollider2D for Ball {
    fn get_world_position(&self) -> Vector2<f32> {
        self.transform2d.position
    }

    fn get_size(&self) -> Vector2<f32> {
        self.transform2d.size
    }
}