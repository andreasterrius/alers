use alexyt::math::Transform2D;
use alexyt::renderer::job::SpriteRenderable;
use alexyt::cgmath::prelude::*;
use alexyt::cgmath::{Vector2, Vector3, Vector4};
use alexyt::fisika::BoxCollider2D;
use alexyt::ale::idgen::TimestampIdGenerator;

pub struct Block {
    pub id : i64,
    pub transform2d : Transform2D,
    pub sprite: SpriteRenderable,
    pub is_alive : bool, //should render, and fisika tick
    pub is_solid : bool
}

impl Block {

    pub fn new (id : i64,
                transform2d : Transform2D,
                shader_key : String,
                texture_keys : Vec<String>,
                color : Vector4<f32>,
                is_solid : bool) -> Block {
        Block {
            id,
            transform2d,
            sprite: SpriteRenderable::new(
                color,
                shader_key,
                None,
                texture_keys,
            ),
            is_alive : true,
            is_solid,
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

                if *index == 1 {
                    blocks.push(Block::new(
                        idgen.next(),
                        Transform2D {
                            position,
                            size,
                            depth: 0.0,
                        },
                        String::from("sprite"),
                        vec!(String::from("block")),
                        Vector4::new(1.0, 1.0, 1.0, 1.0),
                        false
                    ));
                } else {
                    blocks.push(Block::new(
                        idgen.next(),
                        Transform2D {
                            position,
                            size,
                            depth: 0.0,
                        },
                        String::from("sprite"),
                        vec!(String::from("block_solid")),
                        Vector4::new(1.0, 1.0, 1.0, 1.0),
                        true
                    ));
                }
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