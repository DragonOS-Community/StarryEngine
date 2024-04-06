use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};

use starry_client::base::{color::Color, renderer::Renderer};

use crate::{base::{point::Point, rect::Rect}, traits::{place::Place, text::Text}};

use super::{HorizontalPlacement, VerticalPlacement, Widget};

pub struct Label {
    pub rect: Cell<Rect>,
    local_position: Cell<Point>,
    vertical_placement: Cell<VerticalPlacement>,
    horizontal_placement: Cell<HorizontalPlacement>,
    children: RefCell<Vec<Arc<dyn Widget>>>,
    pub text: RefCell<String>,
    pub text_offset: Cell<Point>,
}

impl Label {
        pub fn new() -> Arc<Self> {
        Arc::new(Label {
            rect: Cell::new(Rect::default()),
            local_position: Cell::new(Point::new(0, 0)),
            vertical_placement: Cell::new(VerticalPlacement::Absolute),
            horizontal_placement: Cell::new(HorizontalPlacement::Absolute),
            children: RefCell::new(vec![]),
            text: RefCell::new(String::new()),
            text_offset: Cell::new(Point::default()),
        })
    }

    fn adjust_size(&self) {
        let text = self.text.borrow();
        self.size(
            text.len() as u32 * 8 + 2 * self.text_offset.get().x as u32,
            16 + 2 * self.text_offset.get().y as u32,
        );
    }
}

impl Place for Label {}

impl Widget for Label {
    fn name(&self) -> &str {
        "Label"
    }

    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn local_position(&self) -> &Cell<Point> {
        &self.local_position
    }

    fn vertical_placement(&self) -> &Cell<VerticalPlacement> {
        &self.vertical_placement
    }

    fn horizontal_placement(&self) -> &Cell<HorizontalPlacement> {
        &self.horizontal_placement
    }

    fn children(&self) -> &RefCell<Vec<Arc<dyn Widget>>> {
        &self.children
    }

    fn draw(&self, renderer: &mut dyn Renderer) {
        let origin_rect = self.rect().get();
        let mut current_rect = self.rect().get(); // 当前字符渲染矩形
        let origin_x = origin_rect.x;
        let text = self.text.borrow().clone();

        for char in text.chars() {
            if char == '\n' {
                // 换行 退格到起始位置
                current_rect.x = origin_x;
                current_rect.y += 16;
            } else {
                // 避免超出矩形范围
                if current_rect.x + 8 <= origin_rect.x + origin_rect.width as i32
                    && current_rect.y + 16 <= origin_rect.y + origin_rect.height as i32
                {
                    // 默认渲染白色字体
                    // TODO 应用主题(Theme)颜色
                    renderer.char(current_rect.x, current_rect.y, char, Color::rgb(255, 255, 255));
                }
                current_rect.x += 8;
            }
        }
    }
}

impl Text for Label {
    fn text<S: Into<String>>(&self, target_text: S) -> &Self {
        {
            let mut text = self.text.borrow_mut();
            *text = target_text.into();
        }
        self.adjust_size();
        self
    }

    fn text_offset(&self, x: i32, y: i32) -> &Self {
        self.text_offset.set(Point::new(x, y));
        self.adjust_size();
        self
    }
}