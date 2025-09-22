use cgmath::{perspective, Deg, Point3, Vector3};
use glfw::{Context, Key};

use std::sync::mpsc::Receiver;

use crate::c_str;
use crate::core::input::InputHandler;
use crate::core::time::Time;
use crate::glutils::{
    shader::Shader
};

use crate::core::frame_context::FrameContext;
use crate::camera::Camera;
use crate::graphics::cuboid_renderer::CuboidRenderer;
use crate::world::entity::Entity;
use crate::world::scene::Scene;
use cgmath::{Matrix4, vec3};

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub struct Application {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    camera: Camera,
    time: Time,
    shader: Shader,
    scene: Scene,
    input: InputHandler
}

impl Application {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        // glfw: initialize and configure
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os="macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // glfw window creation
        let (mut window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        let (xpos, ypos) = glfw.with_primary_monitor(|_glfw, monitor_opt| {
            if let Some(monitor) = monitor_opt {
                if let Some(video_mode) = monitor.get_video_mode() {
                    let xpos = (video_mode.width - width as u32) / 2;
                    let ypos = (video_mode.height - height as u32) / 2;
                    return (xpos, ypos);
                }
            }

            (100, 100)
        });

        window.set_pos(xpos as i32, ypos as i32);

        window.make_current();
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Disabled);
        window.set_key_polling(true);

        // gl: load all OpenGL function pointers
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        // Shader and OpenGL setup
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        let shader = Shader::new("shaders/shader.vs", "shaders/shader.fs");
        unsafe {
            shader.use_program();
            shader.set_int(c_str!("texture1"), 0);
        }
        
        let scene = Scene::new();
        let root = scene.get_root();
        
        let cube_positions: [Vector3<f32>; 10] = [
            vec3(0.0, 0.0, 0.0),
            vec3(2.0, 5.0, -15.0),
            vec3(-1.5, -2.2, -2.5),
            vec3(-3.8, -2.0, -12.3),
            vec3(2.4, -0.4, -3.5),
            vec3(-1.7, 3.0, -7.5),
            vec3(1.3, -2.0, -2.5),
            vec3(1.5, 2.0, -2.5),
            vec3(1.5, 0.2, -1.5),
            vec3(-1.3, 1.0, -1.5)
        ];

        let mut root_borrow = root.borrow_mut();
        for (i, position) in cube_positions.iter().enumerate() {
            let child = Entity::new();
            let child_rc = std::rc::Rc::new(std::cell::RefCell::new(child));

            let angle = 20.0 * i as f32;
            child_rc.borrow_mut().transform.set_local_rotation(vec3(angle, angle, angle));

            child_rc.borrow_mut().parent = Some(std::rc::Rc::downgrade(&root));
            child_rc.borrow_mut().add_component(CuboidRenderer::new());
            child_rc.borrow_mut().transform.set_local_position(*position);
            
            root_borrow.children.push_back(child_rc);
        }

        let mut input = InputHandler::new();
        let (xpos, ypos) = window.get_cursor_pos();
        input.last_x = xpos as f32;
        input.last_y = ypos as f32;

        Self {
            glfw,
            window,
            events,
            camera: Camera { position: Point3::new(0.0, 0.0, 3.0), ..Camera::default()},
            time: Time::new(),
            shader,
            scene,
            input
        }
    }

    pub fn run(&mut self) {
        while !self.window.should_close() {
            // Per-frame time logic
            self.time.update(self.glfw.get_time());

            // Events and input
            self.process_events();
            self.process_input();

            let frame_context = FrameContext {
                time: &self.time,
                input: &self.input
            };

            self.camera.update(&frame_context);
            self.scene.update(&frame_context);
            self.input.end_frame();

            // Render
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                self.shader.use_program();

                let projection: Matrix4<f32> = perspective(Deg(self.camera.zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
                self.shader.set_mat4(c_str!("projection"), &projection);

                let view = self.camera.get_view_matrix();
                self.shader.set_mat4(c_str!("view"), &view);

                self.scene.render(&self.shader);
            }

            // glfw: swap buffers and poll IO events
            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    fn process_events(&mut self) {
        self.input.reset_scroll_delta();

        for (_, event) in glfw::flush_messages(&self.events) {
            self.input.update(&event);
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height) }
                },
                _ => {}
            }
        }
    }

    fn process_input(&mut self) {
        if self.input.is_key_pressed(Key::Escape) {
            self.window.set_should_close(true)
        }
    }
}
