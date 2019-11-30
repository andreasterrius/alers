#![allow(non_upper_case_globals)]
pub extern crate glfw;
pub extern crate gl;
pub extern crate image;
pub extern crate cgmath;
pub extern crate time;
pub extern crate rand;
pub extern crate rodio;
pub extern crate rusttype;
pub extern crate unicode_normalization;
pub extern crate fbxcel_dom;
pub extern crate log as lg;
pub extern crate simplelog;
pub extern crate approx;
pub extern crate snowflake;

#[macro_use]
pub mod macros;
pub mod log;
pub mod graphics;
pub mod fisika;
pub mod ale;
pub mod resource;
pub mod math;
pub mod audio;
pub mod engine;
pub mod window;
pub mod renderer;
pub mod data;

use self::glfw::{Context, Key, Action};
use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3, Vector2};
use std::sync::mpsc::Receiver;
use std::str;
use std::path::Path;
use std::time::Instant;
use graphics::opengl::OpenGLRenderer;
use ale::input::InputManager;
use ale::ticker::FixedStepTick;
use resource::ResourceManager;
use graphics::state::RenderState;
use ale::time::TimerManager;
use ale::idgen::TimestampIdGenerator;
use ale::scene::SceneLoader;
use audio::AudioManager;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

#[allow(non_snake_case)]
pub fn start_engine<SetupFn>(mut setup: SetupFn)
where
    SetupFn: FnMut(&mut SceneLoader, &mut TimestampIdGenerator, u32, u32),
{

    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")] glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw.create_window(
        SCR_WIDTH,
        SCR_HEIGHT,
        "LearnOpenGL",
        glfw::WindowMode::Windowed,
    ).expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut audio_manager = AudioManager::new();
    let mut idgen = TimestampIdGenerator::new();
    let mut resources = ResourceManager::new();
    let mut renderer = OpenGLRenderer::new(SCR_WIDTH, SCR_HEIGHT);
    let mut render_state = RenderState::new();
    let mut timer_manager = TimerManager::new();
    let mut scene_loader = SceneLoader::new();

    setup(&mut scene_loader, &mut idgen, SCR_WIDTH, SCR_HEIGHT);
    scene_loader.get_active_scene().load_resources(
        &mut resources,
    );
    scene_loader.get_active_scene().configure_renderer(
        &resources,
        &mut renderer,
    );
    scene_loader.get_active_scene().configure_audio(
        &resources,
        &mut audio_manager,
    );

    let mut ticker = FixedStepTick::new(0.01);
    let mut input = InputManager::new();

    while !window.should_close() {

        process_events(&mut window, &events, &mut input);

        //Deterministic physics with 0.01 dt
        let accumulator = ticker.tick(&mut |dt, is_last_tick| {
            timer_manager.fixed_tick(dt);
            scene_loader.get_active_scene().fixed_tick(
                dt,
                &input,
                &audio_manager,
                &mut timer_manager,
                &mut idgen,
            );

            if is_last_tick {
                render_state.last_frame = scene_loader.get_active_scene().get_renderables();
            }
        });
        render_state.current_frame = scene_loader.get_active_scene().get_renderables();

        renderer.render(
            render_state.lerp_frame(accumulator),
            &scene_loader.get_active_scene().get_postprocess_tick(),
        );

        window.swap_buffers();
        glfw.poll_events();
    }

    renderer.delete_buffers();
}


fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>, input: &mut InputManager) {
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
