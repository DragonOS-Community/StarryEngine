use core::fmt;

/// 一个像素的颜色值
#[derive(Clone, Copy)]
pub struct Color {
    /// ARGB
    pub data: u32,
}

#[allow(dead_code)]
impl Color {
    /// 通过RGB值创建颜色（不透明）
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color {
            data: 0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
        }
    }

    /// 通过RGBA值创建颜色
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            data: ((a as u32) << 24) | ((r as u32) << 16) | (g as u32) << 8 | (b as u32),
        }
    }

    /// 获取R值
    pub fn r(&self) -> u8 {
        ((self.data & 0x00FF0000) >> 16) as u8
    }

    /// 获取G值
    pub fn g(&self) -> u8 {
        ((self.data & 0x0000FF00) >> 8) as u8
    }

    /// 获取B值
    pub fn b(&self) -> u8 {
        (self.data & 0x000000FF) as u8
    }

    /// 获取A值
    pub fn a(&self) -> u8 {
        ((self.data & 0xFF000000) >> 24) as u8
    }

    /// 颜色插值
    pub fn interpolate(from_color: Color, to_color: Color, scale: f64) -> Color {
        let r = Color::value_interpolate(from_color.r(), to_color.r(), scale);
        let g = Color::value_interpolate(from_color.r(), to_color.r(), scale);
        let b = Color::value_interpolate(from_color.r(), to_color.r(), scale);
        let a = Color::value_interpolate(from_color.r(), to_color.r(), scale);
        Color::rgba(r, g, b, a)
    }

    /// 颜色值插值
    fn value_interpolate(from_value: u8, to_value: u8, scale: f64) -> u8 {
        ((to_value as f64 - from_value as f64) * scale + from_value as f64) as u8
    }

    /// 转化为RGBA字节形式
    pub fn to_rgba_bytes(&self) -> [u8; 4] {
        [self.r(), self.g(), self.b(), self.a()]
    }

    /// 转化为BGRA字节形式(DragonOS帧缓冲数据格式)
    pub fn to_bgra_bytes(&self) -> [u8; 4] {
        [self.b(), self.g(), self.r(), self.a()]
    }
}

/// 比较两个颜色值（不考虑透明度）
impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r() == other.r() && self.g() == other.g() && self.b() == other.b()
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:#010X}", { self.data })
    }
}
