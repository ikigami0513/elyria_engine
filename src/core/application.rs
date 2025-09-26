use cgmath::{ortho, Vector3, Matrix4, vec3};
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
use crate::graphics::sprite::{SpriteCreator};
use crate::graphics::spritesheet::Spritesheet;
use crate::world::components::{Parent, TransformComponent};
use crate::world::system::{System, TransformSystem, SpriteRenderSystem};
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
    world: World,
    systems: Vec<Box<dyn System>>,
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

        let spritesheet = Spritesheet::new("resources/data/spritesheets/player_base.json").unwrap();

        let root_entity = world.new_entity();
        world.add_component(root_entity, TransformComponent::new());

        let container_entity = world.new_entity();
        let mut container_transform = TransformComponent::new();
        container_transform.transform.set_local_position(vec3(400.0, 300.0, 0.0));
        container_transform.transform.set_local_scale(vec3(0.1, 0.1, 0.1));

        world.add_component(container_entity, container_transform);
        world.add_component(container_entity, SpriteCreator::from_texture("resources/textures/container.jpg"));
        world.add_component(container_entity, Parent(root_entity));

        let player_entity = world.new_entity();
        let mut player_transform = TransformComponent::new();
        player_transform.transform.set_local_position(vec3(200., 200., 0.0));
        world.add_component(player_entity, player_transform);
        world.add_component(player_entity, SpriteCreator::from_sprite(&spritesheet, "idle_down_0").unwrap());

        let mut input = InputHandler::new();
        let (xpos, ypos) = window.get_cursor_pos();
        input.last_x = xpos as f32;
        input.last_y = ypos as f32;

        let mut systems: Vec<Box<dyn System>> = Vec::new();
        systems.push(Box::new(TransformSystem));
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

            let mut frame_context = FrameContext {
                time: &self.time,
                input: &self.input,
                world: &mut self.world
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
