use starry_client::base::color::Color;
use starry_server::core::{SCREEN_HEIGHT, SCREEN_WIDTH};
use starry_toolkit::{
    base::{panel::Panel, rect::Rect},
    layout::grid::Grid,
    traits::text::Text,
    widgets::label::Label,
};

fn main() {
    let panel = Panel::new(
        Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
        "Title",
        Color::rgb(255, 255, 255),
    );

    let label1 = Label::new();
    label1.set_text("abc");

    let label2 = Label::new();
    label2.set_text("....");

    let label3 = Label::new();
    label3.set_text("12.g");

    let grid = Grid::new();
    grid.set_space(10, 10);
    grid.set_upper_limit(2);
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
