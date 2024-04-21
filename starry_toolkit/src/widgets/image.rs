use std::{
    any::Any,
    cell::{Cell, RefCell},
    sync::{Arc, Weak},
};

use starry_client::base::{color::Color, renderer::Renderer};

use crate::{
    base::{panel::Panel, rect::Rect, vector2::Vector2},
    util::get_local_rect,
};

use super::{PivotType, Widget};

use crate::starry_server::base::image::Image as ImageResource;

pub struct Image {
    self_ref: RefCell<Weak<Image>>,
    rect: Cell<Rect>,
    pivot: Cell<PivotType>,
    pivot_offset: Cell<Vector2>,
    children: RefCell<Vec<Arc<dyn Widget>>>,
    parent: RefCell<Option<Arc<dyn Widget>>>,
    panel: RefCell<Option<Arc<Panel>>>,
    /// 图像源数据
    image: RefCell<ImageResource>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Arc<Self> {
        Self::new_from_image(ImageResource::new(width as i32, height as i32))
    }

    pub fn new_from_color(width: u32, height: u32, color: Color) -> Arc<Self> {
        Self::new_from_image(ImageResource::from_color(
            width as i32,
            height as i32,
            color,
        ))
    }

    pub fn new_from_image(image: ImageResource) -> Arc<Self> {
        let image = Arc::new(Image {
            self_ref: RefCell::new(Weak::default()),
            rect: Cell::new(Rect::new(0, 0, image.width() as u32, image.height() as u32)),
            pivot: Cell::new(PivotType::TopLeft),
            pivot_offset: Cell::new(Vector2::new(0, 0)),
            children: RefCell::new(vec![]),
            parent: RefCell::new(None),
            panel: RefCell::new(None),
            image: RefCell::new(image),
        });

        (*image.self_ref.borrow_mut()) = Arc::downgrade(&image);

        return image;
    }

    pub fn new_from_path(path: &[u8]) -> Option<Arc<Self>> {
        if let Some(image) = ImageResource::from_path(path) {
            Some(Self::new_from_image(image))
        } else {
            None
        }
    }

    pub fn set_from_path(&self, path: &[u8]) {
        if let Some(image) = ImageResource::from_path(path) {
            (*self.image.borrow_mut()) = image;
        } else {
            println!("[Error] Image failed to set image");
        }
    }

    pub fn set_from_color(&self, color: Color) {
        (*self.image.borrow_mut()) = ImageResource::from_color(
            self.rect.get().width as i32,
            self.rect.get().height as i32,
            color,
        );
    }
}

impl Widget for Image {
    fn self_ref(&self) -> Arc<dyn Widget> {
        self.self_ref.borrow().upgrade().unwrap()
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn name(&self) -> &str {
        "Image"
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

    fn children(&self) -> &RefCell<Vec<Arc<dyn Widget>>> {
        &self.children
    }

    fn parent(&self) -> &RefCell<Option<Arc<dyn Widget>>> {
        &self.parent
    }

    fn panel(&self) -> &RefCell<Option<Arc<Panel>>> {
        &self.panel
    }

    fn draw(&self, renderer: &mut dyn Renderer, _focused: bool) {
        let image = self.image.borrow();
        if self.panel().borrow().is_some() {
            let panel_rect = self.panel.borrow().clone().unwrap().rect();
            let local_rect = get_local_rect(self.rect.get(), panel_rect);
            renderer.image(
                local_rect.x,
                local_rect.y,
                local_rect.width,
                local_rect.height,
                image.data(),
            );
        } else {
            println!("[Error] Image do not belong to any panel!");
        }
    }
}
