use std::{
    cell::Cell,
    cmp,
    fs::File,
    io::{Seek, SeekFrom, Write},
    mem, ptr, slice,
};

use image::GenericImageView;
use resize::Type;
use starry_client::base::{
    color::Color,
    renderer::{RenderMode, Renderer},
};

use crate::core::{SCREEN_HEIGHT, SCREEN_WIDTH};

use super::rect::Rect;

/// Roi区域中的行数据
pub struct ImageRoiRows<'a> {
    /// Roi矩形区域(相对位置)
    rect: Rect,
    /// 矩形宽度
    w: i32,
    /// 帧缓冲数据
    data: &'a [Color],
    /// 当前行号
    i: i32,
}

// 实现迭代器
impl<'a> Iterator for ImageRoiRows<'a> {
    type Item = &'a [Color];
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.rect.height() {
            let start = (self.rect.top() + self.i) * self.w + self.rect.left();
            let end = start + self.rect.width();
            self.i += 1;
            Some(&self.data[start as usize..end as usize])
        } else {
            None
        }
    }
}

/// Roi区域中的行数据
pub struct ImageRoiRowsMut<'a> {
    /// Roi矩形区域(相对位置)
    rect: Rect,
    /// 矩形宽度
    w: i32,
    /// 帧缓冲数据
    data: &'a mut [Color],
    /// 当前行号
    i: i32,
}

// 实现迭代器
impl<'a> Iterator for ImageRoiRowsMut<'a> {
    type Item = &'a mut [Color];
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.rect.height() {
            let mut data = mem::take(&mut self.data);

            // 剔除掉矩形以上的部分
            if self.i == 0 {
                data = data
                    .split_at_mut(self.rect.top() as usize * self.w as usize)
                    .1
            };

            // 分离当前行和剩下的部分
            let (row, tail) = data.split_at_mut(self.w as usize);
            self.data = tail;
            let start = self.rect.left() as usize;
            let end = self.rect.left() as usize + self.rect.width() as usize;
            self.i += 1;
            Some(&mut row[start..end])
        } else {
            None
        }
    }
}

/// 图像中的ROI区域
pub struct ImageRoi<'a> {
    /// ROI矩形区域(相对位置)
    rect: Rect,
    /// 矩形区域宽度
    w: i32,
    /// 帧缓冲数据
    data: &'a mut [Color],
}

// 实现到迭代器的转换
impl<'a> IntoIterator for ImageRoi<'a> {
    type Item = &'a [Color];
    type IntoIter = ImageRoiRows<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { rect, w, data } = self;
        // 两次切片操作
        let data =
            &mut data[rect.top() as usize * w as usize..][..rect.height() as usize * w as usize];
        ImageRoiRows {
            rect,
            w,
            data,
            i: 0,
        }
    }
}

