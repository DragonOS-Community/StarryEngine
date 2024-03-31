use std::{
    cell::Cell,
    fs::File,
    io::{Seek, SeekFrom, Write},
};

use crate::base::{
    color::Color,
    renderer::{RenderMode, Renderer},
};

// TODO: 读帧缓冲设备属性
/// 屏幕宽度
const SCREEN_WIDTH: usize = 1440;
/// 屏幕高度
#[allow(dead_code)]
const SCREEN_HEIGHT: usize = 900;

#[allow(dead_code)]
pub struct Window {
    /// 窗口左上角的x坐标
    x: i32,
    /// 窗口左上角的y坐标
    y: i32,
    /// 窗口的宽度
    w: u32,
    /// 窗口的高度
    h: u32,
    /// 窗口的标题
    title: String,
    /// TODO
    // window_async: bool,
    /// 窗口是否大小可变
    resizable: bool,
    /// 窗口的渲染模式
    mode: Cell<RenderMode>,
    // TODO
    // file_opt: Option<File>,
    // TODO: 改定长数组
    // data_opt: Option<& 'static mut [Color]>,
    /// 窗口的渲染数据
    data_opt: Option<Box<[Color]>>,
}

impl Renderer for Window {
    fn width(&self) -> u32 {
        self.w
    }

    fn height(&self) -> u32 {
        self.h
    }

    fn data(&self) -> &[Color] {
        self.data_opt.as_ref().unwrap()
    }

    fn data_mut(&mut self) -> &mut [Color] {
        self.data_opt.as_mut().unwrap()
    }

    /// TODO
    fn sync(&mut self) -> bool {
        true
    }

    fn mode(&self) -> &Cell<RenderMode> {
        &self.mode
    }
}

#[allow(dead_code)]
impl Window {
    /// TODO: 接收flags
    pub fn new(x: i32, y: i32, w: u32, h: u32, title: &str) -> Self {
        Window {
            x,
            y,
            w,
            h,
            title: title.to_string(),
            // window_async: false,
            resizable: false,
            mode: Cell::new(RenderMode::Blend),
            // file_opt: None,
            data_opt: Some(vec![Color::rgb(0, 0, 0); (w * h) as usize].into_boxed_slice()),
        }

        // TODO: 与服务器通信
    }

    /// # 函数功能
    /// 同步数据至系统帧缓冲
    pub fn sync(&self) {
        let mut fb = File::open("/dev/fb0").expect("Unable to open framebuffer");

        for y in 0..self.height() as i32 {
            for x in 0..self.width() as i32 {
                let pixel = self.get_pixel(x, y);
                let offset = (((y + self.y()) * SCREEN_WIDTH as i32) + x + self.x()) * 4;
                // 写缓冲区
                fb.seek(SeekFrom::Start(offset as u64))
                    .expect("Unable to seek framebuffer");
                fb.write_all(&pixel.to_rgba_bytes())
                    .expect("Unable to write framebuffer");
            }
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }
}
