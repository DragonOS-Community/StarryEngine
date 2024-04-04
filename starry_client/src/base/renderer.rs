use std::{cell::Cell, cmp};

use super::{
    color::Color,
    graphicspath::{GraphicsPath, PointType},
};

static FONT_ASSET : &[u8] = include_bytes!("../font/unifont.font");

#[derive(Clone, Copy, Debug)]
pub enum RenderMode {
    /// 颜色混合
    Blend,
    /// 颜色覆盖
    Overwrite,
}

/// 用于进行渲染
pub trait Renderer {
    /// 获取渲染窗口宽度
    fn width(&self) -> u32;

    /// 获取渲染窗口高度
    fn height(&self) -> u32;

    /// 获取帧缓冲数据
    fn data(&self) -> &[Color];

    /// 获取可变帧缓存数据
    fn data_mut(&mut self) -> &mut [Color];

    /// 同步数据
    fn sync(&mut self) -> bool;

    /// 获取/设置渲染模式
    fn mode(&self) -> &Cell<RenderMode>;

    /// # 函数功能
    /// 绘制指定位置的像素（左下角为原点）
    ///
    /// ## 参数
    /// - x: 像素x坐标
    /// - y: 像素y坐标
    /// - color: 像素颜色值
    fn pixel(&mut self, x: i32, y: i32, color: Color) {
        let replace = match self.mode().get() {
            RenderMode::Blend => false,
            RenderMode::Overwrite => true,
        };
        let w = self.width();
        let h = self.height();
        let data = self.data_mut();

        if x >= 0 && y >= 0 && x < w as i32 && y < h as i32 {
            let new_color = color.data;
            let alpha = (new_color >> 24) & 0xFF;
            let old_color = &mut data[y as usize * w as usize + x as usize].data;

            if alpha >= 255 || replace {
                *old_color = new_color;
            }
            // 颜色混合算法（效率更高的实现方法）
            else if alpha > 0 {
                let n_alpha = 255 - alpha;
                let rb = ((n_alpha * (*old_color & 0x00FF00FF))
                    + (alpha * (new_color & 0x00FF00FF)))
                    >> 8;
                let ag = (n_alpha * ((*old_color & 0xFF00FF00) >> 8))
                    + (alpha * (0x01000000 | ((new_color & 0x0000FF00) >> 8)));

                *old_color = (rb & 0x00FF00FF) | (ag & 0xFF00FF00);
            }
        }
    }

    /// TODO 注释补充
    fn arc(&mut self, x0: i32, y0: i32, radius: i32, parts: u8, color: Color) {
        let mut x = radius.abs();
        let mut y = 0;
        let mut err = 0;

        // https://github.com/rust-lang/rust-clippy/issues/5354
        while x >= y {
            if radius < 0 {
                if parts & 1 << 0 != 0 {
                    self.rect(x0 - x, y0 + y, x as u32, 1, color);
                }
                if parts & 1 << 1 != 0 {
                    self.rect(x0, y0 + y, x as u32 + 1, 1, color);
                }
                if parts & 1 << 2 != 0 {
                    self.rect(x0 - y, y0 + x, y as u32, 1, color);
                }
                if parts & 1 << 3 != 0 {
                    self.rect(x0, y0 + x, y as u32 + 1, 1, color);
                }
                if parts & 1 << 4 != 0 {
                    self.rect(x0 - x, y0 - y, x as u32, 1, color);
                }
                if parts & 1 << 5 != 0 {
                    self.rect(x0, y0 - y, x as u32 + 1, 1, color);
                }
                if parts & 1 << 6 != 0 {
                    self.rect(x0 - y, y0 - x, y as u32, 1, color);
                }
                if parts & 1 << 7 != 0 {
                    self.rect(x0, y0 - x, y as u32 + 1, 1, color);
                }
            } else if radius == 0 {
                self.pixel(x0, y0, color);
            } else {
                if parts & 1 << 0 != 0 {
                    self.pixel(x0 - x, y0 + y, color);
                }
                if parts & 1 << 1 != 0 {
                    self.pixel(x0 + x, y0 + y, color);
                }
                if parts & 1 << 2 != 0 {
                    self.pixel(x0 - y, y0 + x, color);
                }
                if parts & 1 << 3 != 0 {
                    self.pixel(x0 + y, y0 + x, color);
                }
                if parts & 1 << 4 != 0 {
                    self.pixel(x0 - x, y0 - y, color);
                }
                if parts & 1 << 5 != 0 {
                    self.pixel(x0 + x, y0 - y, color);
                }
                if parts & 1 << 6 != 0 {
                    self.pixel(x0 - y, y0 - x, color);
                }
                if parts & 1 << 7 != 0 {
                    self.pixel(x0 + y, y0 - x, color);
                }
            }

            y += 1;
            err += 1 + 2 * y;
            if 2 * (err - x) + 1 > 0 {
                x -= 1;
                err += 1 - 2 * x;
            }
        }
    }
    