impl<'a> ImageRoi<'a> {
    /// 获得Roi相应的行数据
    pub fn rows(&'a self) -> ImageRoiRows<'a> {
        ImageRoiRows {
            rect: self.rect,
            w: self.w,
            data: self.data,
            i: 0,
        }
    }

    /// 获得Roi相应的行数据
    pub fn rows_mut(&'a mut self) -> ImageRoiRowsMut<'a> {
        ImageRoiRowsMut {
            rect: self.rect,
            w: self.w,
            data: self.data,
            i: 0,
        }
    }

    /// Roi区域颜色混合
    pub fn blend(&'a mut self, other: &ImageRoi) {
        for (self_row, other_row) in self.rows_mut().zip(other.rows()) {
            for (old, new) in self_row.iter_mut().zip(other_row.iter()) {
                let alpha = (new.data >> 24) & 0xFF;
                if alpha >= 255 {
                    old.data = new.data;
                } else if alpha > 0 {
                    let n_r = (((new.data >> 16) & 0xFF) * alpha) >> 8;
                    let n_g = (((new.data >> 8) & 0xFF) * alpha) >> 8;
                    let n_b = ((new.data & 0xFF) * alpha) >> 8;

                    let n_alpha = 255 - alpha;

                    let o_r = (((old.data >> 16) & 0xFF) * n_alpha) >> 8;
                    let o_g = (((old.data >> 8) & 0xFF) * n_alpha) >> 8;
                    let o_b = ((old.data & 0xFF) * n_alpha) >> 8;

                    old.data = ((o_r << 16) | (o_g << 8) | o_b) + ((n_r << 16) | (n_g << 8) | n_b);
                }
            }
        }
    }

    /// Roi区域颜色覆盖
    pub fn cover(&'a mut self, other: &ImageRoi) {
        for (self_row, other_row) in self.rows_mut().zip(other.rows()) {
            let len = cmp::min(self_row.len(), other_row.len());
            unsafe {
                ptr::copy(other_row.as_ptr(), self_row.as_mut_ptr(), len);
            }
        }
    }
}

/// 包含帧缓冲区的图像
pub struct ImageRef<'a> {
    w: i32,
    h: i32,
    data: &'a mut [Color],
    mode: Cell<RenderMode>,
}

impl<'a> ImageRef<'a> {
    /// 根据帧缓冲数据创建新图像
    pub fn from_data(width: i32, height: i32, data: &'a mut [Color]) -> Self {
        ImageRef {
            w: width,
            h: height,
            data,
            mode: Cell::new(RenderMode::Blend),
        }
    }

    /// 获得图像宽度
    pub fn width(&self) -> i32 {
        self.w
    }

    /// 获得图像高度
    pub fn height(&self) -> i32 {
        self.h
    }

    /// 根据矩形区域返回相应的Roi
    pub fn roi(&mut self, rect: &Rect) -> ImageRoi {
        ImageRoi {
            rect: *rect,
            w: self.w,
            data: &mut self.data,
        }
    }
}

impl<'a> Renderer for ImageRef<'a> {
    fn width(&self) -> u32 {
        self.w as u32
    }

    fn height(&self) -> u32 {
        self.h as u32
    }

    fn data(&self) -> &[Color] {
        self.data
    }

    fn data_mut(&mut self) -> &mut [Color] {
        self.data
    }

    fn sync(&mut self) -> bool {
        true
    }

    fn mode(&self) -> &Cell<RenderMode> {
        &self.mode
    }
}

/// 包含帧缓冲区的图像
#[derive(Clone)]
pub struct Image {
    /// 宽度
    w: i32,
    /// 高度
    h: i32,
    /// 像素数据
    data: Box<[Color]>,
    /// 渲染模式
    mode: Cell<RenderMode>,
}

impl Image {
    /// 创建默认图像
    pub fn new(width: i32, height: i32) -> Self {
        Image::from_color(width, height, Color::rgb(0, 0, 0))
    }

    /// 创建单一颜色的图像
    pub fn from_color(width: i32, height: i32, color: Color) -> Self {
        Image::from_data(
            width,
            height,
            vec![color; (width * height) as usize].into_boxed_slice(),
        )
    }

    /// 根据帧缓冲数据创建新图像
    pub fn from_data(width: i32, height: i32, data: Box<[Color]>) -> Self {
        Image {
            w: width,
            h: height,
            data,
            mode: Cell::new(RenderMode::Blend),
        }
    }

