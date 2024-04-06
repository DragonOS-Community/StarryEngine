use starry_server::core::{SCREEN_HEIGHT, SCREEN_WIDTH};
use starry_toolkit::{
    base::{panel::Panel, rect::Rect},
    traits::{place::Place, text::Text},
    widgets::{image::Image, label::Label},
};

const IMAGE_PATH: &[u8] = include_bytes!("./asset/desktop_bg.png");

fn main() {
    let panel = Panel::new(
        Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
        "Title",
    );

    // Label
    let label = Label::new();
    label.position(100, 100);
    label.text("hello world");
    label.text_offset(50, 50);
    panel.add_child(&label);

    // Image
    let image = Image::from_path(IMAGE_PATH).unwrap();
    image.position(0, SCREEN_HEIGHT as i32 / 2);
    image.size(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32 / 2);
    panel.add_child(&image);

    panel.draw();

    // 便于观察结果
    loop {}
}
