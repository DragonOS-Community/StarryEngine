use std::{
    cell::{Cell, RefCell},
    str::FromStr,
    sync::Arc,
};

use starry_client::base::{color::Color, renderer::Renderer};
use starry_toolkit::{
    base::{rect::Rect, vector2::Vector2},
    traits::text::Text,
    widgets::{
        image::Image,
        label::{Label, LabelOverflowType},
        widget_add_child, PivotType, Widget,
    },
};

use crate::starry_server::base::image::Image as ImageResource;

const FILE_ICON_PATH: &[u8] = include_bytes!("../resource/file_icon.png");
const DIR_ICON_PATH: &[u8] = include_bytes!("../resource/dir_icon.png");

pub struct AssetItem {
    pub rect: Cell<Rect>,
    pivot: Cell<PivotType>,
    pivot_offset: Cell<Vector2>,
    parent: RefCell<Option<Arc<dyn Widget>>>,
    children: RefCell<Vec<Arc<dyn Widget>>>,
    /// 缓存值
    cache_focused: Cell<bool>,
    _file_path: RefCell<String>,
}

impl AssetItem {
    pub const ITEM_WIDTH: u32 = 144;
    pub const ITEM_HEIGHT: u32 = 144;

    pub fn new(file_name: &str, is_dir: bool) -> Arc<Self> {
        let item = Arc::new(AssetItem {
            rect: Cell::new(Rect::new(0, 0, Self::ITEM_WIDTH, Self::ITEM_HEIGHT)),
            pivot: Cell::new(PivotType::TopLeft),
            pivot_offset: Cell::new(Vector2::new(0, 0)),
            parent: RefCell::new(None),
            children: RefCell::new(Vec::new()),
            cache_focused: Cell::new(false),
            _file_path: RefCell::new(String::from_str(file_name).unwrap()),
        });

        // 背景Image
        let bg = Image::from_color(Self::ITEM_WIDTH, Self::ITEM_HEIGHT, Color::rgba(0, 0, 0, 0));
        bg.set_pivot_type(PivotType::Center);
        widget_add_child(item.clone(), bg.clone());

        // 文件图标Image
        if let Some(icon) = match is_dir {
            true => ImageResource::from_path(DIR_ICON_PATH),
            false => ImageResource::from_path(FILE_ICON_PATH),
        } {
            let icon = Image::from_image(icon);
            icon.set_pivot_type(PivotType::Top);
            widget_add_child(item.clone(), icon.clone());
        }

        // 文件名Label
        let name = Label::new();
        name.set_adapt_type(LabelOverflowType::Omit);
        name.resize(Self::ITEM_WIDTH, 16);
        name.set_text(file_name);
        name.set_pivot_type(PivotType::Bottom);
        name.set_pivot_offset(Vector2::new(0, -4));
        widget_add_child(item.clone(), name.clone());

        return item;
    }
}

impl Widget for AssetItem {
    fn name(&self) -> &str {
        "AssetItem"
    }

    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn pivot(&self) -> &Cell<PivotType> {
        &self.pivot
    }

    fn pivot_offset(&self) -> &Cell<Vector2> {
        &self.pivot_offset
    }

    fn parent(&self) -> &RefCell<Option<Arc<dyn Widget>>> {
        &self.parent
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
