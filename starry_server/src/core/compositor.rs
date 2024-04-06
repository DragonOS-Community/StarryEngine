use std::{
    fs::File,
    io::{Seek, SeekFrom, Write},
    sync::{Arc, RwLock},
};

use starry_client::base::renderer::Renderer;

use crate::base::rect::Rect;

use super::{starry_server, window_manager::window_manager, SCREEN_WIDTH};

static mut COMPOSITOR: Option<Arc<Compositor>> = None;

/// 获得合成渲染器实例
pub fn compositor() -> Option<Arc<Compositor>> {
    unsafe { COMPOSITOR.clone() }
}

#[allow(dead_code)]
/// 合成渲染器
pub struct Compositor {
    /// 数据锁
    data: RwLock<CompositorData>,
}

pub struct CompositorData {
    /// 待重绘的窗口
    redraws: Vec<Rect>,

    fb_file: File,
}

#[allow(dead_code)]
impl Compositor {
    /// 创建合成渲染器
    pub fn new() {
        let compositor = Compositor {
            data: RwLock::new(CompositorData {
                redraws: Vec::new(),
                fb_file: File::open("/dev/fb0").expect("[Error] Unable to open framebuffer"),
            }),
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

        let mut window_manager_guard = window_manager.data.write().unwrap();
        let mut compositor_guard = self.data.write().unwrap();
        let mut server_guard = server.data.write().unwrap();

        let mut total_redraw_rect_opt: Option<Rect> = None;
        for original_rect in compositor_guard.redraws.drain(..) {
            // 更新重绘的总矩形区域
            if !original_rect.is_empty() {
                total_redraw_rect_opt = match total_redraw_rect_opt {
                    Some(total_redraw) => Some(total_redraw.container(&original_rect)),
                    None => Some(original_rect),
                }
            }

            let mut cursors = server_guard.cursors.clone();
            // 遍历所有显示窗口
            for display in server_guard.displays.iter_mut() {
                let rect = original_rect.intersection(&display.screen_rect());
                if !rect.is_empty() {
                    // TODO: 填充默认颜色

                    // 倒序渲染所有窗口
                    let len = window_manager_guard.zbuffer.len();
                    for index in (0..len).rev() {
                        let entry = window_manager_guard.zbuffer.get(index).unwrap();
                        let _id = entry.0;
                        let index = entry.2;
                        if let Some(window) = window_manager_guard.windows.get_mut(&index) {
                            // TODO: 渲染窗口标题

                            // 渲染窗体
                            window.draw(display, &rect);
                        }
                    }
                }

                let cursor_intersect = rect.intersection(&cursor_rect);
                if !cursor_intersect.is_empty() {
                    if let Some(cursor) = cursors.get_mut(&window_manager_guard.cursor_i) {
                        display.roi(&cursor_intersect).blend(&cursor.roi(
                            &cursor_intersect.offset(-cursor_rect.left(), -cursor_rect.top()),
                        ));
                    }
                }
            }
        }

        // println!("[Info] Compositor calculate total redraw rect done!");

        // TODO
        let mut fb = &compositor_guard.fb_file;

        if let Some(total_redraw_rect) = total_redraw_rect_opt {
            for display in server_guard.displays.iter_mut() {
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
        let mut guard = self.data.write().unwrap();
        let mut push = true;

        for rect in guard.redraws.iter_mut() {
            let container = rect.container(&rect);
            if container.area() <= rect.area() + rect.area() {
                *rect = container;
                push = false;
            }
        }

        if push {
            guard.redraws.push(rect);
        }
    }
}
