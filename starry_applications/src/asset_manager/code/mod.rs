use crate::starry_toolkit::traits::focus::Focus;
use starry_server::core::{SCREEN_HEIGHT, SCREEN_WIDTH};
use starry_toolkit::{
    base::{panel::Panel, rect::Rect},
    layout::grid::Grid,
    traits::transform::Transform,
    widgets::image::Image,
};
use std::{cell::RefCell, fs, sync::Arc};

use self::asset_item::AssetItem;

use crate::starry_server::base::image::Image as ImageResource;

pub mod asset_item;

const DESKTOP_BG_PATH: &[u8] = include_bytes!("../resource/desktop_bg.png");

pub struct AssetViewer {
    cur_path: String,
    asset_grid: RefCell<Arc<Grid>>,
}

impl AssetViewer {
    pub fn new() -> Self {
        AssetViewer {
            cur_path: String::from("/"),
            asset_grid: RefCell::new(Grid::new()),
        }
    }

    pub fn init(&self) {
        let grid = self.asset_grid.borrow();
        grid.resize(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
        grid.reposition(0, 0);
        grid.set_max_columns(5);
        grid.set_space(20, 20);
    }

    pub fn refresh(&self) {
        // 读取目录中的文件列表
        if let Ok(entries) = fs::read_dir(&self.cur_path) {
            for entry in entries {
                if let Ok(item) = entry {
                    let item = AssetItem::new(
                        item.file_name().to_str().unwrap(),
                        item.metadata().unwrap().is_dir(),
                    );
                    self.asset_grid.borrow_mut().add(&Arc::new(item));
                }
            }
        }

        // TODO 代码整理
        let grid = self.asset_grid.borrow_mut();
        let elements = grid.elements.borrow();
        if let Some(widget) = elements.get(&(0, 0)) {
            grid.focused_id.set(Some((0, 0)));
            grid.focus(widget);
        }
    }

    pub fn draw(&self) {
        let panel = Panel::new(
            Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
            "Title",
        );

        panel.add_child(&Image::from_image(
            ImageResource::from_path(DESKTOP_BG_PATH).unwrap(),
        ));

        panel.add_child(&(self.asset_grid.borrow()));
        panel.draw();
    }
}
