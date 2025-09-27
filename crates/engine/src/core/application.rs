use cgmath::{ortho, Vector3, Matrix4};
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
use crate::graphics::animation::AnimationComponent;
use crate::graphics::managers::{AnimationManager, SpritesheetManager};
use crate::graphics::sprite::SpriteRendererComponent;
use crate::world::components::{Parent, TransformComponent};
use crate::world::system::{AnimationSystem, SpriteRenderSystem, System, TransformSystem};
use crate::world::world::World;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub struct Application {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    camera: Camera,
    time: Time,
    shader: Shader,
    pub world: World,
    systems: Vec<Box<dyn System>>,
    input: InputHandler,
    pub spritesheet_manager: SpritesheetManager,
    pub animation_manager: AnimationManager
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
        // window.set_cursor_mode(glfw::CursorMode::Disabled);
        window.set_key_polling(true);

        // gl: load all OpenGL function pointers
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        // Shader and OpenGL setup
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        let shader = Shader::new("shaders/shader.vs", "shaders/shader.fs");
        
        let mut world = World::new();

        world.register_component::<Parent>();
        world.register_component::<TransformComponent>();
        world.register_component::<SpriteRendererComponent>();
        world.register_component::<AnimationComponent>();

        let spritesheet_manager = SpritesheetManager::new();
        let animation_manager = AnimationManager::new();

        let mut input = InputHandler::new();
        let (xpos, ypos) = window.get_cursor_pos();
        input.last_x = xpos as f32;
        input.last_y = ypos as f32;

        let mut systems: Vec<Box<dyn System>> = Vec::new();
        systems.push(Box::new(TransformSystem));
        systems.push(Box::new(AnimationSystem));
        systems.push(Box::new(SpriteRenderSystem));

        Self {
            glfw,
            window,
            events,
            camera: Camera { position: Vector3::new(0.0, 0.0, 3.0), ..Camera::default()},
            time: Time::new(),
            shader,
            world,
            systems,
            input,
            spritesheet_manager,
            animation_manager
        }
    }

    pub fn run(&mut self) {
        while !self.window.should_close() {
            // Per-frame time logic
            self.time.update(self.glfw.get_time());

            // Events and input
            self.process_events();
            self.process_input();

            let mut frame_context = FrameContext {
                time: &self.time,
                input: &self.input,
                world: &mut self.world,
                spritesheet_manager: &mut self.spritesheet_manager,
                animation_manager: &mut self.animation_manager
            };

            self.camera.update(&frame_context);

            for system in self.systems.iter_mut() {
                system.update(&mut frame_context);
            }

            // Render
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                self.shader.use_program();

                let projection: Matrix4<f32> = ortho(0.0, SCR_WIDTH as f32, 0.0, SCR_HEIGHT as f32, -1.0, 1.0);
                self.shader.set_mat4(c_str!("projection"), &projection);

                let view = self.camera.get_view_matrix();
                self.shader.set_mat4(c_str!("view"), &view);

                for system in self.systems.iter_mut() {
                    system.render(&mut frame_context, &self.shader);
                }
            }

            self.input.end_frame();

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
