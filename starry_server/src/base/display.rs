use starry_client::base::{color::Color, renderer::Renderer};

use super::{
    image::{Image, ImageRoi},
    rect::Rect,
};

/// 一个显示窗口
pub struct Display {
    /// 左上角x坐标
    pub x: i32,
    /// 左上角y坐标
    pub y: i32,
    /// 帧缓冲区
    pub image: Image,
}

impl Display {
    /// 创建新窗口
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Display {
            x,
            y,
            image: Image::new(width, height),
        }
    }

    /// # 函数功能
    /// 向一个矩形区域内填充单一颜色
    ///
    /// ## 参数值
    /// - rect: 矩形区域(绝对位置)
    /// - color: 填充的颜色
    pub fn rect(&mut self, rect: &Rect, color: Color) {
        self.image.rect(
            rect.left() - self.x,
            rect.top() - self.y,
            rect.width().try_into().unwrap_or(0),
            rect.height().try_into().unwrap_or(0),
            color,
        );
    }

    /// # 函数功能
    /// 获得矩形区域相应的Roi
    ///
    /// ## 参数值
    /// - rect: 矩形区域(绝对位置)
    ///
    /// ## 返回值
    /// 矩形区域对应的Roi
    pub fn roi(&mut self, rect: &Rect) -> ImageRoi {
        // 得到相对位置的矩形
        self.image.roi(&Rect::new(
            rect.left() - self.x,
            rect.top() - self.y,
            rect.width(),
            rect.height(),
        ))
    }

    /// 窗口调整大小
    pub fn resize(&mut self, width: i32, height: i32) {
        self.image = Image::new(width, height)
    }

    /// 获得展示窗口矩形
    pub fn screen_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.image.width(), self.image.height())
    }
}
