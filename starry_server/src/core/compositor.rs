use std::{
    cell::RefCell,
    fs::File,
    io::{Seek, SeekFrom, Write},
    sync::Arc,
};

use starry_client::base::renderer::Renderer;

use crate::base::rect::Rect;

use super::{starry_server, window_manager::window_manager, SCREEN_WIDTH};

static mut COMPOSITOR: Option<Arc<Compositor>> = None;

const FB_FILE_PATH: &str = "/dev/fb0";

/// 获得合成渲染器实例
pub fn compositor() -> Option<Arc<Compositor>> {
    unsafe { COMPOSITOR.clone() }
}

#[allow(dead_code)]
/// 合成渲染器
pub struct Compositor {
    /// 待重绘的矩形区域
    redraws: RefCell<Vec<Rect>>,
    /// 帧缓冲文件
    fb_file: RefCell<File>,
}

#[allow(dead_code)]
impl Compositor {
    /// 创建合成渲染器
    pub fn new() {
        let compositor = Compositor {
            redraws: RefCell::new(Vec::new()),
            fb_file: RefCell::new(
                File::open(FB_FILE_PATH).expect("[Error] Compositor failed to open fb file"),
            ),
        };

        unsafe {
            COMPOSITOR = Some(Arc::new(compositor));
        }

        // println!("[Init] Compositor created successfully!");
    }

    /// TODO
    /// 重绘所有请求的窗口
    pub fn redraw_all(&self) {
        // println!("[Info] Compositor begin redraw_all...");
        let window_manager = window_manager().unwrap();
        let server = starry_server().unwrap();
        let cursor_rect = window_manager.cursor_rect();

        // 对窗口排序
        window_manager.rezbuffer();

        let mut total_redraw_rect_opt: Option<Rect> = None;
        for original_rect in self.redraws.borrow_mut().drain(..) {
            // 更新重绘的总矩形区域
            if !original_rect.is_empty() {
                total_redraw_rect_opt = match total_redraw_rect_opt {
                    Some(total_redraw) => Some(total_redraw.container(&original_rect)),
                    None => Some(original_rect),
                }
            }

            // 遍历所有显示窗口
            for display in server.displays.borrow_mut().iter_mut() {
                let rect = original_rect.intersection(&display.screen_rect());
                if !rect.is_empty() {
                    // TODO: 填充默认颜色

                    // 倒序渲染所有窗口
                    let zbuffer = window_manager.zbuffer.borrow_mut();
                    let len = zbuffer.len();
                    for index in (0..len).rev() {
                        let entry = zbuffer.get(index).unwrap();
                        let _id = entry.0;
                        let index = entry.2;
                        let mut windows = window_manager.windows.borrow_mut();
                        if let Some(window) = windows.get_mut(&index) {
                            // TODO: 渲染窗口标题

                            // 渲染窗体
                            window.draw(display, &rect);
                        }
                    }
                }

                let cursor_intersect = rect.intersection(&cursor_rect);
                if !cursor_intersect.is_empty() {
                    if let Some(cursor) = server
                        .cursors
                        .borrow_mut()
                        .get_mut(&window_manager.cursor_i.get())
                    {
                        display.roi(&cursor_intersect).blend(&cursor.roi(
                            &cursor_intersect.offset(-cursor_rect.left(), -cursor_rect.top()),
                        ));
                    }
                }
            }
        }

        // println!("[Info] Compositor calculate total redraw rect done!");

        // TODO
        let mut fb = self.fb_file.borrow_mut();

        if let Some(total_redraw_rect) = total_redraw_rect_opt {
            for display in server.displays.borrow_mut().iter_mut() {
                let display_redraw = total_redraw_rect.intersection(&display.screen_rect());
                if !display_redraw.is_empty() {
                    for y in 0..display_redraw.height() {
                        for x in 0..display_redraw.width() {
                            let pixel = display.image.get_pixel(
                                x + display_redraw.left() - display.x,
                                y + display_redraw.top() - display.y,
                            );
                            let offset = (((y + display_redraw.top()) * SCREEN_WIDTH as i32)
                                + x
                                + display_redraw.left())
                                * 4;
                            fb.seek(SeekFrom::Start(offset as u64))
                                .expect("Unable to seek framebuffer");
                            fb.write_all(&pixel.to_bgra_bytes())
                                .expect("Unable to write framebuffer");
                        }
                    }
                }
            }
        }
    }

    /// 窗口请求重绘
    pub fn request_redraw(&self, rect: Rect) {
        // println!("[Info] Compositor request redraw rect {:?}", rect);
        let mut push = true;

        for rect in self.redraws.borrow_mut().iter_mut() {
            let container = rect.container(&rect);
            if container.area() <= rect.area() + rect.area() {
                *rect = container;
                push = false;
            }
        }

        if push {
            self.redraws.borrow_mut().push(rect);
        }
    }
}
