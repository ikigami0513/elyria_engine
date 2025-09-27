#[link(name = "shell32")]
unsafe extern "C" {}

use engine::core::application::Application;

fn main() {
    let mut app = Application::new(1920, 1200, "Elyria");
    app.run();
}
