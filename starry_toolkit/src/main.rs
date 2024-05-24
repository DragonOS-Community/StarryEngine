use starry_client::base::color::Color;
use starry_server::core::{SCREEN_HEIGHT, SCREEN_WIDTH};
use starry_toolkit::base::{
    panel::{Panel, PanelRendererMode},
    rect::Rect,
};
use starry_toolkit::{
    layout::grid::{Grid, GridArrangeType},
    widgets::image::Image,
};

fn main() {
    let panel = Panel::new(
        Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
        "Title",
        Color::rgb(255, 255, 255),
    );
    // 显示矩形线框
    panel.set_renderer_mode(PanelRendererMode::WithWireframe);

    let image1 = Image::new_from_color(32, 32, Color::rgb(128, 255, 255));
    let image2 = Image::new_from_color(32, 32, Color::rgb(128, 255, 255));
    let image3 = Image::new_from_color(32, 32, Color::rgb(128, 255, 255));
    let image4 = Image::new_from_color(32, 32, Color::rgb(128, 255, 255));
    let image5 = Image::new_from_color(32, 32, Color::rgb(128, 255, 255));

    let grid = Grid::new();
    grid.set_arrange_type(GridArrangeType::Horizontal); //优先横向排列
    grid.set_upper_limit(3); //每行最大元素个数为3

    grid.set_space(20, 20);
    grid.add_element(&image1);
    grid.add_element(&image2);
    grid.add_element(&image3);
    grid.add_element(&image4);
    grid.add_element(&image5);

    panel.add_child(&grid);
    panel.draw();

    // 便于观察结果
    loop {}
}
