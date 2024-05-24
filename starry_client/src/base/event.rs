#[derive(Copy, Clone, Debug)]
pub enum EventOption {
    /// 按键事件
    Key(KeyEvent),
    /// 鼠标相对移动事件
    MouseRelative(MouseRelativeEvent),
    /// 鼠标按键事件
    Button(ButtonEvent),
    /// 窗口位置移动事件
    WindowMove(WindowMoveEvent),
    /// 窗口大小改变事件
    WindowResize(WindowResizeEvent),
    /// 未知事件
    Unknown(Event),
    /// 空事件
    None,
}

pub const EVENT_NONE: i64 = 0;
pub const EVENT_KEY: i64 = 1;
pub const EVENT_MOUSE_RELATIVE: i64 = 2;
pub const EVENT_BUTTON: i64 = 3;
pub const EVENT_MOUSE_UPDATE: i64 = 4;
pub const EVENT_WINDOW_MOVE: i64 = 5;
pub const EVENT_WINDOW_RESIZE: i64 = 6;

/// 通用事件
#[derive(Copy, Clone, Debug)]
pub struct Event {
    pub code: i64,
    pub a: i64,
    pub b: i64,
}

impl Event {
    pub fn new() -> Event {
        Event {
            code: 0,
            a: 0,
            b: 0,
        }
    }

    pub fn to_option(self) -> EventOption {
        match self.code {
            EVENT_NONE => EventOption::None,
            EVENT_KEY => EventOption::Key(KeyEvent::from_event(self)),
            EVENT_MOUSE_RELATIVE => {
                EventOption::MouseRelative(MouseRelativeEvent::from_event(self))
            }
            EVENT_BUTTON => EventOption::Button(ButtonEvent::from_event(self)),
            _ => EventOption::Unknown(self),
        }
    }
}

/// 键盘按键事件
#[derive(Copy, Clone, Debug)]
pub struct KeyEvent {
    /// 按键字符
    pub character: char,
    /// 按键扫描码
    pub scancode: u8,
    /// 是否按下
    pub pressed: bool,
}

impl KeyEvent {
    /// 转换为Event
    pub fn to_event(&self) -> Event {
        Event {
            code: EVENT_KEY,
            a: self.character as i64,
            b: self.scancode as i64 | (self.pressed as i64) << 8,
        }
    }

    /// 从Event转换为KeyEvent
    pub fn from_event(event: Event) -> KeyEvent {
        KeyEvent {
            character: char::from_u32(event.a as u32).unwrap_or('\0'),
            scancode: event.b as u8,
            pressed: event.b & (1 << 8) == (1 << 8),
        }
    }
}

/// 鼠标相对移动事件
#[derive(Copy, Clone, Debug)]
pub struct MouseRelativeEvent {
    /// x轴向上的相对运动
    pub dx: i32,
    /// y轴向上的相对运动
    pub dy: i32,
}

impl MouseRelativeEvent {
    /// 转换为Event
    pub fn to_event(&self) -> Event {
        Event {
            code: EVENT_MOUSE_RELATIVE,
            a: self.dx as i64,
            b: self.dy as i64,
        }
    }

    /// 从Event转换为MouseRelativeEvent
    pub fn from_event(event: Event) -> MouseRelativeEvent {
        MouseRelativeEvent {
            dx: event.a as i32,
            dy: event.b as i32,
        }
    }
}

/// TODO: 按键松开事件
/// 鼠标按键事件
#[derive(Clone, Copy, Debug)]
pub struct ButtonEvent {
    /// 左键是否按下
    pub left: bool,
    /// 右键是否按下
    pub right: bool,
    /// 中键是否按下
    pub middle: bool,
}

impl ButtonEvent {
    pub fn new(byte: u8) -> Self {
        ButtonEvent {
            left: byte & (1 << 0) == 1,
            middle: byte & (1 << 1) == 1,
            right: byte & (1 << 2) == 1,
        }
    }

    /// 转换为Event
    pub fn to_event(&self) -> Event {
        Event {
            code: EVENT_BUTTON,
            a: self.left as i64 | (self.middle as i64) << 1 | (self.right as i64) << 2,
            b: 0,
        }
    }

    /// 从Event转换为ButtonEvent
    pub fn from_event(event: Event) -> ButtonEvent {
        ButtonEvent {
            left: event.a & (1 << 0) == 1,
            middle: event.a & (1 << 1) == 1,
            right: event.a & (1 << 2) == 1,
        }
    }
}

/// 鼠标位置更新事件
pub struct MouseUpdateEvent {
    /// 更新后鼠标位置x坐标
    pub x: i32,
    /// 更新后鼠标位置y坐标
    pub y: i32,
}

impl MouseUpdateEvent {
    /// 转换为Event
    pub fn to_event(&self) -> Event {
        Event {
            code: EVENT_MOUSE_UPDATE,
            a: self.x as i64,
            b: self.y as i64,
        }
    }

    /// 从Event转换为MouseUpdateEvent
    pub fn from_event(event: Event) -> MouseUpdateEvent {
        MouseUpdateEvent {
            x: event.a as i32,
            y: event.b as i32,
        }
    }
}

/// 窗口位置移动事件
#[derive(Copy, Clone, Debug)]
pub struct WindowMoveEvent {
    pub x: i32,
    pub y: i32,
}

impl WindowMoveEvent {
    pub fn to_event(&self) -> Event {
        Event {
            code: EVENT_WINDOW_MOVE,
            a: self.x as i64,
            b: self.y as i64,
        }
    }

    pub fn from_event(event: Event) -> WindowMoveEvent {
        WindowMoveEvent {
            x: event.a as i32,
            y: event.b as i32,
        }
    }
}

/// 窗口改变大小事件
#[derive(Copy, Clone, Debug)]
pub struct WindowResizeEvent {
    pub width: u32,
    pub height: u32,
}

impl WindowResizeEvent {
    pub fn to_event(&self) -> Event {
        Event {
            code: EVENT_WINDOW_RESIZE,
            a: self.width as i64,
            b: self.height as i64,
        }
    }

    pub fn from_event(event: Event) -> WindowResizeEvent {
        WindowResizeEvent {
            width: event.a as u32,
            height: event.b as u32,
        }
    }
}