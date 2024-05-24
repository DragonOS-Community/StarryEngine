use std::{
    any::Any,
    cell::{Cell, RefCell},
    str::FromStr,
    sync::{Arc, Weak},
};

use starry_client::base::{color::Color, renderer::Renderer};
use starry_toolkit::{
    base::{panel::Panel, rect::Rect, vector2::Vector2},
    traits::text::Text,
    widgets::{
        image::Image,
        label::{Label, LabelOverflowType},
        PivotType, Widget,
    },
};

pub struct AssetItemList {
    self_ref: RefCell<Weak<AssetItemList>>,
    pub rect: Cell<Rect>,
    pivot: Cell<PivotType>,
    pivot_offset: Cell<Vector2>,
    parent: RefCell<Option<Arc<dyn Widget>>>,
    children: RefCell<Vec<Arc<dyn Widget>>>,
    panel: RefCell<Option<Arc<Panel>>>,
    /// 缓存值
    cache_focused: Cell<bool>,
    pub file_path: RefCell<String>,
}

impl AssetItemList {
    pub const ITEM_WIDTH: u32 = 1024;
    pub const ITEM_HEIGHT: u32 = 32;

    pub fn new(file_name: &str, is_dir: bool) -> Arc<Self> {
        let item = Arc::new(AssetItemList {
            self_ref: RefCell::new(Weak::default()),
            rect: Cell::new(Rect::new(0, 0, Self::ITEM_WIDTH, Self::ITEM_HEIGHT)),
            pivot: Cell::new(PivotType::TopLeft),
            pivot_offset: Cell::new(Vector2::new(0, 0)),
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(None),
            panel: RefCell::new(None),
            cache_focused: Cell::new(false),
            file_path: RefCell::new(String::from_str(file_name).unwrap()),
        });

        (*item.self_ref.borrow_mut()) = Arc::downgrade(&item);

        // 背景Image
        let bg =
            Image::new_from_color(Self::ITEM_WIDTH, Self::ITEM_HEIGHT, Color::rgba(0, 255, 255, 64));
        bg.set_pivot_type(PivotType::Center);
        item.add_child(bg);

        // 文件名Label
        let name = Label::new();
        name.set_adapt_type(LabelOverflowType::Omit);
        name.resize(256, 16);
        name.set_text(file_name);
        name.set_pivot_type(PivotType::Left);
        name.set_pivot_offset(Vector2::new(20, 0));
        name.set_text_pivot_type(PivotType::Left);
        item.add_child(name);

        // 文件类型Label
        let file_type = Label::new();
        file_type.set_adapt_type(LabelOverflowType::ShinkToFit);
        let type_name = if is_dir { "direction" } else { "file" };
        file_type.set_text(type_name);
        file_type.set_pivot_type(PivotType::Center);
        item.add_child(file_type);

        return item;
    }
}

impl Widget for AssetItemList {
    fn self_ref(&self) -> Arc<dyn Widget> {
        self.self_ref.borrow().upgrade().unwrap()
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn name(&self) -> &str {
        "AssetItem_List"
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

    fn panel(&self) -> &RefCell<Option<Arc<Panel>>> {
        &self.panel
    }

    fn draw(&self, renderer: &mut dyn Renderer, focused: bool) {
        if focused != self.cache_focused.get() {
            self.cache_focused.set(focused);

            // 如果当前被选中，则背景高亮
            let children = self.children.borrow_mut();
            let bg_image = children[0].self_ref();
            let bg_image = bg_image
                .as_any_ref()
                .downcast_ref::<Image>()
                .expect("[Error] AssetItem failed to cast widget to image");
            if focused {
                bg_image.set_from_color(Color::rgba(0, 255, 255, 64));
            } else {
                bg_image.set_from_color(Color::rgba(0, 0, 0, 0));
            }
        }

        for child in self.children.borrow().iter() {
            child.draw(renderer, focused);
        }
    }
}
