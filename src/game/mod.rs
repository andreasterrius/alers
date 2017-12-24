use cgmath::{Vector2, Vector3, Matrix4};
use renderer::Renderable2D;
use {Transform2D};
use fisika::{self, BoxCollider2D, CircleCollider2D};
use ale::input::Input;
use glfw::{Key, Action};
use resource::*;
use renderer::opengl::*;

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

    pub fn load_resources(&self, resources : &mut ResourceManager) {
        resources.load_glsl(
            "sprite",
            "shaders/sprite.vs",
            "shaders/sprite.fs"
        );
        resources.load_image("ball", "resources/ball.png");
        resources.load_image("block", "resources/block.png");
        resources.load_image("paddle", "resources/paddle.png");
    }

    pub fn configure_renderer(&self, resources : &ResourceManager, renderer: &mut OpenGLRenderer){
        let sprite_shader = resources.get_glsl("sprite").unwrap();
        renderer.register_shader(
            "sprite",
            &sprite_shader.vertex_shader,
            &sprite_shader.fragment_shader
        );
        renderer.register_image("ball", &resources.get_image("ball").unwrap().image);
        renderer.register_image("block", &resources.get_image("block").unwrap().image);
        renderer.register_image("paddle", &resources.get_image("paddle").unwrap().image);
    }

    pub fn get_renderables_2d(&self) -> Vec<(Matrix4<f32>, Renderable2D)> {
        let mut renderables = vec!();
        for block in &self.blocks {
            if !block.is_alive { continue; }
            renderables.push((block.transform2d.get_matrix(), block.renderable.clone()));
        }

        renderables.push((self.paddle.transform2d.get_matrix(), self.paddle.renderable.clone()));
        renderables.push((self.ball.transform2d.get_matrix(), self.ball.renderable.clone()));

        renderables
    }

    pub fn fixed_tick(&mut self, dt : f32, input : &Input) {
        self.ball.do_move(dt);

        let move_right = {
            let mut kk = 0.0;

            kk += input.get_key(&Key::Right)
                .map_or(0.0, | action | {
                    match *action {
                        Action::Press => 1.0,
                        Action::Repeat => 1.0,
                        _ => 0.0
                    }
                });
            kk += input.get_key(&Key::Left)
                .map_or(0.0, | action | {
                    match *action {
                        Action::Press => -1.0,
                        Action::Repeat => -1.0,
                        _ => 0.0
                    }
                });

            kk
        };
        self.paddle.do_move(dt, move_right);

        for block in &mut self.blocks {
            if !block.is_alive() { continue }

            if fisika::aabb_collision_box_box(&self.ball, block)
            {
                block.destroy();
            }
        }

        if fisika::aabb_collision_box_box(&self.ball, &self.paddle) {

            //On the first first collision tick
            if !self.ball.is_colliding {

                //evaluate the position
                let ball_position = self.ball.get_world_position();
                let top = self.paddle.get_world_position().y;
                let bottom = self.paddle.get_world_position().y + self.paddle.get_size().y;
                let left = self.paddle.get_world_position().x;
                let right = self.paddle.get_world_position().x + self.paddle.get_size().x;

                //Upper right
                if ball_position.x > right {
                    println!("upper right");
                    self.ball.bounce(true, false);
                }
                else if ball_position.x < left {
                    println!("upper left");
                    self.ball.bounce(true, false);
                }
                else {
                    println!("upper");
                    self.ball.bounce(false, false);
                }

                self.ball.bounce(false, true);
                self.ball.is_colliding = true;
            }
        }
        else {
            self.ball.is_colliding = false;
        }
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
        ),
        speed : 10.0,
        moving_right: 0.0,
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
        is_on_paddle : false,
        is_colliding : false
    }
}