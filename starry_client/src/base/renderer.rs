use std::{cell::Cell, cmp};

use super::{
    color::Color,
    graphicspath::{GraphicsPath, PointType},
};

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

    #[allow(unused_variables)]
    /// TODO
    fn arc(&mut self, x0: i32, y0: i32, radius: i32, parts: u8, color: Color) {}
    #[allow(unused_variables)]
    /// TODO
    fn circle(&mut self, x0: i32, y0: i32, radius: i32, color: Color) {}
    #[allow(unused_variables)]
    /// TODO
    fn line4points(&mut self, argx1: i32, argy1: i32, argx2: i32, argy2: i32, color: Color) {}
    #[allow(unused_variables)]
    /// TODO
    /// # 函数功能
    /// 绘制指定颜色的一条线段
    /// 
    /// ## 参数
    /// - argx1: 起点x坐标
    /// - argy1: 起点y坐标
    /// - argx2: 终点x坐标
    /// - argy2: 终点y坐标
    /// - color:绘制颜色
    fn line(&mut self, argx1: i32, argy1: i32, argx2: i32, argy2: i32, color: Color) {}

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
    #[allow(unused_variables)]
    /// TODO
    fn char(&mut self, x: i32, y: i32, c: char, color: Color) {}
}
