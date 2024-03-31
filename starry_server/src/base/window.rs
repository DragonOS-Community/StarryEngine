use starry_client::base::event::Event;

use crate::core::SCREEN_HEIGHT;

use super::{display::Display, image::Image, rect::Rect};

/// 窗口按Z值排序的模式
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum WindowZOrderMode {
    /// 背景窗口
    Back,
    /// 普通窗口
    Normal,
    /// 前景窗口
    Front,
}

#[allow(dead_code)]
/// 窗口对象
pub struct Window {
    /// 窗口左上角x坐标
    pub x: i32,
    /// 窗口左上角y坐标
    pub y: i32,
    /// 窗口大小系数
    pub scale: i32,
    /// 窗口标题
    pub title: String,
    // pub asynchronous: bool,
    // pub barderless: bool,
    // pub resizable: bool,
    /// 是否透明
    pub transparent: bool,
    // pub unclosable: bool,
    /// 排序模式
    pub zorder: WindowZOrderMode,

    /// 窗体图像
    pub image: Image,
    /// 窗口标题图像(聚焦)
    // pub title_image: Image,
    /// 窗口标题图像(非聚焦)
    // pub title_image_unfocused: Image,

    /// 事件数组
    pub events: Vec<Event>,
    // TODO 增加所需字段
}

impl Window {
    pub fn new(x: i32, y: i32, _w: i32, _h: i32, scale: i32, image_path: &[u8]) -> Window {
        Window {
            x: x,
            y: y,
            scale: scale,
            title: String::new(),
            transparent: false,
            zorder: WindowZOrderMode::Normal,
            image: Image::from_path_scale(image_path)
                .unwrap_or(Image::new(SCREEN_HEIGHT as i32, SCREEN_HEIGHT as i32)),
            events: Vec::new(),
        }
    }

    /// 窗体宽度
    pub fn width(&self) -> i32 {
        self.image.width()
    }

    /// 窗体高度
    pub fn height(&self) -> i32 {
        self.image.height()
    }

    /// 返回窗体对应矩形
    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width(), self.height())
    }

    // TODO
    // pub fn title_rect(&self) -> Rect {}

    /// # 函数功能
    /// 渲染窗体到显示窗口中
    ///
    /// ## 参数
    /// - display: 展示窗口
    /// - rect: 渲染的矩形区域(绝对位置)
    pub fn draw(&mut self, display: &mut Display, rect: &Rect) {
        let self_rect = self.rect();
        let intersect = self_rect.intersection(rect);
        if !intersect.is_empty() {
            // (半)透明窗口
            if self.transparent {
                display.roi(&intersect).blend(
                    &self
                        .image
                        .roi(&intersect.offset(-self_rect.left(), -self_rect.top())),
                );
            }
            // 不透明窗口
            else {
                display.roi(&intersect).cover(
                    &self
                        .image
                        .roi(&intersect.offset(-self_rect.left(), -self_rect.top())),
                );
            }
        }
    }
}
