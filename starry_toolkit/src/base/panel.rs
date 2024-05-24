use starry_client::{
    base::{
        color::Color,
        renderer::{RenderMode, Renderer},
    },
    window::Window,
};
use std::{
    cell::{Cell, RefCell},
    fs::File,
    io::Read,
    sync::{Arc, Weak},
    thread,
    time::Duration,
};

use crate::{traits::focus::Focus, util::widget_set_panel, widgets::Widget};

use super::{event::Event, rect::Rect};

const TTY_DEVICE_PATH: &str = "/dev/char/tty0";

const DURATION_TIME: Duration = Duration::from_millis(25);

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PanelRendererMode {
    /// 标准模式
    Normal,
    /// 绘制线框
    WithWireframe,
    /// 仅线框
    OnlyWireframe,
}

/// 面板渲染器
pub struct PanelRenderer<'a> {
    /// 客户端窗口
    window: &'a mut Window,
}

impl<'a> PanelRenderer<'a> {
    pub fn new(window: &'a mut Window) -> Self {
        PanelRenderer { window: window }
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

/// UI面板类作为容器管理一组UI组件(UI-Widget)   
/// 拥有一个窗口对象用于渲染和事件传递
pub struct Panel {
    /// 指向自身的弱引用
    self_ref: RefCell<Weak<Panel>>,
    /// 客户端窗口对象
    window: RefCell<Window>,
    /// 面板矩形
    rect: Cell<Rect>,
    /// 管理的控件对象数组
    widgets: RefCell<Vec<Arc<dyn Widget>>>,
    /// 窗口是否打开
    running: Cell<bool>,
    /// 当前聚焦的窗口
    focused_widget: RefCell<Option<Arc<dyn Widget>>>,
    /// 事件数组
    events: RefCell<Vec<Event>>,
    /// 需要重绘画面
    redraw: Cell<bool>,
    /// tty文件
    tty_file: RefCell<File>,
    /// 渲染模式
    renderer_mode: Cell<PanelRendererMode>,
}

impl Panel {
    pub fn new(rect: Rect, title: &str, color: Color) -> Arc<Panel> {
        Panel::from_window(
            Window::new(rect.x, rect.y, rect.width, rect.height, title, color),
            rect,
        )
    }

    pub fn from_window(window: Window, rect: Rect) -> Arc<Panel> {
        let panel = Arc::new(Panel {
            self_ref: RefCell::new(Weak::default()),
            window: RefCell::new(window),
            rect: Cell::new(rect),
            widgets: RefCell::new(Vec::new()),
            running: Cell::new(true),
            focused_widget: RefCell::new(None),
            events: RefCell::new(Vec::new()),
            redraw: Cell::new(false),
            tty_file: RefCell::new(
                File::open(TTY_DEVICE_PATH).expect("[Error] Panel failed to open tty file"),
            ),
            renderer_mode: Cell::new(PanelRendererMode::Normal),
        });

        (*panel.self_ref.borrow_mut()) = Arc::downgrade(&panel);

        return panel;
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

    /// 返回面板矩形
    pub fn rect(&self) -> Rect {
        self.rect.get()
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

    /// 设置是否绘制线框
    pub fn set_renderer_mode(&self, renderer_mode: PanelRendererMode) {
        self.renderer_mode.set(renderer_mode);
    }

    /// 关闭窗口
    pub fn close(&self) {
        self.running.set(false);
    }

    /// 添加子组件，返回子组件id
    pub fn add_child<T: Widget>(&self, widget: &Arc<T>) -> usize {
        widget_set_panel(
            &widget.self_ref(),
            &self.self_ref.borrow().upgrade().unwrap(),
        );

        let mut widgets = self.widgets.borrow_mut();
        let id = widgets.len();
        widgets.push(widget.clone());
        widget.arrange_all();
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

        if self.renderer_mode.get() == PanelRendererMode::Normal
            || self.renderer_mode.get() == PanelRendererMode::WithWireframe
        {
            widget.draw(renderer, self.is_focused(widget));
        }

        if self.renderer_mode.get() == PanelRendererMode::WithWireframe
            || self.renderer_mode.get() == PanelRendererMode::OnlyWireframe
        {
            Self::draw_rect_wireframe(renderer, widget.rect().get(), Color::rgb(0, 0, 0));
        }

        // 渲染子组件
        for child in widget.children().borrow().iter() {
            if self.renderer_mode.get() == PanelRendererMode::Normal
                || self.renderer_mode.get() == PanelRendererMode::WithWireframe
            {
                self.draw_widget(renderer, child);
            }

            if self.renderer_mode.get() == PanelRendererMode::WithWireframe
                || self.renderer_mode.get() == PanelRendererMode::OnlyWireframe
            {
                Self::draw_rect_wireframe(renderer, child.rect().get(), Color::rgb(0, 0, 0));
            }
        }
    }

    /// 绘制矩形线框
    fn draw_rect_wireframe(renderer: &mut dyn Renderer, rect: Rect, color: Color) {
        renderer.lines(
            &[
                [rect.top_left_pos().x, rect.top_left_pos().y],
                [rect.top_right_pos().x, rect.top_right_pos().y],
                [rect.bottom_right_pos().x, rect.bottom_right_pos().y],
                [rect.bottom_left_pos().x, rect.bottom_left_pos().y],
                [rect.top_left_pos().x, rect.top_left_pos().y],
            ],
            color,
        );
    }

    pub fn tick(&self) {
        // TODO 通过服务器，先从Window对象接收事件，再进行处理
        self.handle_events();
    }

    /// 将事件传递给Widget对象
    fn handle_events(&self) {
        while let Some(event) = self.events.borrow_mut().pop() {
            // 事件是否已被处理
            let caught = Cell::new(false);

            for widget in self.widgets.borrow().iter().rev() {
                // TODO 处理返回值
                widget.handle_event(event, self.is_focused(widget), &self.redraw, &caught);

                if caught.get() {
                    break;
                }
            }
        }
    }

    // TODO 临时函数 用于客户端直接处理用户输入
    pub fn push_event(&self, event: Event) {
        self.events.borrow_mut().push(event);
    }

    pub fn exec(&self) {
        while self.running.get() {
            self.polling_tty();
            self.tick();
            self.draw_if_needed();

            thread::sleep(DURATION_TIME);
        }
    }

    /// 必要时重绘
    fn draw_if_needed(&self) {
        if self.redraw.get() {
            self.draw();
            self.redraw.set(false);
        }
    }

    // TODO 临时在客户端做输入读取  后续改为由服务器实现
    fn polling_tty(&self) {
        let mut bufffer: [u8; 128] = [0; 128];
        let count = self
            .tty_file
            .borrow_mut()
            .read(&mut bufffer)
            .expect("[Error] Panel failed to read tty file");
        for i in 0..count {
            self.push_event(Event::KeyPressed {
                character: Some(bufffer[i] as char),
            });
        }
    }
}

impl Focus for Panel {
    fn focused_widget(&self) -> RefCell<Option<Arc<dyn Widget>>> {
        self.focused_widget.clone()
    }

    fn focus(&self, widget: &Arc<dyn Widget>) {
        (*self.focused_widget.borrow_mut()) = Some(widget.clone());
    }
}
