#[link(name = "shell32")]
unsafe extern "C" {}

mod glutils;
mod core;
mod camera;
mod math;
mod world;
mod graphics;

use core::application::Application;

fn main() {
    let mut app = Application::new();
    app.run();
}
