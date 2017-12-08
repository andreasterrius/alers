use cgmath::{Vector2, Vector3, Matrix4};
use renderer::Renderable2D;
use fisika::FisikaObject;
use {Transform2D};

pub struct Block {
    transform2d : Transform2D,
    renderable : Renderable2D
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
            )
        }
    }
}

impl FisikaObject for Block {
    fn fisika_tick(&mut self, fixed_dt: f32) {
        //do nothing
    }

    fn on_collision(&mut self, fixed_dt: f32, other: &FisikaObject) {
        //TODO: destroy self
    }
}

pub struct Paddle  {
    transform2d : Transform2D,
    renderable : Renderable2D
}

impl FisikaObject for Paddle {
    fn fisika_tick(&mut self, fixed_dt: f32) {
        //do nothing
    }

    fn on_collision(&mut self, fixed_dt: f32, other: &FisikaObject) {
        //do nothing
    }
}

pub struct Ball  {
    transform2d : Transform2D,
    renderable : Renderable2D,
    velocity : Vector2<f32>,
    is_on_paddle : bool
}

impl FisikaObject for Ball {
    fn fisika_tick(&mut self, fixed_dt : f32) {

        self.transform2d.position += self.velocity * fixed_dt;

        if self.transform2d.position.x <= 0.0 {
            self.velocity.x = -self.velocity.x;
            self.transform2d.position.x = 0.0
        }
        else if self.transform2d.position.x + self.transform2d.size.x >= 800.0 {
            //TODO find a way to get arena height here
            self.velocity.x = - self.velocity.x;
            self.transform2d.position.x = 800.0 - self.transform2d.size.x;
        }

        if self.transform2d.position.y <= 0.0
        {
            self.velocity.y = -self.velocity.y;
            self.transform2d.position.y = 0.0;
        }
    }

    fn on_collision(&mut self, fixed_dt: f32, other: &FisikaObject) {
        //do nothing
    }
}

pub struct Game {
    blocks : Vec<Block>,
    paddle : Paddle,
    ball : Ball
}

impl Game  {
    pub fn new(arena_width : u32, arena_height : u32) -> Game  {
        let blocks = create_blocks(arena_width, arena_height);
        let paddle = create_paddle(arena_width, arena_height);
        let ball = create_ball(arena_width, arena_height, &paddle);

        Game {
            blocks,
            paddle,
            ball
        }
    }

    pub fn get_renderables_2d(&self) -> Vec<(Matrix4<f32>, Renderable2D)> {
        let mut renderables = vec!();
        for block in &self.blocks {
            renderables.push((block.transform2d.get_matrix(), block.renderable.clone()));
        }

        renderables.push((self.paddle.transform2d.get_matrix(), self.paddle.renderable.clone()));
        renderables.push((self.ball.transform2d.get_matrix(), self.ball.renderable.clone()));

        renderables
    }

    pub fn get_fisika_objects(&mut self) -> Vec<&mut FisikaObject> {
        let mut objects : Vec<&mut FisikaObject> = vec!();
        for block in &mut self.blocks {
            objects.push(block);
        }

        objects.push(&mut self.paddle);
        objects.push(&mut self.ball);

        objects
    }
}

fn create_blocks(arena_width : u32, arena_height : u32) -> Vec<Block> {

    let arena_data : Vec<Vec<i32>> = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let mut blocks : Vec<Block> = vec!();

    for i in 0..arena_data.len(){
        let row = &arena_data[i];
        for j in 0..row.len(){
            let index = &row[j];

            let position = Vector2::<f32>::new(
                j as f32 * arena_width as f32 / row.len() as f32,
                i as f32 * arena_height as f32 / arena_data.len() as f32 / 2.0
            );
            let size = Vector2::<f32>::new(
                arena_width as f32 / row.len() as f32,
                arena_height as f32 / arena_data.len() as f32 / 2.0
            );

            blocks.push(Block::new(
                Transform2D {
                    position, size
                },
                String::from("sprite"),
                vec!(String::from("block")),
                Vector3::new(1.0, 1.0, 1.0)
            ));
        }
    }

    blocks
}

fn create_paddle(arena_width : u32, arena_height : u32) -> Paddle  {

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
        transform2d,
        renderable: Renderable2D::new(
            Vector3::new(1.0, 1.0, 1.0),
            String::from("sprite"),
            vec!(String::from("paddle")),
        )
    }
}

fn create_ball(arena_width : u32, arena_height : u32, paddle : &Paddle) -> Ball  {
    let size = Vector2::new(30.0, 30.0);
    let position = Vector2::new(
        arena_width as f32 / 2.0 - size.x / 2.0,
        arena_height as f32 - size.y - paddle.transform2d.size.y
    );

    let transform2d = Transform2D {
        position,
        size
    };

    Ball {
        transform2d,
        renderable: Renderable2D::new(
            Vector3::new(1.0, 1.0, 1.0),
            String::from("sprite"),
            vec!(String::from("ball")),
        ),
        velocity: Vector2::new(500.0, -500.0),
        is_on_paddle : false
    }
}