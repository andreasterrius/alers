#![allow(non_upper_case_globals)]
extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
extern crate image;
extern crate cgmath;
extern crate time;
extern crate rand;
extern crate rodio;

mod game;
mod renderer;
mod fisika;
mod ale;
mod resource;
mod math;
mod audio;

use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3, Vector2};
use std::sync::mpsc::Receiver;
use std::str;
use std::path::Path;
use std::time::Instant;
use renderer::opengl::OpenGLRenderer;
use ale::input::Input;
use ale::ticker::FixedStepTick;
use resource::ResourceManager;
use renderer::state::RenderState;
use ale::time::TimerManager;
use ale::idgen::TimestampIdGenerator;
use audio::AudioPlayer;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

#[allow(non_snake_case)]
pub fn main() {

    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

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

    let mut audio_player = AudioPlayer::new();
    let mut idgen = TimestampIdGenerator::new();
    let mut resources = ResourceManager::new();
    let mut renderer = OpenGLRenderer::new(SCR_WIDTH, SCR_HEIGHT);
    let mut render_state = RenderState::new();
    let mut timer_factory = TimerManager::new();

    let mut ticker = FixedStepTick::new(0.01);

    let mut game = game::Game::new(SCR_WIDTH, SCR_HEIGHT, &mut idgen);
    game.load_resources(&mut resources);
    game.configure_renderer(&resources, &mut renderer);
    game.configure_audio(&resources, &mut audio_player);

    let mut input = Input::new();

    while !window.should_close() {

        process_events(&mut window, &events, &mut input);

        //Deterministic physics with 0.01 dt
        let accumulator = ticker.tick(&mut | dt, is_last_tick | {
            timer_factory.fixed_tick(dt);
            game.fixed_tick(dt, &input, &mut timer_factory, &audio_player, &mut idgen);

            if is_last_tick {
                render_state.last_frame = game.get_renderables();
            }
        });
        render_state.current_frame = game.get_renderables();

        renderer.render(render_state.lerp_frame(accumulator), &game.get_postprocess());

        window.swap_buffers();
        glfw.poll_events();
    }

    renderer.delete_buffers();
}


fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>, input : &mut Input){
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            glfw::WindowEvent::Key(key, _, action, _) => input.mutate_key(key, action),
            _ => {}
        }
    }
}