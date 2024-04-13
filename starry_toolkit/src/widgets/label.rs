use std::{
    cell::{Cell, RefCell},
    cmp::max,
    sync::Arc,
};

use starry_client::base::{color::Color, renderer::Renderer};

use crate::{
    base::{rect::Rect, vector2::Vector2},
    traits::text::Text,
};

use super::{align_rect, PivotType, Widget};

#[derive(PartialEq, Copy, Clone)]
pub enum LabelOverflowType {
    /// 不适配 溢出部分不显示
    None,
    /// 根据字数调整大小
    ShinkToFit,
    /// 省略多余内容
    Omit,
    // TODO 支持"调整字体大小以适配"的选项
}

pub struct Label {
    pub rect: Cell<Rect>,
    pivot: Cell<PivotType>,
    pivot_offset: Cell<Vector2>,
    parent: RefCell<Option<Arc<dyn Widget>>>,
    children: RefCell<Vec<Arc<dyn Widget>>>,
    /// 实际上的文本
    real_text: RefCell<String>,
    /// 用于显示的文本
    show_text: RefCell<String>,
    text_color: Cell<Color>,
    adapt_type: Cell<LabelOverflowType>,
    /// 渲染文本时的矩形区域
    text_rect: Cell<Rect>,
    /// 文本在矩形框内的对齐方式
    text_pivot: Cell<PivotType>,
}

// TODO 暂时只支持渲染一行字体
impl Label {
    pub fn new() -> Arc<Self> {
        Arc::new(Label {
            rect: Cell::new(Rect::default()),
            pivot: Cell::new(PivotType::TopLeft),
            pivot_offset: Cell::new(Vector2::new(0, 0)),
            parent: RefCell::new(None),
            children: RefCell::new(vec![]),
            real_text: RefCell::new(String::new()),
            show_text: RefCell::new(String::new()),
            text_color: Cell::new(Color::rgb(0, 0, 0)), // 默认黑色字体
            adapt_type: Cell::new(LabelOverflowType::None),
            text_rect: Cell::new(Rect::default()),
            text_pivot: Cell::new(PivotType::Center),
        })
    }

    /// 处理文本溢出的情况
    /// 在文本内容改变或大小改变时调用
    fn handle_overflow(&self) {
        let text = self.real_text.borrow();

        match self.adapt_type.get() {
            LabelOverflowType::None => {}
            LabelOverflowType::ShinkToFit => {
                self.resize(text.len() as u32 * 8 as u32, 16);
            }
            LabelOverflowType::Omit => {
                let rect = self.rect.get();

                if text.len() as u32 * 8 > rect.width {
                    let max_count = max(0, (rect.width as i32 - 3 * 8) / 8);
                    let mut omit_str = self.real_text.borrow().clone();
                    let _ = omit_str.split_off(max_count as usize);
                    omit_str.push_str("..."); // 溢出字符用省略号取代
                    (*self.show_text.borrow_mut()) = omit_str;
                }
            }
        }

        self.text_rect.set(Rect::new(
            0,
            0,
            self.show_text.borrow().len() as u32 * 8,
            16,
        ));
    }

    pub fn set_adapt_type(&self, adapt_type: LabelOverflowType) {
        self.adapt_type.set(adapt_type);
    }
}

impl Widget for Label {
    fn name(&self) -> &str {
        "Label"
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
        let origin_rect = self.text_rect.get();
        let mut current_rect = self.text_rect.get(); // 当前字符渲染矩形
        let origin_x = origin_rect.x;
        let text = self.show_text.borrow().clone();

        // 矩形高度不满足
        if origin_rect.height < 16 {
            return;
        }

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
                    // TODO 应用主题(Theme)颜色
                    renderer.char(current_rect.x, current_rect.y, char, self.text_color.get());
                }
                current_rect.x += 8;
            }
        }
    }

    fn set_pivot_type(&self, pivot_type: PivotType) {
        self.set_pivot_type_base(pivot_type);

        self.text_rect.set(align_rect(
            self.text_rect.get(),
            self.rect.get(),
            self.text_pivot.get(),
            Vector2::new(0, 0),
        ));
    }

    fn set_pivot_offset(&self, pivot_offset: Vector2) {
        self.set_pivot_offset_base(pivot_offset);

        self.text_rect.set(align_rect(
            self.text_rect.get(),
            self.rect.get(),
            self.text_pivot.get(),
            Vector2::new(0, 0),
        ));
    }

    fn resize(&self, width: u32, height: u32) {
        self.resize_base(width, height);

        self.handle_overflow();
        self.text_rect.set(align_rect(
            self.text_rect.get(),
            self.rect.get(),
            self.text_pivot.get(),
            Vector2::new(0, 0),
        ));
    }

    fn arrange_self(&self) {
        self.arrange_self_base();

        self.text_rect.set(align_rect(
            self.text_rect.get(),
            self.rect.get(),
            self.text_pivot.get(),
            Vector2::new(0, 0),
        ));
    }
}

impl Text for Label {
    fn set_text<S: Into<String>>(&self, text: S) -> &Self {
        let text = text.into();
        (*self.real_text.borrow_mut()) = text.clone();
        (*self.show_text.borrow_mut()) = text;
        self.handle_overflow();
        align_rect(
            self.text_rect.get(),
            self.rect.get(),
            self.text_pivot.get(),
            Vector2::new(0, 0),
        );
        self
    }

    fn set_text_color(&self, color: Color) -> &Self {
        self.text_color.set(color);
        self
    }
}
