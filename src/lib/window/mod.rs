use glfw::{Context, Key, Action, WindowEvent};
use std::sync::mpsc::Receiver;

pub struct WindowCreator <'a> {
    glfw : &'a mut glfw::Glfw
}

impl <'a> WindowCreator<'a> {

    pub fn new_creator(glfw : &'a mut glfw::Glfw) -> WindowCreator <'a> {
        WindowCreator {
            glfw
        }
    }

    pub fn new(mut self, scr_width : u32, scr_height : u32) -> Window {

        self.glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        self.glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        #[cfg(target_os = "macos")] glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        // glfw window creation
        // --------------------
        let (mut glfw_window, glfw_events) = self.glfw.create_window(
            scr_width,
            scr_height,
            "LearnOpenGL",
            glfw::WindowMode::Windowed,
        ).expect("Failed to create GLFW window");

        glfw_window.make_current();
        glfw_window.set_key_polling(true);
        glfw_window.set_framebuffer_size_polling(true);

        // gl: load all OpenGL function pointers
        // ---------------------------------------
        gl::load_with(|symbol| glfw_window.get_proc_address(symbol) as *const _);

        Window {
            glfw_window,
            glfw_events
        }
    }
}

pub struct Window {
    glfw_window: glfw::Window,
    glfw_events: Receiver<(f64, WindowEvent)>
}

impl Window {
    pub fn is_closing(&self) -> bool {
        self.glfw_window.should_close()
    }

    pub fn swap_buffers(&mut self) { self.glfw_window.swap_buffers(); }

    pub fn handle_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.glfw_events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.glfw_window.set_should_close(true)
                },
                _ => {},
            }
        }
    }
}