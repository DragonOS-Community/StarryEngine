use std::cmp::{max, min};

/// 表示一个矩形区域
#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    /// 矩形左上角x坐标
    x: i32,
    /// 矩形左上角y坐标
    y: i32,
    /// 矩形宽度
    w: i32,
    /// 矩形高度
    h: i32,
}

#[allow(dead_code)]
impl Rect {
    /// 创建矩形
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        assert!(w >= 0);
        assert!(h >= 0);

        Rect { x, y, w, h }
    }

    /// 矩形的面积
    pub fn area(&self) -> i32 {
        self.w * self.h
    }

    /// 矩形的左边界
    pub fn left(&self) -> i32 {
        self.x
    }

    /// 矩形的右边界
    pub fn right(&self) -> i32 {
        self.x + self.w
    }

    /// 矩形的上边界
    pub fn top(&self) -> i32 {
        self.y
    }

    /// 矩形的下边界
    pub fn bottom(&self) -> i32 {
        self.y + self.h
    }

    /// 矩形的宽度
    pub fn width(&self) -> i32 {
        self.w
    }

    /// 矩形的高度
    pub fn height(&self) -> i32 {
        self.h
    }

    /// 求两矩形的并集
    pub fn container(&self, other: &Rect) -> Rect {
        let left = min(self.left(), other.left());
        let right = max(self.right(), other.right());
        let top = min(self.top(), other.top());
        let bottom = max(self.bottom(), other.bottom());

        assert!(left <= right);
        assert!(top <= bottom);

        Rect::new(left, top, right - left, bottom - top)
    }

    /// 求两矩形的交集
    pub fn intersection(&self, other: &Rect) -> Rect {
        let left = max(self.left(), other.left());
        let right = min(self.right(), other.right());
        let top = max(self.top(), other.top());
        let bottom = min(self.bottom(), other.bottom());

        Rect::new(left, top, max(0, right - left), max(0, bottom - top))
    }

    /// 判断点是否在矩形中
    pub fn contains(&self, x: i32, y: i32) -> bool {
        self.left() <= x && self.right() >= x && self.top() <= y && self.bottom() >= y
    }

    /// 判断矩形是否为空
    pub fn is_empty(&self) -> bool {
        self.w == 0 || self.h == 0
    }

    /// # 函数功能
    /// 偏移矩形的位置
    /// 可用于矩形绝对和相对位置的转换
    ///
    /// ## 参数
    /// - x: 向右偏移的量
    /// - y: 向下偏移的量
    ///
    /// ## 返回值
    /// 偏移得到的矩形
    pub fn offset(&self, x: i32, y: i32) -> Rect {
        Rect::new(self.x + x, self.y + y, self.w, self.h)
    }
}
