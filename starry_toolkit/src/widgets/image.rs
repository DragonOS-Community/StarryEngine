use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};

use starry_client::base::{color::Color, renderer::Renderer};

use crate::{
    base::{point::Point, rect::Rect},
    traits::transform::Transform,
};

use super::{HorizontalPlacement, VerticalPlacement, Widget};

use crate::starry_server::base::image::Image as ImageAsset;

pub struct Image {
    pub rect: Cell<Rect>,
    local_position: Cell<Point>,
    vertical_placement: Cell<VerticalPlacement>,
    horizontal_placement: Cell<HorizontalPlacement>,
    children: RefCell<Vec<Arc<dyn Widget>>>,
    /// 图像源数据
    pub image: RefCell<ImageAsset>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Arc<Self> {
        Self::from_image(ImageAsset::new(width as i32, height as i32))
    }

    pub fn from_color(width: u32, height: u32, color: Color) -> Arc<Self> {
        Self::from_image(ImageAsset::from_color(width as i32, height as i32, color))
    }

    pub fn from_image(image: ImageAsset) -> Arc<Self> {
        Arc::new(Image {
            rect: Cell::new(Rect::new(0, 0, image.width() as u32, image.height() as u32)),
            local_position: Cell::new(Point::new(0, 0)),
            vertical_placement: Cell::new(VerticalPlacement::Absolute),
            horizontal_placement: Cell::new(HorizontalPlacement::Absolute),
            children: RefCell::new(vec![]),
            image: RefCell::new(image),
        })
    }

    pub fn from_path(path: &[u8]) -> Option<Arc<Self>> {
        if let Some(image) = ImageAsset::from_path(path) {
            Some(Self::from_image(image))
        } else {
            None
        }
    }
}

impl Transform for Image {}

impl Widget for Image {
    fn name(&self) -> &str {
        "Image"
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

    fn draw(&self, renderer: &mut dyn Renderer, _focused: bool) {
        let rect = self.rect.get();
        let image = self.image.borrow();
        renderer.image(rect.x, rect.y, rect.width, rect.height, image.data());
    }
}
