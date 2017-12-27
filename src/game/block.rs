use math::Transform2D;
use renderer::job::SpriteRenderable;
use cgmath::prelude::*;
use cgmath::{Vector2, Vector3, Vector4};
use fisika::BoxCollider2D;
use ale::idgen::TimestampIdGenerator;

pub struct Block {
    pub id : i64,
    pub transform2d : Transform2D,
    pub sprite: SpriteRenderable,
    pub is_alive : bool //should render, and fisika tick
}

impl Block {

    pub fn new (id : i64,
                transform2d : Transform2D,
                shader_key : String,
                texture_keys : Vec<String>,
                color : Vector4<f32>) -> Block {
        Block {
            id,
            transform2d,
            sprite: SpriteRenderable::new(
                color,
                shader_key,
                texture_keys
            ),
            is_alive : true
        }
    }

    pub fn arena(arena_width : u32, arena_height : u32, idgen : &mut TimestampIdGenerator) -> Vec<Block> {
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
                    idgen.next(),
                    Transform2D {
                        position, size
                    },
                    String::from("sprite"),
                    vec!(String::from("block")),
                    Vector4::new(1.0, 1.0, 1.0, 1.0)
                ));
            }
        }

        blocks
    }

    pub fn destroy(&mut self) {
        self.is_alive = false;
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
}

impl BoxCollider2D for Block {
    fn worldpos(&self) -> Vector2<f32> {
        self.transform2d.position
    }

    fn size(&self) -> Vector2<f32> {
        self.transform2d.size
    }
}