    pub fn from_path(path: &[u8]) -> Option<Self> {
        if let Ok(mut img) = image::load_from_memory(path) {
            // let img = img.resize(20, 20, image::imageops::FilterType::Gaussian);

            let (mut img_width, mut img_heigh) = img.dimensions();
            if img_width > SCREEN_WIDTH as u32 || img_heigh > SCREEN_HEIGHT as u32 {
                img = img.resize(
                    SCREEN_WIDTH as u32,
                    SCREEN_HEIGHT as u32,
                    image::imageops::FilterType::Gaussian,
                );
                (img_width, img_heigh) = img.dimensions();
            }

            let mut image = Image::new(img_width as i32, img_heigh as i32);
            for y in 0..img_heigh {
                for x in 0..img_width as u32 {
                    let pixel = img.get_pixel(x, y);
                    let offset = y * img_width + x;
                    // println!("Cursor pixel print x:{:?} y:{:?} rgba:{:?} {:?} {:?} {:?}", x, y, pixel[0], pixel[1], pixel[2], pixel[3]);
                    image.data[offset as usize] =
                        Color::rgba(pixel[0], pixel[1], pixel[2], pixel[3]);
                }
            }

            // println!(
            //     "[Info] Image created from path successfully,  width: {:?} height: {:?}",
            //     img_width, img_heigh
            // );

            return Some(image);
        } else {
            println!("[Error] Image created from path failed");
            return None;
        }
    }

    /// 返回图像宽度
    pub fn width(&self) -> i32 {
        self.w
    }

    /// 返回图像高度
    pub fn height(&self) -> i32 {
        self.h
    }

    /// 返回图像宽度和高度
    pub fn dimensions(&self) -> (i32, i32) {
        (self.w, self.h)
    }

    /// # 函数功能
    /// 根据矩形区域返回相应的Roi
    ///
    /// ## 参数值
    /// - rect: 矩形区域(相对位置)
    ///
    /// ## 返回值
    /// Roi对象
    pub fn roi(&mut self, rect: &Rect) -> ImageRoi {
        ImageRoi {
            rect: *rect,
            w: self.w,
            data: &mut self.data,
        }
    }

    /// 展示在桌面中央
    pub fn show_on_desktop(&self) {
        let xoffset = (SCREEN_WIDTH as i32 - self.width()) / 2;
        let yoffset = (SCREEN_HEIGHT as i32 - self.height()) / 2;
        let mut fb = File::open("/dev/fb0").expect("[Error] Unable to open framebuffer");
        for y in 0..self.height() {
            for x in 0..self.width() {
                let index: i32 = y * self.width() + x;
                let offset = ((y + yoffset) * SCREEN_WIDTH as i32 + x + xoffset) * 4;
                let color = &self.data[index as usize];
                println!(
                    "Image show print x:{:?} y:{:?} rgba:{:?} {:?} {:?} {:?}",
                    x,
                    y,
                    color.r(),
                    color.g(),
                    color.b(),
                    color.a()
                );
                fb.seek(SeekFrom::Start(offset as u64)).expect("error");
                fb.write(&self.data[index as usize].to_bgra_bytes())
                    .expect("error");
            }
        }
    }

    /// 改变图像大小
    pub fn resize(&self, w: u32, h: u32, resize_type: Type) -> Self {
        let mut dst_color = vec![Color { data: 0 }; w as usize * h as usize].into_boxed_slice();

        let src =
            unsafe { slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len() * 4) };

        let mut dst = unsafe {
            slice::from_raw_parts_mut(dst_color.as_mut_ptr() as *mut u8, dst_color.len() * 4)
        };

        let mut resizer = resize::new(
            self.w as usize,
            self.h as usize,
            w as usize,
            h as usize,
            resize::Pixel::RGBA,
            resize_type,
        );
        resizer.resize(&src, &mut dst);

        Image::from_data(w as i32, h as i32, dst_color)
    }
}

impl Renderer for Image {
    fn width(&self) -> u32 {
        self.w as u32
    }

    fn height(&self) -> u32 {
        self.h as u32
    }

    fn data(&self) -> &[Color] {
        &self.data
    }

    fn data_mut(&mut self) -> &mut [Color] {
        &mut self.data
    }

    fn mode(&self) -> &Cell<RenderMode> {
        &self.mode
    }

    fn sync(&mut self) -> bool {
        true
    }
}
