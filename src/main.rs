#[link(name = "shell32")]
unsafe extern "C" {}

mod shader;
mod buffer;
mod texture;
mod camera;
mod r#macro;

use cgmath::{perspective, Deg, Point3, Vector3};
use cgmath::prelude::*;
use glfw::{Context, Key, Action};
use gl::types::*;

use std::sync::mpsc::Receiver;
use std::os::raw::c_void;

use shader::Shader;
use camera::{Camera, CameraMovement};
use crate::buffer::{VertexArray, VertexBuffer};
use crate::texture::Texture;
use cgmath::{Matrix4, vec3};

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct Application {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    camera: Camera,
    first_mouse: bool,
    last_x: f32,
    last_y: f32,
    delta_time: f32,
    last_frame: f32,
    shader: Shader,
    vao: VertexArray,
    vbo: VertexBuffer,
    texture: Texture,
    cube_positions: [Vector3<f32>; 10]
}

impl Application {
    fn new() -> Self {
        // glfw: initialize and configure
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os="macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // glfw window creation
        let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "Elyria", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Disabled);

        // gl: load all OpenGL function pointers
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        // Shader and OpenGL setup
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        let vertices: [f32; 180] = [
            -0.5, -0.5, -0.5, 0.0, 0.0,
            0.5, -0.5, -0.5, 1.0, 0.0,
            0.5, 0.5, -0.5, 1.0, 1.0,
            0.5, 0.5, -0.5, 1.0, 1.0,
            -0.5, 0.5, -0.5, 0.0, 1.0,
            -0.5, -0.5, -0.5, 0.0, 0.0,

            -0.5, -0.5, 0.5, 0.0, 0.0,
            0.5, -0.5, 0.5, 1.0, 0.0,
            0.5, 0.5, 0.5, 1.0, 1.0,
            0.5, 0.5, 0.5, 1.0, 1.0,
            -0.5, 0.5, 0.5, 0.0, 1.0,
            -0.5, -0.5, 0.5, 0.0, 0.0,

            -0.5, 0.5, 0.5, 1.0, 0.0,
            -0.5, 0.5, -0.5, 1.0, 1.0,
            -0.5, -0.5, -0.5, 0.0, 1.0,
            -0.5, -0.5, -0.5, 0.0, 1.0,
            -0.5, -0.5, 0.5, 0.0, 0.0,
            -0.5, 0.5, 0.5, 1.0, 0.0,

            0.5, 0.5, 0.5, 1.0, 0.0,
            0.5, 0.5, -0.5, 1.0, 1.0,
            0.5, -0.5, -0.5, 0.0, 1.0,
            0.5, -0.5, -0.5, 0.0, 1.0,
            0.5, -0.5, 0.5, 0.0, 0.0,
            0.5, 0.5, 0.5, 1.0, 0.0,

            -0.5, -0.5, -0.5, 0.0, 1.0,
            0.5, -0.5, -0.5, 1.0, 1.0,
            0.5, -0.5, 0.5, 1.0, 0.0,
            0.5, -0.5, 0.5, 1.0, 0.0,
            -0.5, -0.5, 0.5, 0.0, 0.0,
            -0.5, -0.5, -0.5, 0.0, 1.0,

            -0.5, 0.5, -0.5, 0.0, 1.0,
            0.5, 0.5, -0.5, 1.0, 1.0,
            0.5, 0.5, 0.5, 1.0, 0.0,
            0.5, 0.5, 0.5, 1.0, 0.0,
            -0.5, 0.5, 0.5, 0.0, 0.0,
            -0.5, 0.5, -0.5, 0.0, 1.0
        ];
        
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

        let shader = Shader::new("shaders/shader.vs", "shaders/shader.fs");
        let vao = VertexArray::new();
        let vbo = VertexBuffer::new();

        vao.bind();
        vbo.bind();
        vbo.set_data(&vertices);

        let stride = 5 * std::mem::size_of::<GLfloat>() as GLsizei;

        // position attribute
        vao.set_attribute(0, 3, gl::FLOAT, stride, std::ptr::null());

        // texture coord attribute
        vao.set_attribute(1, 2, gl::FLOAT, stride, (3 * std::mem::size_of::<GLfloat>()) as *const c_void);

        vbo.unbind();
        vao.unbind();

        let texture = Texture::new("resources/textures/container.jpg");

        unsafe {
            shader.use_program();
            shader.set_int(c_str!("texture1"), 0);
        }

        Self {
            glfw,
            window,
            events,
            camera: Camera { position: Point3::new(0.0, 0.0, 3.0), ..Camera::default()},
            first_mouse: true,
            last_x: SCR_WIDTH as f32 / 2.0,
            last_y: SCR_HEIGHT as f32 / 2.0,
            delta_time: 0.0,
            last_frame: 0.0,
            shader,
            vao,
            vbo,
            texture,
            cube_positions,
        }
    }

    fn run(&mut self) {
        while !self.window.should_close() {
            // Per-frame time logic
            let current_frame = self.glfw.get_time() as f32;
            self.delta_time = current_frame - self.last_frame;
            self.last_frame = current_frame;

            // Events and input
            self.process_events();
            self.process_input();

            // Render
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                self.texture.bind();
                self.shader.use_program();

                let projection: Matrix4<f32> = perspective(Deg(self.camera.zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
                self.shader.set_mat4(c_str!("projection"), &projection);

                let view = self.camera.get_view_matrix();
                self.shader.set_mat4(c_str!("view"), &view);

                self.vao.bind();
                for (i, position) in self.cube_positions.iter().enumerate() {
                    let mut model: Matrix4<f32> = Matrix4::from_translation(*position);
                    let angle = 20.0 * i as f32;
                    model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));
                    self.shader.set_mat4(c_str!("model"), &model);

                    gl::DrawArrays(gl::TRIANGLES, 0, 36);
                }
            }

            // glfw: swap buffers and poll IO events
            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    let (xpos, ypos) = (xpos as f32, ypos as f32);
                    if self.first_mouse {
                        self.last_x = xpos;
                        self.last_y = ypos;
                        self.first_mouse = false;
                    }

                    let xoffset = xpos - self.last_x;
                    let yoffset = self.last_y - ypos; // reversed since y-coordinates go from bottom to top

                    self.last_x = xpos;
                    self.last_y = ypos;

                    self.camera.process_mouse_movement(xoffset, yoffset, true);
                }
                glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                    self.camera.process_mouse_scroll(yoffset as f32);
                }
                _ => {}
            }
        }
    }

    fn process_input(&mut self) {
        if self.window.get_key(Key::Escape) == Action::Press {
            self.window.set_should_close(true)
        }

        if self.window.get_key(Key::W) == Action::Press {
            self.camera.process_keyboard(CameraMovement::FORWARD, self.delta_time);
        }
        if self.window.get_key(Key::S) == Action::Press {
            self.camera.process_keyboard(CameraMovement::BACKWARD, self.delta_time);
        }
        if self.window.get_key(Key::A) == Action::Press {
            self.camera.process_keyboard(CameraMovement::LEFT, self.delta_time);
        }
        if self.window.get_key(Key::D) == Action::Press {
            self.camera.process_keyboard(CameraMovement::RIGHT, self.delta_time);
        }
    }
}

fn main() {
    let mut app = Application::new();
    app.run();
}