    /// TODO 注释补充
    fn circle(&mut self, x0: i32, y0: i32, radius: i32, color: Color) {
        let mut x = radius.abs();
        let mut y = 0;
        let mut err = -radius.abs();

        match radius {
            radius if radius > 0 => {
                err = 0;
                while x >= y {
                    self.pixel(x0 - x, y0 + y, color);
                    self.pixel(x0 + x, y0 + y, color);
                    self.pixel(x0 - y, y0 + x, color);
                    self.pixel(x0 + y, y0 + x, color);
                    self.pixel(x0 - x, y0 - y, color);
                    self.pixel(x0 + x, y0 - y, color);
                    self.pixel(x0 - y, y0 - x, color);
                    self.pixel(x0 + y, y0 - x, color);

                    y += 1;
                    err += 1 + 2 * y;
                    if 2 * (err - x) + 1 > 0 {
                        x -= 1;
                        err += 1 - 2 * x;
                    }
                }
            }

            radius if radius < 0 => {
                while x >= y {
                    let lasty = y;
                    err += y;
                    y += 1;
                    err += y;
                    self.line4points(x0, y0, x, lasty, color);
                    if err >= 0 {
                        if x != lasty {
                            self.line4points(x0, y0, lasty, x, color);
                        }
                        err -= x;
                        x -= 1;
                        err -= x;
                    }
                }
            }
            _ => {
                self.pixel(x0, y0, color);
            }
        }
    }

    /// TODO 注释补充
    fn line4points(&mut self, x0: i32, y0: i32, x: i32, y: i32, color: Color) {
        //self.line(x0 - x, y0 + y, (x+x0), y0 + y, color);
        self.rect(x0 - x, y0 + y, x as u32 * 2 + 1, 1, color);
        if y != 0 {
            //self.line(x0 - x, y0 - y, (x+x0), y0-y , color);
            self.rect(x0 - x, y0 - y, x as u32 * 2 + 1, 1, color);
        }
    }
    
    /// # 函数功能
    /// 绘制指定颜色的一条线段
    ///
    /// ## 参数
    /// - argx1: 起点x坐标
    /// - argy1: 起点y坐标
    /// - argx2: 终点x坐标
    /// - argy2: 终点y坐标
    /// - color:绘制颜色
    /// TODO
    fn line(&mut self, argx1: i32, argy1: i32, argx2: i32, argy2: i32, color: Color) {
        let mut x = argx1;
        let mut y = argy1;

        let dx = if argx1 > argx2 {
            argx1 - argx2
        } else {
            argx2 - argx1
        };
        let dy = if argy1 > argy2 {
            argy1 - argy2
        } else {
            argy2 - argy1
        };

        let sx = if argx1 < argx2 { 1 } else { -1 };
        let sy = if argy1 < argy2 { 1 } else { -1 };

        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut err_tolerance;

        loop {
            self.pixel(x, y, color);

            if x == argx2 && y == argy2 {
                break;
            };

            err_tolerance = 2 * err;

            if err_tolerance > -dx {
                err -= dy;
                x += sx;
            }
            if err_tolerance < dy {
                err += dx;
                y += sy;
            }
        }
    }

