use starry_client::{
    base::{color::Color, renderer::Renderer},
    window::Window,
};

const SCREEN_WIDTH: usize = 1440;
const SCREEN_HEIGHT: usize = 900;

pub fn main() {
    let mut window = Window::new(
        (SCREEN_WIDTH / 4) as i32,
        (SCREEN_HEIGHT / 4) as i32,
        (SCREEN_WIDTH / 2) as u32,
        (SCREEN_HEIGHT / 2) as u32,
        "First Window",
    );

    window.set(Color::rgb(10, 200, 10));

    window.sync();
}
