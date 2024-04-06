use std::{
    fs::File,
    io::Read,
    sync::{Arc, RwLock},
};

use starry_client::base::event::Event;

use self::inputs::MouseInputHandler;

use super::window_manager::window_manager;

pub mod inputs;

static mut INPUT_MANAGER: Option<Arc<InputManager>> = None;

pub fn input_manager() -> Option<Arc<InputManager>> {
    unsafe { INPUT_MANAGER.clone() }
}

/// 输入管理器
#[allow(dead_code)]
pub struct InputManager {
    /// 数据锁
    data: RwLock<InputManagerData>,
}

pub struct InputManagerData {
    /// 轮询的文件数组
    handlers: Vec<Box<dyn InputHandler>>,
}

impl InputManager {
    /// 创建输入管理器
    pub fn new() {
        let mut input_handlers = Vec::new();
        // TODO: 通过设备检测添加
        input_handlers.push(MouseInputHandler::new() as Box<dyn InputHandler>);
        // TODO: 处理键盘输入
        let input_manager = InputManager {
            data: RwLock::new(InputManagerData {
                handlers: input_handlers,
            }),
        };

        unsafe {
            INPUT_MANAGER = Some(Arc::new(input_manager));
        }

        // println!("[Init] Input_Manager created successfully!");
    }

    /// 轮询所有输入设备
    pub fn polling_all(&self) {
        // println!("[Info] Input_Manager polling all");
        let mut guard = self.data.write().unwrap();
        for handle in guard.handlers.iter_mut() {
            handle.polling();
        }
    }
}

/// 输入处理器需要实现的特性
#[allow(dead_code)]
pub trait InputHandler {
    /// 获得监听的文件
    fn get_listening_file(&mut self) -> &File;

    /// 设置监听的文件
    fn set_listening_file(&mut self, file: File);

    /// 处理字节数据
    fn handle(&mut self, packet: u8) -> Vec<Event>;

    /// 轮询文件
    fn polling(&mut self) {
        let mut buf: [u8; 1024] = [0; 1024];
        // TODO: 错误信息提示相应文件路径
        let count = self
            .get_listening_file()
            .read(&mut buf)
            .expect("[Error] Fail to polling file");
        // println!("[Info] Input_Handler polling read {:?} bytes", count);
        for i in 0..count {
            let events = self.handle(buf[i]);
            window_manager().unwrap().send_events(events);
        }
    }
}
