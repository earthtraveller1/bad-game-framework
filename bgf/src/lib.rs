pub mod graphics;
pub mod math;
pub mod ui;

use glad_gl::gl;
use glfw::Context;
use graphics::Renderer2D;
use std::sync::mpsc::Receiver;

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::Window,
    _events: Receiver<(f64, glfw::WindowEvent)>,
}

extern "system" fn opengl_debug_callback(
    _source: u32,
    _type: u32,
    _id: u32,
    _severity: u32,
    _length: i32,
    message: *const i8,
    _: *mut std::ffi::c_void,
) {
    let message = unsafe { std::ffi::CStr::from_ptr(message) }
        .to_bytes()
        .to_vec();

    println!("[OPENGL]: {}", String::from_utf8(message).unwrap());
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW!");

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::Visible(false));
        glfw.window_hint(glfw::WindowHint::Resizable(false));

        glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create the Window!");

        window.make_current();
        window.set_key_polling(true);

        gl::load(|procname| glfw.get_proc_address_raw(procname));

        unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);

            gl::DebugMessageCallback(opengl_debug_callback, std::ptr::null());

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        glfw.with_primary_monitor(|_, m| {
            let video_mode = m.unwrap().get_video_mode().unwrap();

            let new_x: u32 = (video_mode.width - width) / 2;
            let new_y: u32 = (video_mode.height - height) / 2;

            window.set_pos(new_x.try_into().unwrap(), new_y.try_into().unwrap());
        });

        return Window {
            glfw,
            window,
            _events: events,
        };
    }
    
    pub fn build_renderer(&self, max_quads: u32) -> Renderer2D {
        let (width, height) = self.window.get_size();
        Renderer2D::new(max_quads.try_into().unwrap(), width as f32, height as f32)
    }

    pub fn show(&mut self) {
        self.window.show();
    }

    pub fn is_open(&self) -> bool {
        return !(self.window.should_close());
    }

    pub fn update(&mut self) {
        self.window.swap_buffers();
        self.glfw.poll_events();
    }

    pub fn get_time(&self) -> f64 {
        self.glfw.get_time()
    }

    pub fn is_key_down(&self, key: glfw::Key) -> bool {
        let action = self.window.get_key(key);
        return action == glfw::Action::Press || action == glfw::Action::Repeat;
    }

    pub fn is_mouse_button_down(&self, button: glfw::MouseButton) -> bool {
        let action = self.window.get_mouse_button(button);
        return action == glfw::Action::Press || action == glfw::Action::Repeat;
    }

    pub fn get_mouse_position(&self) -> (f64, f64) {
        self.window.get_cursor_pos()
    }
}

pub struct SceneManager {
    active_scene: Box<dyn Scene>,
    next_scene: Option<Box<dyn Scene>>,
}

impl SceneManager {
    pub fn new(initial_scene: Box<dyn Scene>) -> SceneManager {
        return SceneManager {
            active_scene: initial_scene,
            next_scene: None,
        };
    }

    pub fn set_active(&mut self, new_active_scene: Box<dyn Scene>) {
        self.active_scene = new_active_scene;
    }

    pub fn update_active(&mut self, delta_time: f64) {
        self.next_scene = self.active_scene.update(delta_time);
    }

    pub fn render_active(&mut self) {
        self.active_scene.render();
    }

    pub fn handle_next_scene(&mut self) {
        if let Some(next_scene) = self.next_scene.take() {
            self.active_scene = next_scene;
        }
    }
}

pub trait Scene {
    fn update(&mut self, delta_time: f64) -> Option<Box<dyn Scene>>;

    fn render(&mut self);
}
