use super::InputHandler;
use bitflags::bitflags;
use starry_client::base::event::{ButtonEvent, Event, MouseRelativeEvent};
use std::{fs::File, io::Read};

const MOUSE_DEVICE_PATH: &str = "/dev/char/psmouse";

bitflags! {
    /// 鼠标状态掩码
    #[derive(Default)]
    pub struct MouseFlags: u8 {
        /// 左键是否按下
        const LEFT_BUTTON = 0b0000_0001;

        /// 右键是否按下
        const RIGHT_BUTTON = 0b0000_0010;

        /// 滚轮是否按下
        const MIDDLE_BUTTON = 0b0000_0100;

        /// 鼠标是否启用
        const ALWAYS_ONE = 0b0000_1000;

        /// x轴移动方向是否为负
        const X_SIGN = 0b0001_0000;

        /// y轴移动方向是否为负
        const Y_SIGN = 0b0010_0000;

        /// x方向移动超出范围
        const X_OVERFLOW = 0b0100_0000;

        /// y方向移动超出范围
        const Y_OVERFLOW = 0b1000_0000;
    }
}

#[allow(dead_code)]
pub struct MouseInputHandler {
    /// 读取的文件
    file: File,
    /// 当前数据包序号
    packet_index: u8,
    /// 鼠标状态
    flags: MouseFlags,
    /// x轴相对移动
    dx: i16,
    /// y轴相对移动
    dy: i16,
    /// 移动系数/灵敏度
    scale: u8,
}

impl MouseInputHandler {
    pub fn new() -> Box<MouseInputHandler> {
        let file = File::open(MOUSE_DEVICE_PATH).expect("Fail to open mouse device");
        // println!("[Init] Mouse_Input_Handler created successfully!");
        Box::new(MouseInputHandler {
            flags: MouseFlags::empty(),
            packet_index: 0,
            file: file,
            dx: 0,
            dy: 0,
            scale: 1,
        })
    }
}

impl InputHandler for MouseInputHandler {
    fn get_listening_file(&mut self) -> &File {
        self.file.by_ref()
    }

    fn set_listening_file(&mut self, file: File) {
        self.file = file;
    }

    fn handle(&mut self, packet: u8) -> Vec<Event> {
        // println!("[Info] Mouse_Input_Handler handle packet {:?}", packet);
        /// 求补码
        fn sign_extend(value: u8) -> i16 {
            ((value as u16) | 0xFF00) as i16
        }

        let mut events: Vec<Event> = Vec::new();
        match self.packet_index {
            0 => {
                let flags = MouseFlags::from_bits_truncate(packet);
                if flags.contains(MouseFlags::ALWAYS_ONE) {
                    self.flags = flags;
                    events.push(ButtonEvent::new(packet).to_event());
                }
            }
            1 => {
                // 计算dx
                if self.flags.contains(MouseFlags::X_OVERFLOW) {
                    self.dx = 0;
                } else if self.flags.contains(MouseFlags::X_SIGN) {
                    self.dx = sign_extend(packet);
                } else {
                    self.dx = packet as i16;
                }
            }
            2 => {
                // 计算dy
                if self.flags.contains(MouseFlags::Y_OVERFLOW) {
                    self.dy = 0;
                } else if self.flags.contains(MouseFlags::Y_SIGN) {
                    self.dy = sign_extend(packet);
                } else {
                    self.dy = packet as i16;
                }

                // 传入移动事件
                events.push(
                    MouseRelativeEvent {
                        dx: self.dx as i32 * self.scale as i32,
                        dy: self.dy as i32 * self.scale as i32,
                    }
                    .to_event(),
                );
            }
            _ => unreachable!(),
        }
        self.packet_index = (self.packet_index + 1) % 3;
        return events;
    }
}
