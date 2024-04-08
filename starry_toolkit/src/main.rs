use starry_server::core::{SCREEN_HEIGHT, SCREEN_WIDTH};
use starry_toolkit::{
    base::{panel::Panel, rect::Rect},
    layout::grid::Grid,
    traits::{text::Text, transform::Transform},
    widgets::label::Label,
};

// const IMAGE_PATH: &[u8] = include_bytes!("./asset/desktop_bg.png");

fn main() {
    let panel = Panel::new(
        Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
        "Title",
    );

    let label1 = Label::new();
    label1.text("hello world");

    let label2 = Label::new();
    label2.text("hello world");

    let label3 = Label::new();
    label3.text("hello world");

    let grid = Grid::new();
    grid.set_space(10, 10);
    grid.resize(500, 500);
    grid.set_max_columns(2);
    grid.add(&label1);
    grid.add(&label2);
    grid.add(&label3);

    panel.add_child(&grid);

    // // Image
    // let image = Image::from_path(IMAGE_PATH).unwrap();
    // image.reposition(0, SCREEN_HEIGHT as i32 / 2);
    // image.resize(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32 / 2);
    // panel.add_child(&image);

    panel.draw();

    // 便于观察结果
    loop {}
}