    /// # 函数功能
    /// 绘制指定颜色的若干线段（首尾相连）
    ///
    /// ## 参数
    /// - points: 点集合
    /// - color: 绘制颜色
    fn lines(&mut self, points: &[[i32; 2]], color: Color) {
        if points.is_empty() {
        } else if points.len() == 1 {
            self.pixel(points[0][0], points[0][1], color);
        } else {
            for i in 0..points.len() - 1 {
                self.line(
                    points[i][0],
                    points[i][1],
                    points[i + 1][0],
                    points[i + 1][1],
                    color,
                );
            }
        }
    }

    /// # 函数功能
    /// 绘制一条指定颜色的几何路径
    ///
    /// ## 参数
    /// - graphicspath: 几何路径
    /// - color: 绘制颜色
    fn draw_path(&mut self, graphicspath: GraphicsPath, color: Color) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        for point in graphicspath.points {
            if let PointType::Connect = point.2 {
                self.line(x, y, point.0, point.1, color);
            }
            x = point.0;
            y = point.1;
        }
    }

    /// # 函数功能
    /// 绘制单一颜色的矩形
    ///
    /// ## 参数
    /// - x: 起始x坐标
    /// - y: 起始y坐标
    /// - w: 矩形宽度
    /// - h: 矩形高度
    /// - color: 矩形颜色
    fn rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
        let replace = match self.mode().get() {
            RenderMode::Blend => false,
            RenderMode::Overwrite => true,
        };
        let self_w = self.width();
        let self_h = self.height();

        let start_y = cmp::max(0, cmp::min(self_h as i32 - 1, y));
        let end_y = cmp::max(start_y, cmp::min(self_h as i32, y + h as i32));

        let start_x = cmp::max(0, cmp::min(self_w as i32 - 1, x));
        let end_x = cmp::max(start_x, cmp::min(self_w as i32, x + w as i32));
        let len_x = end_x - start_x;

        let alpha = (color.data >> 24) & 0xFF;

        if alpha >= 255 || replace {
            let data = self.data_mut();
            let data_ptr = data.as_mut_ptr();
            for y in start_y..end_y {
                let start = (y * self_w as i32 + start_x) as isize;
                let end = start + len_x as isize;
                for i in start..end {
                    unsafe {
                        *data_ptr.offset(i) = color;
                    }
                }
            }
        } else {
            for y in start_y..end_y {
                for x in start_x..end_x {
                    self.pixel(x, y, color);
                }
            }
        }
    }

    /// # 函数功能
    /// 将整个窗口填充单一颜色
    ///
    /// ## 参数
    /// - color: 窗口颜色
    fn set(&mut self, color: Color) {
        let data = self.data_mut();
        let data_ptr = data.as_mut_ptr();
        for i in 0..data.len() as isize {
            unsafe {
                *data_ptr.offset(i) = color;
            }
        }
    }

    /// # 函数功能
    /// 将整个窗口置黑
    fn clear(&mut self) {
        self.set(Color::rgb(0, 0, 0));
    }

    /// # 函数功能
    /// 获取指定坐标的像素颜色
    ///
    /// ## 参数
    /// - x: x坐标
    /// - y: y坐标
    ///
    /// ## 返回值
    /// 像素颜色
    fn get_pixel(&self, x: i32, y: i32) -> Color {
        let p = (self.width() as i32 * y + x) as usize;
        if p >= self.data().len() {
            println!("[Error] Client window get pixel overflow!");
            return Color::rgb(0, 0, 0);
        }
        return self.data()[p];
    }

    /// # 函数功能
    /// 在指定位置绘制字符
    /// 
    /// ## 参数
    /// - x: x坐标
    /// - y: y坐标
    /// - c: 待绘制的字符
    /// - color: 字符颜色
    fn char(&mut self, x: i32, y: i32, c: char, color: Color) {
        let mut offset = (c as usize) * 16;
        for row in 0..16 {
            let row_data = if offset < FONT_ASSET.len() {
                FONT_ASSET[offset]
            } else {
                0
            };

            for col in 0..8 {
                let pixel = (row_data >> (7 - col)) & 1;
                if pixel > 0 {
                    self.pixel(x + col, y + row, color);
                }
            }
            offset += 1;
        }
    }

    /// # 函数功能
    /// 在指定位置绘制一幅图像至帧缓冲区
    /// 
    /// ## 参数
    /// - start_x: 起始x坐标(左上角)
    /// - start_y: 起始y坐标(左上角)
    /// - w: 图像宽度
    /// - h: 图像高度
    /// - data: 图像数据
    fn image(&mut self, start_x: i32, start_y: i32, w: u32, h: u32, data: &[Color]) {
        match self.mode().get() {
            RenderMode::Blend => self.image_fast(start_x, start_y, w, h, data),
            RenderMode::Overwrite => self.image_opaque(start_x, start_y, w, h, data),
        }
    }
    

    /// # 函数功能
    /// 从指定行开始绘制一幅图像至帧缓冲区
    /// 
    /// ## 参数
    /// - start: 起始行数
    /// - image_data: 图像帧缓冲数据
    fn image_over(&mut self, start: i32, image_data: &[Color]) {
        let start = start as usize * self.width() as usize;
        let window_data = self.data_mut();
        let stop = cmp::min(start + image_data.len(), window_data.len());
        let end = cmp::min(image_data.len(), window_data.len() - start);

        window_data[start..stop].copy_from_slice(&image_data[..end]);
    }

    ///Display an image using non transparent method
    /// TODO 注释补充
    #[inline(always)]
    fn image_opaque(&mut self, start_x: i32, start_y: i32, w: u32, h: u32, image_data: &[Color]) {
        let w = w as usize;
        let mut h = h as usize;
        let width = self.width() as usize;
        let height = self.height() as usize;
        let start_x = start_x as usize;
        let start_y = start_y as usize;

        //check boundaries
        if start_x >= width || start_y >= height {
            return;
        }
        if h + start_y > height {
            h = height - start_y;
        }
        let window_data = self.data_mut();
        let offset = start_y * width + start_x;
        //copy image slices to window line by line
        for l in 0..h {
            let start = offset + l * width;
            let mut stop = start + w;
            let begin = l * w;
            let mut end = begin + w;
            //check boundaries
            if start_x + w > width {
                stop = (start_y + l + 1) * width - 1;
                end = begin + stop - start;
            }
            window_data[start..stop].copy_from_slice(&image_data[begin..end]);
        }
    }

    /// Speed improved, image can be outside of window boundary
    /// TODO 注释补充
    #[inline(always)]
    fn image_fast(&mut self, start_x: i32, start_y: i32, w: u32, h: u32, image_data: &[Color]) {
        let w = w as usize;
        let h = h as usize;
        let width = self.width() as usize;
        let start_x = start_x as usize;
        let start_y = start_y as usize;

        //simply return if image is outside of window
        if start_x >= width || start_y >= self.height() as usize {
            return;
        }
        let window_data = self.data_mut();
        let offset = start_y * width + start_x;

        //copy image slices to window line by line
        for l in 0..h {
            let start = offset + l * width;
            let mut stop = start + w;
            let begin = l * w;
            let end = begin + w;

            //check boundaries
            if start_x + w > width {
                stop = (start_y + l + 1) * width;
            }
            let mut k = 0;
            for i in begin..end {
                if i < image_data.len() {
                    let new = image_data[i].data;
                    let alpha = (new >> 24) & 0xFF;
                    if alpha > 0 && (start + k) < window_data.len() && (start + k) < stop {
                        let old = &mut window_data[start + k].data;
                        if alpha >= 255 {
                            *old = new;
                        } else {
                            let n_alpha = 255 - alpha;
                            let rb = ((n_alpha * (*old & 0x00FF00FF))
                                + (alpha * (new & 0x00FF00FF)))
                                >> 8;
                            let ag = (n_alpha * ((*old & 0xFF00FF00) >> 8))
                                + (alpha * (0x01000000 | ((new & 0x0000FF00) >> 8)));

                            *old = (rb & 0x00FF00FF) | (ag & 0xFF00FF00);
                        }
                    }
                    k += 1;
                }
            }
        }
    }
    
}
