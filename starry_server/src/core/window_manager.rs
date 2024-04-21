use std::{
    cell::{Cell, RefCell},
    cmp,
    collections::{BTreeMap, VecDeque},
    sync::Arc,
};

use starry_client::base::event::{Event, EventOption, MouseRelativeEvent, MouseUpdateEvent};

use crate::{
    base::{
        rect::Rect,
        window::{Window, WindowZOrderMode},
    },
    core::{SCREEN_HEIGHT, SCREEN_WIDTH},
};

use super::{compositor::compositor, starry_server};

static mut WINDOW_MANAGER: Option<Arc<WindowManager>> = None;

pub fn window_manager() -> Option<Arc<WindowManager>> {
    unsafe { WINDOW_MANAGER.clone() }
}

/// 鼠标样式
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CursorKind {
    /// 无/不显示
    None,
    /// 默认状态
    Normal,
    /// 左下角
    BottomLeftCorner,
    /// 右下角
    BottomRightCorner,
    /// 下边界
    BottomSide,
    /// 左边界
    LeftSide,
    /// 右边界
    RightSide,
}

/// 窗口管理器
#[allow(dead_code)]
pub struct WindowManager {
    /// 下一个窗口的id值
    next_id: Cell<isize>,
    /// TODO
    _hover: RefCell<Option<usize>>,
    /// 窗口顺序
    pub order: RefCell<VecDeque<usize>>,
    /// 窗口顺序信息(下标index，模式，窗口id)
    pub zbuffer: RefCell<Vec<(usize, WindowZOrderMode, usize)>>,
    /// 窗口字典
    pub windows: RefCell<BTreeMap<usize, Window>>,

    /// 鼠标x坐标
    pub cursor_x: Cell<i32>,
    /// 鼠标y坐标
    pub cursor_y: Cell<i32>,
    /// 鼠标状态
    pub cursor_i: Cell<CursorKind>,

    /// 待处理的事件数组
    events: RefCell<Vec<Event>>,
}

impl WindowManager {
    /// 创建窗口管理器
    pub fn new() {
        let window_manager = WindowManager {
            next_id: Cell::new(0),
            _hover: RefCell::new(None),
            order: RefCell::new(VecDeque::new()),
            zbuffer: RefCell::new(Vec::new()),
            windows: RefCell::new(BTreeMap::new()),
            cursor_x: Cell::new(SCREEN_WIDTH as i32 / 2),
            cursor_y: Cell::new(SCREEN_HEIGHT as i32 / 2),
            cursor_i: Cell::new(CursorKind::Normal),
            events: RefCell::new(Vec::new()),
        };

        unsafe {
            WINDOW_MANAGER = Some(Arc::new(window_manager));
        }

        // println!("[Init] Window_Manager created successfully!");
    }

    /// # 函数功能
    /// 新建窗口
    ///
    /// ## 参数
    /// - x: 窗口左上角x坐标
    /// - y: 窗口左上角y坐标
    /// - width: 窗口宽度
    /// - height: 窗口高度
    /// - flags: 窗口属性
    /// - title: 窗口标题
    ///
    /// ## 返回值
    /// 窗口id
    pub fn window_new(
        &self,
        mut x: i32,
        mut y: i32,
        width: i32,
        height: i32,
        _flags: &str,
        _title: String,
        image_path: &[u8],
    ) {
        let compositor = compositor().unwrap();

        let id = self.next_id.get() as usize; // 新窗口的id
        self.next_id.set(self.next_id.get() + 1);

        if self.next_id.get() < 0 {
            self.next_id.set(1);
        }

        if x < 0 && y < 0 {
            x = cmp::max(0, (SCREEN_WIDTH as i32 - width) / 2);
            y = cmp::max(0, (SCREEN_HEIGHT as i32 - height) / 2);
        }

        // TODO 传入正确的scale
        // TODO 传入title
        let window = Window::new(x, y, width, height, 1, image_path);

        // TODO 处理flags

        // TODO 重绘title_rect
        compositor.request_redraw(window.rect());

        match window.zorder {
            WindowZOrderMode::Front | WindowZOrderMode::Normal => {
                self.order.borrow_mut().push_front(id);
            }
            WindowZOrderMode::Back => {
                self.order.borrow_mut().push_back(id);
            }
        }

        self.windows.borrow_mut().insert(id, window);

        // 确保鼠标正确显示
        let mouse_update_event = MouseUpdateEvent {
            x: self.cursor_x.get(),
            y: self.cursor_y.get(),
        };

        self.handle_mouse_update_event(mouse_update_event);
    }

