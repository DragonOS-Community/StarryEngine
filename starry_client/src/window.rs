use std::{cell::Cell, fs::File, io::{Seek, SeekFrom, Write}};

use crate::base::{color::Color, renderer::{RenderMode, Renderer}};

// TODO: 读帧缓冲设备属性
/// 屏幕宽度
const SCREEN_WIDTH: usize = 1440;
/// 屏幕高度
#[allow(dead_code)]
const SCREEN_HEIGHT: usize = 900;

/// 客户端的窗口类，与服务端的窗口对象一一对应  
/// 一般来说客户端应用程序不直接使用该类，而通过Toolkit库间接使用
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

    fn data_mut(&mut self) -> &mut [Color]{
        self.data_opt.as_mut().unwrap()
    }

    fn sync(&mut self) -> bool {
        let mut fb = File::open("/dev/fb0").expect("Unable to open framebuffer");

        for y in 0..self.height() as i32 {
            for x in 0..self.width() as i32 {
                let pixel = self.get_pixel(x, y);
                let offset =  (((y + self.y()) * SCREEN_WIDTH as i32) + x + self.x()) * 4;
                // 写缓冲区
                fb.seek(SeekFrom::Start(offset as u64)).expect("Unable to seek framebuffer");
                fb.write_all(&pixel.to_bgra_bytes()).expect("Unable to write framebuffer");
            }
        }
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
            x: x,
            y: y,
            w: w,
            h: h,
            title: title.to_string(),
            // window_async: false,
            resizable: false,
            mode: Cell::new(RenderMode::Blend),
            // file_opt: None,
            data_opt: Some(vec!(Color::rgb(0, 0, 0); (w * h) as usize).into_boxed_slice()),
        }

        // TODO: 与服务器通信 
    }

    /// 返回窗口x坐标
    pub fn x(&self) -> i32 {
        self.x
    }

    /// 返回窗口y坐标
    pub fn y(&self) -> i32 {
        self.y
    }

    /// 返回窗口标题
    pub fn title(&self) -> String {
        self.title.clone()
    }

    /// 改变窗口的位置
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    /// 改变窗口的大小
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.w = width;
        self.h = height;
    }

    /// 改变窗口标题
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }
}