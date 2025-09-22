#[link(name = "shell32")]
unsafe extern "C" {}

mod shader;
mod buffer;
mod texture;

use glfw::{Context, Key, Action};
use gl::types::*;

use std::ptr;
use std::sync::mpsc::Receiver;
use std::os::raw::c_void;

use shader::Shader;

use crate::buffer::ElementBuffer;
use crate::buffer::VertexArray;
use crate::buffer::VertexBuffer;
use crate::texture::Texture;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
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
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let vertices: [f32; 32] = [
        // positions       // colors        // texture coords
         0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
         0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
        -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left
    ];

    let indices = [
        0, 1, 3,  // first Triangle
        1, 2, 3   // second Triangle
    ];

    let shader =  Shader::new("shaders/shader.vs", "shaders/shader.fs");
    let vao = VertexArray::new();
    let vbo = VertexBuffer::new();
    let ebo = ElementBuffer::new();

    vao.bind();

    vbo.bind();
    vbo.set_data(&vertices);

    ebo.bind();
    ebo.set_data(&indices);

    let stride = 8 * std::mem::size_of::<GLfloat>() as GLsizei;

    // position attribute
    vao.set_attribute(0, 3, gl::FLOAT, stride, std::ptr::null());

    // color attribute
    vao.set_attribute(1, 3, gl::FLOAT, stride, (3 * std::mem::size_of::<GLfloat>()) as *const c_void);

    // texture coord attribute
    vao.set_attribute(2, 2, gl::FLOAT, stride, (6 * std::mem::size_of::<GLfloat>()) as *const c_void);

    vbo.unbind();
    vao.unbind();
    ebo.unbind();

    let texture = Texture::new("resources/textures/container.jpg");

    // render loop
    while !window.should_close() {
        // events
        process_events(&mut window, &events);

        // render
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            texture.bind();
            
            // render the triangle
            shader.use_program();
            vao.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        window.swap_buffers();
        glfw.poll_events();
    }
}

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