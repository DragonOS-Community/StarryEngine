use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};

use starry_client::{
    base::{
        color::Color,
        renderer::{RenderMode, Renderer},
    },
    window::Window,
};

use crate::{traits::focus::Focus, widgets::Widget};

use super::rect::Rect;

/// 面板渲染器
pub struct PanelRenderer<'a> {
    /// 客户端窗口
    window: &'a mut Window,
}

impl<'a> PanelRenderer<'a> {
    pub fn new(window: &'a mut Window) -> Self {
        PanelRenderer { window }
    }
}

impl<'a> Renderer for PanelRenderer<'a> {
    fn width(&self) -> u32 {
        self.window.width()
    }

    fn height(&self) -> u32 {
        self.window.height()
    }

    fn data(&self) -> &[Color] {
        self.window.data()
    }

    fn data_mut(&mut self) -> &mut [Color] {
        self.window.data_mut()
    }

    fn sync(&mut self) -> bool {
        self.window.sync()
    }

    fn mode(&self) -> &Cell<RenderMode> {
        &self.window.mode()
    }

    // TODO
    // fn char(&mut self, x: i32, y: i32, c: char, color: Color) {
    // }
}

impl<'a> Drop for PanelRenderer<'a> {
    fn drop(&mut self) {
        self.window.sync();
    }
}

/// UI面板类作为容器管理一组UI组件(UI-Widget)  
/// 拥有一个窗口对象用于渲染和事件传递
pub struct Panel {
    /// 客户端窗口对象
    window: RefCell<Window>,
    /// 子组件数组
    pub widgets: RefCell<Vec<Arc<dyn Widget>>>,
    /// 窗口是否打开
    pub running: Cell<bool>,
    /// 当前聚焦的窗口
    pub focused_widget: RefCell<Option<Arc<dyn Widget>>>,
}

impl Panel {
    pub fn new(rect: Rect, title: &str) -> Self {
        Panel::from_window(Window::new(rect.x, rect.y, rect.width, rect.height, title))
    }

    pub fn from_window(window: Window) -> Self {
        Panel {
            window: RefCell::new(window),
            widgets: RefCell::new(Vec::new()),
            running: Cell::new(true),
            focused_widget: RefCell::new(None),
        }
    }

    /// 获得客户端窗口对象
    pub fn into_window(self) -> Window {
        self.window.into_inner()
    }

    /// 返回x坐标
    pub fn x(&self) -> i32 {
        let window = self.window.borrow();
        (*window).x()
    }

    /// 返回y坐标
    pub fn y(&self) -> i32 {
        let window = self.window.borrow();
        (*window).y()
    }

    /// 返回宽度值
    pub fn width(&self) -> u32 {
        let window = self.window.borrow();
        (*window).width()
    }

    /// 返回高度值
    pub fn height(&self) -> u32 {
        let window = self.window.borrow();
        (*window).height()
    }

    /// 窗口标题
    pub fn title(&self) -> String {
        let window = self.window.borrow();
        (*window).title()
    }

    /// 改变窗口位置
    pub fn set_pos(&self, x: i32, y: i32) {
        let mut window = self.window.borrow_mut();
        (*window).set_pos(x, y);
    }

    /// 改变窗口大小
    pub fn set_size(&self, width: u32, height: u32) {
        let mut window = self.window.borrow_mut();
        (*window).set_size(width, height);
    }

    /// 改变窗口标题
    pub fn set_title(&self, title: &str) {
        let mut window = self.window.borrow_mut();
        (*window).set_title(title);
    }

    /// 关闭窗口
    pub fn close(&self) {
        self.running.set(false);
    }

    /// 添加子组件，返回子组件id
    pub fn add_child<T: Widget>(&self, widget: &Arc<T>) -> usize {
        let mut widgets = self.widgets.borrow_mut();
        let id = widgets.len();
        widgets.push(widget.clone());

        return id;
    }

    /// 渲染面板(渲染子组件数组)
    pub fn draw(&self) {
        let mut window = self.window.borrow_mut();
        let mut renderer = PanelRenderer::new(&mut window);

        for widget in self.widgets.borrow().iter() {
            self.draw_widget(&mut renderer, widget);
        }

        renderer.sync();
    }

    /// 渲染单个组件
    pub fn draw_widget(&self, renderer: &mut dyn Renderer, widget: &Arc<dyn Widget>) {
        widget.update();
        widget.draw(renderer, self.is_focused(widget));

        // 渲染子组件
        for child in widget.children().borrow().iter() {
            self.draw_widget(renderer, child);
        }
    }
}

impl Focus for Panel {
    fn focused_widget(&self) -> RefCell<Option<Arc<dyn Widget>>> {
        self.focused_widget.clone()
    }
}
