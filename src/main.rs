#![allow(non_upper_case_globals)]
extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
extern crate image;
extern crate cgmath;

mod game;
mod renderer;
mod fisika;
mod input;
mod ale;

use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3, Vector2};
use std::sync::mpsc::Receiver;
use std::str;
use std::path::Path;
use std::time::Instant;
use fisika::{FixedFisikaTicker};
use renderer::opengl::OpenGLRenderer;
use ale::WorldStateManager;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub struct Transform2D {
    position : Vector2<f32>,
    size : Vector2<f32>
}

impl Transform2D {
    fn get_matrix(&self) -> Matrix4<f32> {
        let mut transform : Matrix4<f32> = Matrix4::identity();
        transform = transform * Matrix4::from_translation(Vector3::<f32>::new(self.position.x, self.position.y, 0.0));
        transform = transform * Matrix4::from_nonuniform_scale(self.size.x, self.size.y, 1.0);

        return transform;
    }
}

#[allow(non_snake_case)]
pub fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let vertexShaderSource: &str = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec2 aTexCoord;

        uniform mat4 projection;
        uniform mat4 model;

        out vec2 TexCoord;

        void main() {
           gl_Position = projection * model * vec4(aPos.x, aPos.y, aPos.z, 1.0);
           TexCoord = vec2(aTexCoord.x, aTexCoord.y);
        }
    "#;

    let fragmentShaderSource: &str = r#"
        #version 330 core
        out vec4 FragColor;
        in vec2 TexCoord;

        uniform vec3 color;
        uniform sampler2D texture0;
        uniform sampler2D texture1;

        void main() {
           FragColor = vec4(color, 1.0) * texture(texture0, TexCoord);
        }
    "#;

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw.create_window(SCR_WIDTH,
                                                  SCR_HEIGHT,
                                                  "LearnOpenGL",
                                                  glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut renderer= OpenGLRenderer::new(SCR_WIDTH, SCR_HEIGHT);

    renderer.create_shader("sprite", vertexShaderSource,
        fragmentShaderSource);

    let ball_img = image::open(&Path::new("resources/ball.png"))
        .expect("Failed to load ball image");
    let block_img = image::open(&Path::new("resources/block.png"))
        .expect("Failed to load block image");
    let paddle_img = image::open(&Path::new("resources/paddle.png"))
        .expect("Failed to load block image");

    renderer.create_texture( "ball", ball_img);
    renderer.create_texture( "block", block_img);
    renderer.create_texture( "paddle", paddle_img);

    let mut world_state = WorldStateManager::new();
    let mut ticker = FixedFisikaTicker::new(0.01);
    let mut game = game::Game::new(SCR_WIDTH, SCR_HEIGHT);

    while !window.should_close() {

        process_events(&mut window, &events);

        let accumulator = ticker.fisika_tick(|dt| {

        });

        //TODO: world_state.interpolate(accumulator)

        renderer.clear();
        renderer.render_sprites(game.get_renderables_2d());

        window.swap_buffers();
        glfw.poll_events();
    }

    renderer.delete_buffers();
}

// NOTE: not the same version as in common.rs!
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}