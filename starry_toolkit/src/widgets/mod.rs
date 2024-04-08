use std::{
    any::Any,
    cell::{Cell, RefCell},
    sync::Arc,
};

use starry_client::base::renderer::Renderer;

use crate::base::{point::Point, rect::Rect};

pub mod image;
pub mod label;

/// 组件的纵向排列方式
#[derive(PartialEq, Copy, Clone)]
pub enum VerticalPlacement {
    /// 向上对齐
    Top,
    /// 居中对齐
    Center,
    /// 向下对齐
    Bottom,
    /// 绝对位置
    Absolute,
    /// 拉伸
    Stretch,
}

/// 组件的横向排列方式
#[derive(PartialEq, Copy, Clone)]
pub enum HorizontalPlacement {
    /// 靠左对齐
    Left,
    /// 居中对齐
    Center,
    /// 靠右对齐
    Right,
    /// 绝对位置
    Absolute,
    /// 拉伸
    Stretch,
}

///  UI组件需要实现的特性
pub trait Widget: Any {
    /// 返回渲染的矩形区域
    fn rect(&self) -> &Cell<Rect>;

    /// 返回组件相对于父物体的相对位置
    fn local_position(&self) -> &Cell<Point>;

    /// 返回纵向排列方式
    fn vertical_placement(&self) -> &Cell<VerticalPlacement>;

    /// 返回横向排列方式
    fn horizontal_placement(&self) -> &Cell<HorizontalPlacement>;

    /// 返回组件的名字
    fn name(&self) -> &str;

    /// 返回子组件数组
    fn children(&self) -> &RefCell<Vec<Arc<dyn Widget>>>;

    /// 添加子组件
    fn add_child(&self, widget: Arc<dyn Widget>) {
        (*self.children().borrow_mut()).push(widget);
        self.arrange();
    }

    /// 渲染组件
    fn draw(&self, renderer: &mut dyn Renderer, focused: bool);

    /// 更新组件状态
    fn update(&self) {}

    /// TODO
    fn arrange(&self) {}
}
