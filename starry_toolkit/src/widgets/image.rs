use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};

use starry_client::base::{color::Color, renderer::Renderer};

use crate::base::{rect::Rect, vector2::Vector2};

use super::{PivotType, Widget};

use crate::starry_server::base::image::Image as ImageResource;

pub struct Image {
    pub rect: Cell<Rect>,
    pivot: Cell<PivotType>,
    pivot_offset: Cell<Vector2>,
    children: RefCell<Vec<Arc<dyn Widget>>>,
    parent: RefCell<Option<Arc<dyn Widget>>>,
    /// 图像源数据
    pub image: RefCell<ImageResource>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Arc<Self> {
        Self::from_image(ImageResource::new(width as i32, height as i32))
    }

    pub fn from_color(width: u32, height: u32, color: Color) -> Arc<Self> {
        Self::from_image(ImageResource::from_color(
            width as i32,
            height as i32,
            color,
        ))
    }

    pub fn from_image(image: ImageResource) -> Arc<Self> {
        Arc::new(Image {
            rect: Cell::new(Rect::new(0, 0, image.width() as u32, image.height() as u32)),
            pivot: Cell::new(PivotType::TopLeft),
            pivot_offset: Cell::new(Vector2::new(0, 0)),
            parent: RefCell::new(None),
            children: RefCell::new(vec![]),
            image: RefCell::new(image),
        })
    }

    pub fn from_path(path: &[u8]) -> Option<Arc<Self>> {
        if let Some(image) = ImageResource::from_path(path) {
            Some(Self::from_image(image))
        } else {
            None
        }
    }
}

impl Widget for Image {
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

    fn parent(&self) -> &RefCell<Option<Arc<dyn Widget>>> {
        &self.parent
    }

    fn children(&self) -> &RefCell<Vec<Arc<dyn Widget>>> {
        &self.children
    }

    fn draw(&self, renderer: &mut dyn Renderer, _focused: bool) {
        let rect = self.rect.get();
        let image = self.image.borrow();
        renderer.image(rect.x, rect.y, rect.width, rect.height, image.data());
    }
}