    /// 发送事件
    pub fn send_event(&self, event: Event) {
        self.events.borrow_mut().push(event);
    }

    /// 发送事件数组
    pub fn send_events(&self, mut events: Vec<Event>) {
        self.events.borrow_mut().append(&mut events);
    }

    /// 处理所有事件
    pub fn handle_all_events(&self) {
        while let Some(event) = self.events.borrow_mut().pop() {
            self.handle_event(event);
        }
    }

    /// # 函数功能
    /// 处理事件
    ///
    /// ## 参数
    /// 事件对象
    pub fn handle_event(&self, event_union: Event) {
        // println!("[Info] Window_Manager handle event {:?}", event_union.to_option());
        match event_union.to_option() {
            EventOption::MouseRelative(event) => self.handle_mouse_relative_event(event),
            EventOption::Button(_event) => {}
            unknown => println!("[Error] Unexpected event: {:?}", unknown),
        }
    }

    /// 处理鼠标相对移动事件
    pub fn handle_mouse_relative_event(&self, event: MouseRelativeEvent) {
        // TODO: 将事件传递给窗口，同时考虑窗口对鼠标位置的影响

        let max_x: i32 = SCREEN_WIDTH as i32;
        let max_y: i32 = SCREEN_HEIGHT as i32;
        let cursor_rect = self.cursor_rect();

        //防止鼠标出界
        let x = cmp::max(
            0,
            cmp::min(max_x - cursor_rect.width(), self.cursor_x.get() + event.dx),
        );
        let y = cmp::max(
            0,
            cmp::min(max_y - cursor_rect.height(), self.cursor_y.get() - event.dy),
        ); // 原点在左上角，向上为负

        self.handle_mouse_update_event(MouseUpdateEvent { x, y });
    }

    /// 处理鼠标移动事件
    pub fn handle_mouse_update_event(&self, event: MouseUpdateEvent) {
        let /*mut*/ new_cursor = CursorKind::Normal;

        // TODO: 判断新的鼠标状态
        // TODO: 处理拖拽等事件，传递给相应窗口

        self.update_cursor(event.x, event.y, new_cursor);
    }

    /// # 函数功能
    /// 更新鼠标状态
    ///
    /// ## 参数
    /// - x: 鼠标x坐标
    /// - y: 鼠标y坐标
    /// - kind: 鼠标状态
    fn update_cursor(&self, x: i32, y: i32, kind: CursorKind) {
        // println!("[Info] Mouse_Input_Handler update cursor {:?} {:?} ", x, y);

        if kind != self.cursor_i.get() || x != self.cursor_x.get() || y != self.cursor_y.get() {
            let cursor_rect = self.cursor_rect();
            compositor().unwrap().request_redraw(cursor_rect);

            self.cursor_i.set(kind);
            self.cursor_x.set(x);
            self.cursor_y.set(y);

            let cursor_rect = self.cursor_rect();
            compositor().unwrap().request_redraw(cursor_rect);
        }
    }

    /// # 函数功能
    /// 获得鼠标位置的矩形区域
    pub fn cursor_rect(&self) -> Rect {
        let server = starry_server().unwrap();

        if let Some(image) = server.cursors.borrow().get(&self.cursor_i.get()) {
            return Rect::new(
                self.cursor_x.get(),
                self.cursor_y.get(),
                image.width(),
                image.height(),
            );
        }

        return Rect::new(self.cursor_x.get(), self.cursor_y.get(), 0, 0);
    }

    /// 更新zbuffer
    pub fn rezbuffer(&self) {
        self.zbuffer.borrow_mut().clear();

        let order = self.order.borrow_mut();
        for index in 0..order.len() {
            let id = order[index];
            let window_z = self
                .windows
                .borrow()
                .get(&index)
                .expect("窗口不存在!")
                .zorder
                .clone();
            self.zbuffer.borrow_mut().push((id, window_z, index));
        }

        self.zbuffer.borrow_mut().sort_by(|a, b| b.1.cmp(&a.1));
    }
}
