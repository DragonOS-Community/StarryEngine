use std::{collections::BTreeMap, rc::Rc, sync::{Arc, RwLock}};

use crate::{base::{display::Display, image::Image}, config::Config};

use self::{
    compositor::{compositor, Compositor},
    input::{input_manager, InputManager},
    window_manager::{window_manager, CursorKind, WindowManager},
};

pub mod compositor;
pub mod input;
pub mod window_manager;

/// 屏幕宽度
pub const SCREEN_WIDTH: usize = 1440;
/// 屏幕高度
#[allow(dead_code)]
pub const SCREEN_HEIGHT: usize = 900;

static DESKTOP_BG: &[u8] = include_bytes!("../asset/desktop_bg.png");
static CURSOR_NORMAL: &[u8] = include_bytes!("../asset/cursor_normal.png");

static mut STARRY_SERVER: Option<Arc<StarryServer>> = None;

pub fn starry_server() -> Option<Arc<StarryServer>> {
    unsafe { STARRY_SERVER.clone() }
}

/// 图形系统服务器
pub struct StarryServer {
    /// 数据锁
    data: RwLock<StarryServerData>,
}

pub struct StarryServerData {
    /// 窗口数组
    pub displays: Vec<Display>,
    pub config: Rc<Config>,
    pub cursors: BTreeMap<CursorKind, Image>,
}

impl StarryServer {
    /// 创建图形服务器
    pub fn new(config: Rc<Config>, displays: Vec<Display>){
        let mut cursors = BTreeMap::new();
        cursors.insert(CursorKind::None, Image::new(0, 0));
        cursors.insert(CursorKind::Normal, Image::from_path(CURSOR_NORMAL).unwrap_or(Image::new(10, 10)));        // cursors.insert(CursorKind::BottomLeftCorner, Image::from_path_scale(&config.bottom_left_corner, scale).unwrap_or(Image::new(0, 0)));
        // cursors.insert(CursorKind::BottomRightCorner, Image::from_path_scale(&config.bottom_right_corner, scale).unwrap_or(Image::new(0, 0)));
        // cursors.insert(CursorKind::BottomSide, Image::from_path_scale(&config.bottom_side, scale).unwrap_or(Image::new(0, 0)));
        // cursors.insert(CursorKind::LeftSide, Image::from_path_scale(&config.left_side, scale).unwrap_or(Image::new(0, 0)));
        // cursors.insert(CursorKind::RightSide, Image::from_path_scale(&config.right_side, scale).unwrap_or(Image::new(0, 0)));

        let server = StarryServer {
            data: RwLock::new(StarryServerData {
                displays: displays,
                config: Rc::clone(&config),
                cursors: cursors,
            }),
        };


        unsafe {
            STARRY_SERVER = Some(Arc::new(server));
        }

        // println!("[Init] Starry_Server created successfully!");
    }

    /// 开启主循环
    pub fn run(&self) {
        WindowManager::new();
        Compositor::new();
        InputManager::new();

        // TODO 临时在此创建桌面窗口
        window_manager().unwrap().window_new(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, "", "".to_string(), DESKTOP_BG);
        
        // println!("[Init] Starry_Server start main loop!");
        loop {
            input_manager().unwrap().polling_all();
            window_manager().unwrap().handle_all_events();
            compositor().unwrap().redraw_all();
            // std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
}
