use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};

use starry_client::base::{color::Color, renderer::Renderer};
use starry_toolkit::{
    base::{point::Point, rect::Rect},
    traits::{text::Text, transform::Transform},
    widgets::{image::Image, label::Label, HorizontalPlacement, VerticalPlacement, Widget},
};

use crate::starry_server::base::image::Image as ImageResource;

const FILE_ICON_PATH: &[u8] = include_bytes!("../resource/file_icon.png");
const DIR_ICON_PATH: &[u8] = include_bytes!("../resource/dir_icon.png");

pub enum AssetType {
    Folder,
    File,
}

pub struct AssetItem {
    pub rect: Cell<Rect>,
    local_position: Cell<Point>,
    vertical_placement: Cell<VerticalPlacement>,
    horizontal_placement: Cell<HorizontalPlacement>,
    children: RefCell<Vec<Arc<dyn Widget>>>,
    /// 缓存值
    cache_focused: Cell<bool>,
}

impl AssetItem {
    pub const ITEM_WIDTH: u32 = 144;
    pub const ITEM_HEIGHT: u32 = 144;

    pub fn new(file_name: &str, is_dir: bool) -> Self {
        let item = AssetItem {
            rect: Cell::new(Rect::new(0, 0, Self::ITEM_WIDTH, Self::ITEM_HEIGHT)),
            local_position: Cell::new(Point::new(0, 0)),
            vertical_placement: Cell::new(VerticalPlacement::Absolute),
            horizontal_placement: Cell::new(HorizontalPlacement::Absolute),
            children: RefCell::new(Vec::new()),
            cache_focused: Cell::new(false),
        };

        // 背景Image
        let bg = Image::from_color(160, 160, Color::rgba(0, 0, 0, 0));
        item.add_child(bg);

        // 文件图标Image
        if let Some(icon) = match is_dir {
            true => ImageResource::from_path(DIR_ICON_PATH),
            false => ImageResource::from_path(FILE_ICON_PATH),
        } {
            let icon = Image::from_image(icon);
            icon.horizontal_placement().set(HorizontalPlacement::Center);
            icon.vertical_placement().set(VerticalPlacement::Top);
            item.add_child(icon);
        }

        // 文件名Label
        let name = Label::new();
        name.text(file_name);
        name.horizontal_placement().set(HorizontalPlacement::Center);
        name.vertical_placement().set(VerticalPlacement::Bottom);
        item.add_child(name);

        return item;
    }
}

impl Transform for AssetItem {}

impl Widget for AssetItem {
    fn name(&self) -> &str {
        "AssetItem"
    }

    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn vertical_placement(&self) -> &Cell<VerticalPlacement> {
        &self.vertical_placement
    }

    fn horizontal_placement(&self) -> &Cell<HorizontalPlacement> {
        &self.horizontal_placement
    }

    fn local_position(&self) -> &Cell<Point> {
        &self.local_position
    }

    fn children(&self) -> &RefCell<Vec<Arc<dyn Widget>>> {
        &self.children
    }

    fn draw(&self, renderer: &mut dyn Renderer, focused: bool) {
        if focused != self.cache_focused.get() {
            self.cache_focused.set(focused);

            // 如果当前被选中，则背景高亮
            let mut children = self.children.borrow_mut();
            if focused {
                children[0] = Image::from_color(
                    Self::ITEM_WIDTH,
                    Self::ITEM_HEIGHT,
                    Color::rgba(0, 255, 255, 128),
                );
            } else {
                children[0] =
                    Image::from_color(Self::ITEM_WIDTH, Self::ITEM_HEIGHT, Color::rgba(0, 0, 0, 0));
            }
        }

        for child in self.children.borrow().iter() {
            child.draw(renderer, focused);
        }
    }
}
