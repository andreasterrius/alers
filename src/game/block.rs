pub struct Block {
    transform2d : Transform2D,
    renderable : Renderable2D,
    is_alive : bool //should render, and fisika tick
}

impl Block {
    pub fn new (transform2d : Transform2D,
                shader_key : String,
                texture_keys : Vec<String>,
                color : Vector3<f32>) -> Block {
        Block {
            transform2d,
            renderable : Renderable2D::new(
                color,
                shader_key,
                texture_keys
            ),
            is_alive : true
        }
    }

    pub fn destroy(&mut self) {
        self.is_alive = false;
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
}

impl BoxCollider2D for Block {
    fn get_world_position(&self) -> Vector2<f32> {
        self.transform2d.position
    }

    fn get_size(&self) -> Vector2<f32> {
        self.transform2d.size
    }
}