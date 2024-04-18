use std::{
    cell::{Cell, RefCell},
    str::FromStr,
    sync::{Arc, Weak},
};

use starry_client::base::{color::Color, renderer::Renderer};
use starry_toolkit::{
    base::{rect::Rect, vector2::Vector2},
    traits::text::Text,
    widgets::{
        align_rect,
        image::Image,
        label::{Label, LabelOverflowType},
        PivotType, Widget,
    },
};

use crate::starry_server::base::image::Image as ImageResource;

const FILE_ICON_PATH: &[u8] = include_bytes!("../resource/file_icon.png");
const DIR_ICON_PATH: &[u8] = include_bytes!("../resource/dir_icon.png");

pub struct AssetItem {
    self_ref: RefCell<Weak<AssetItem>>,
    pub rect: Cell<Rect>,
    pivot: Cell<PivotType>,
    pivot_offset: Cell<Vector2>,
    parent: RefCell<Option<Arc<dyn Widget>>>,
    children: RefCell<Vec<Arc<dyn Widget>>>,
    panel_rect: Cell<Option<Rect>>,
    /// 缓存值
    cache_focused: Cell<bool>,
    pub file_path: RefCell<String>,
}

impl AssetItem {
    pub const ITEM_WIDTH: u32 = 144;
    pub const ITEM_HEIGHT: u32 = 144;

    pub fn new(file_name: &str, is_dir: bool) -> Arc<Self> {
        let item = Arc::new(AssetItem {
            self_ref: RefCell::new(Weak::default()),
            rect: Cell::new(Rect::new(0, 0, Self::ITEM_WIDTH, Self::ITEM_HEIGHT)),
            pivot: Cell::new(PivotType::TopLeft),
            pivot_offset: Cell::new(Vector2::new(0, 0)),
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(None),
            panel_rect: Cell::new(None),
            cache_focused: Cell::new(false),
            file_path: RefCell::new(String::from_str(file_name).unwrap()),
        });

        (*item.self_ref.borrow_mut()) = Arc::downgrade(&item);

        // 背景Image
        let bg = Image::from_color(Self::ITEM_WIDTH, Self::ITEM_HEIGHT, Color::rgba(0, 0, 0, 0));
        bg.set_pivot_type(PivotType::Center);
        item.add_child(bg);

        // 文件图标Image
        if let Some(icon) = match is_dir {
            true => ImageResource::from_path(DIR_ICON_PATH),
            false => ImageResource::from_path(FILE_ICON_PATH),
        } {
            let icon = Image::from_image(icon);
            icon.set_pivot_type(PivotType::Top);
            item.add_child(icon);
        }

        // 文件名Label
        let name = Label::new();
        name.set_adapt_type(LabelOverflowType::Omit);
        name.resize(Self::ITEM_WIDTH, 16);
        name.set_text(file_name);
        name.set_pivot_type(PivotType::Bottom);
        name.set_pivot_offset(Vector2::new(0, -4));
        item.add_child(name);

        return item;
    }
}

impl Widget for AssetItem {
    fn self_ref(&self) -> Arc<dyn Widget> {
        self.self_ref.borrow().upgrade().unwrap()
    }

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

    fn panel_rect(&self) -> &Cell<Option<Rect>> {
        &self.panel_rect
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
                    Color::rgba(0, 255, 255, 64),
                );
            } else {
                children[0] =
                    Image::from_color(Self::ITEM_WIDTH, Self::ITEM_HEIGHT, Color::rgba(0, 0, 0, 0));
            }

            // TODO
            // children[0].set_pivot_type(PivotType::Center);

            children[0].rect().set(align_rect(
                children[0].rect().get(),
                self.rect.get(),
                PivotType::Center,
                Vector2::new(0, 0),
            ))
        }

        for child in self.children.borrow().iter() {
            child.draw(renderer, focused);
        }
    }
}
