use std::{
    any::Any,
    cell::{Cell, RefCell},
    sync::Arc,
};

use starry_client::base::renderer::Renderer;
use starry_server::core::{SCREEN_HEIGHT, SCREEN_WIDTH};

use crate::{
    base::{event::Event, panel::Panel, rect::Rect, vector2::Vector2},
    util::{align_rect, widget_set_panel},
};

pub mod image;
pub mod label;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PivotType {
    /// 不进行对齐 pivot_offset即为世界坐标
    None,
    /// 对齐左上角(默认对齐方式，这是由于矩形位置通过左上角顶点坐标来表示)
    TopLeft,
    /// 对齐正上方
    Top,
    /// 对齐右上角
    TopRight,
    /// 对齐正左方
    Left,
    /// 对齐中心
    Center,
    /// 对齐正右方
    Right,
    /// 对齐左下角
    BottomLeft,
    /// 对齐正下方
    Bottom,
    /// 对齐右下角
    BottomRight,
}

///  UI组件需要实现的特性
pub trait Widget: Any {
    /// 返回自身指针
    fn self_ref(&self) -> Arc<dyn Widget>;

    /// 返回Any引用
    fn as_any_ref(&self) -> &dyn Any;

    /// 返回渲染的矩形区域
    fn rect(&self) -> &Cell<Rect>;

    /// 对齐方式
    fn pivot(&self) -> &Cell<PivotType>;

    /// 基于基准点的偏移量
    fn pivot_offset(&self) -> &Cell<Vector2>;

    /// 所属面板
    fn panel(&self) -> &RefCell<Option<Arc<Panel>>>;

    /// 返回组件的名字
    fn name(&self) -> &str;

    /// 返回子组件数组
    fn children(&self) -> &RefCell<Vec<Arc<dyn Widget>>>;

    /// 父物体
    fn parent(&self) -> &RefCell<Option<Arc<dyn Widget>>>;

    /// 添加子物体
    fn add_child(&self, widget: Arc<dyn Widget>) {
        self.children().borrow_mut().push(widget.clone());

        // 赋值父物体
        (*widget.parent().borrow_mut()) = Some(self.self_ref());

        // 赋值所属的面板
        if self.panel().borrow().is_some() {
            widget_set_panel(&widget, &self.panel().borrow().clone().unwrap());
        }
    }

    /// 渲染组件
    fn draw(&self, renderer: &mut dyn Renderer, focused: bool);

    /// 更新组件状态
    fn update(&self) {}

    /// 处理输入事件
    fn handle_event(
        &self,
        _event: Event,
        _focused: bool,
        _redraw: &Cell<bool>,
        _caught: &Cell<bool>,
    ) -> bool {
        false
    }

    fn set_pivot_type(&self, pivot_type: PivotType) {
        self.set_pivot_type_base(pivot_type);
    }

    /// 修改对齐方式的统一处理 方便覆写
    fn set_pivot_type_base(&self, pivot_type: PivotType) {
        self.pivot().set(pivot_type);
        self.arrange_all();
    }

    fn set_pivot_offset(&self, pivot_offset: Vector2) {
        self.set_pivot_offset_base(pivot_offset);
    }

    /// 修改对齐偏移量的统一处理 方便覆写
    fn set_pivot_offset_base(&self, pivot_offset: Vector2) {
        self.pivot_offset().set(pivot_offset);
        self.arrange_all();
    }

    fn resize(&self, width: u32, height: u32) {
        self.resize_base(width, height);
    }

    /// 修改大小时的统一处理 方便覆写
    fn resize_base(&self, width: u32, height: u32) {
        let mut rect = self.rect().get();
        rect.width = width;
        rect.height = height;
        self.rect().set(rect);
        self.arrange_all();
    }

    /// 重新排布自身和子对象的位置
    fn arrange_all(&self) {
        self.arrange_self();

        for child in self.children().borrow_mut().iter() {
            child.arrange_all();
        }
    }

    fn arrange_self(&self) {
        self.arrange_self_base();
    }

    /// 根据参考的矩形和pivot值来调整自身位置(默认为父物体，也可以自定义为其他矩形)
    /// 统一处理 方便覆写
    fn arrange_self_base(&self) {
        let relative_rect: Rect = if self.parent().borrow().is_some() {
            // 优先以父物体作为参考
            self.parent().borrow().clone().unwrap().rect().get()
        } else if self.panel().borrow().is_some() {
            // 没有父物体 则以所属面板作为参考
            self.panel().borrow().clone().unwrap().rect()
        } else {
            // 否则以整个屏幕作为参考
            Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        };

        let target_rect = align_rect(
            self.rect().get(),
            relative_rect,
            self.pivot().get(),
            self.pivot_offset().get(),
        );

        self.rect().set(target_rect);
    }
}
