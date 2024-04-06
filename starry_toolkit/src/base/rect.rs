use super::point::Point;

/// 表示一个矩形区域
#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    /// 左上角x坐标
    pub x: i32,
    /// 左上角y坐标
    pub y: i32,
    /// 矩形宽度
    pub width: u32,
    /// 矩形高度
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Rect {
        Rect {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    /// 返回矩形左上角的位置点
    pub fn point(&self) -> Point {
        Point::new(self.x, self.y)
    }

    /// 判断该矩形是否包含某点
    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.x
            && p.x < self.x + self.width as i32
            && p.y >= self.y
            && p.y < self.y + self.height as i32
    }

    /// 判断该矩形是否完全包含另一个矩形
    pub fn contains_rect(&self, r: &Rect) -> bool {
        let p1 = r.point();
        let p2 = p1 + Point::new(r.width as i32, r.height as i32);
        self.contains(p1) && self.contains(p2)
    }

    // 判断该矩形是否和另一矩形有重叠部分
    pub fn intersects(&self, r: &Rect) -> bool {
        !(r.x >= (self.x + self.width as i32)
            || self.x >= (r.x + r.width as i32)
            || r.y >= (self.y + self.height as i32)
            || self.y >= (r.y + r.height as i32))
    }
}
