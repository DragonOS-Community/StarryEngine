use std::{cell::RefCell, fs::File, io::Read, sync::Arc};

use starry_client::base::event::Event;

use self::inputs::{KeyboardInputHandler, MouseInputHandler};

use super::window_manager::window_manager;

pub mod inputs;

static mut INPUT_MANAGER: Option<Arc<InputManager>> = None;

pub fn input_manager() -> Option<Arc<InputManager>> {
    unsafe { INPUT_MANAGER.clone() }
}

/// 输入管理器
#[allow(dead_code)]
pub struct InputManager {
    /// 轮询的文件数组
    handlers: RefCell<Vec<Box<dyn InputHandler>>>,
}

impl InputManager {
    /// 创建输入管理器
    pub fn new() {
        let mut input_handlers = Vec::new();
        input_handlers.push(MouseInputHandler::new() as Box<dyn InputHandler>);
        input_handlers.push(KeyboardInputHandler::new() as Box<dyn InputHandler>);
        let input_manager = InputManager {
            handlers: RefCell::new(input_handlers),
        };

        unsafe {
            INPUT_MANAGER = Some(Arc::new(input_manager));
        }

        // println!("[Init] Input_Manager created successfully!");
    }

    /// 驱动所有输入处理器
    pub fn polling_all(&self) {
        // println!("[Info] Input_Manager polling all");
        for handle in self.handlers.borrow_mut().iter_mut() {
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
        let mut file = self.get_listening_file();
        let result = file.read(&mut buf);

        if result.is_err() {
            println!("[Error] Filed to polling file {:?}", file);
        } else {
            let count = result.ok().unwrap();
            // println!("[Info] Input_Handler polling read {:?} bytes", count);
            for i in 0..count {
                let events = self.handle(buf[i]);
                window_manager().unwrap().send_events(events);
            }
        }
    }
}
