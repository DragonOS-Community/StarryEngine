use std::{
    any::Any,
    cell::{Cell, RefCell},
    sync::Arc,
};

use starry_client::base::renderer::Renderer;
use starry_server::core::{SCREEN_HEIGHT, SCREEN_WIDTH};

use crate::base::{event::Event, rect::Rect, vector2::Vector2};

pub mod image;
pub mod label;

/// # 函数功能
/// 工具类 根据pivot和offset来进行矩形位置的对齐
///
/// ## 参数
/// - origin_rect: 待对齐的矩形
/// - relative_rect: 作为对齐参考的矩形
/// - pivot: 对齐方式
/// - pivot_offset: 偏移量
///
/// ## 返回值
/// 对齐后的矩形
pub fn align_rect(
    origin_rect: Rect,
    relative_rect: Rect,
    pivot: PivotType,
    pivot_offset: Vector2,
) -> Rect {
    let relative_pos = match pivot {
        PivotType::None => Vector2::new(0, 0),
        PivotType::Bottom => relative_rect.bottom_pos(),
        PivotType::BottomLeft => relative_rect.bottom_left_pos(),
        PivotType::BottomRight => relative_rect.bottom_right_pos(),
        PivotType::Center => relative_rect.center_pos(),
        PivotType::Top => relative_rect.top_pos(),
        PivotType::TopLeft => relative_rect.top_left_pos(),
        PivotType::TopRight => relative_rect.top_right_pos(),
        PivotType::Left => relative_rect.left_pos(),
        PivotType::Right => relative_rect.right_pos(),
    };

    let mut target_pos = relative_pos + pivot_offset;

    let negative_width = -(origin_rect.width as i32);
    let negative_height = -(origin_rect.height as i32);
    let offset_vec = match pivot {
        PivotType::None => Vector2::new(0, 0),
        PivotType::Bottom => Vector2::new(negative_width / 2, negative_height),
        PivotType::BottomLeft => Vector2::new(0, negative_height),
        PivotType::BottomRight => Vector2::new(negative_width, negative_height),
        PivotType::Center => Vector2::new(negative_width / 2, negative_height / 2),
        PivotType::Top => Vector2::new(negative_width / 2, 0),
        PivotType::TopLeft => Vector2::new(0, 0),
        PivotType::TopRight => Vector2::new(negative_width, 0),
        PivotType::Left => Vector2::new(0, negative_height / 2),
        PivotType::Right => Vector2::new(negative_width, negative_height / 2),
    };

    target_pos = target_pos + offset_vec;
    Rect::new(
        target_pos.x,
        target_pos.y,
        origin_rect.width,
        origin_rect.height,
    )
}

#[derive(PartialEq, Copy, Clone)]
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

    /// 返回渲染的矩形区域
    fn rect(&self) -> &Cell<Rect>;

    /// 对齐方式
    fn pivot(&self) -> &Cell<PivotType>;

    /// 基于基准点的偏移量
    fn pivot_offset(&self) -> &Cell<Vector2>;

    /// 所属面板的矩形
    fn panel_rect(&self) -> &Cell<Option<Rect>>;

    /// 返回组件的名字
    fn name(&self) -> &str;

    /// 返回子组件数组
    fn children(&self) -> &RefCell<Vec<Arc<dyn Widget>>>;

    /// 父物体
    fn parent(&self) -> &RefCell<Option<Arc<dyn Widget>>>;

    /// 添加子物体
    fn add_child(&self, widget: Arc<dyn Widget>) {
        self.children().borrow_mut().push(widget.clone());
        (*widget.parent().borrow_mut()) = Some(self.self_ref());
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
        _redraw: &mut bool,
        _caught: &mut bool,
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
        } else if self.panel_rect().get().is_some() {
            // 没有父物体 则以所属面板作为参考
            self.panel_rect().get().unwrap()
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
