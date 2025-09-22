#[link(name = "shell32")]
unsafe extern "C" {}

mod glutils;
mod core;
mod camera;

use core::application::Application;

fn main() {
    let mut app = Application::new();
    app.run();
}
