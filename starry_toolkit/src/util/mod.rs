use std::sync::Arc;

use crate::{
    base::{panel::Panel, rect::Rect, vector2::Vector2},
    widgets::{PivotType, Widget},
};
/// # 函数功能
/// 根据pivot和offset来进行矩形位置的对齐
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

/// # 函数功能
/// 获得局部坐标系下的矩形区域
///
/// ## 参数
/// source_rect: 原来的矩形区域
/// target_rect: 作为参考的矩形
///
/// ## 返回值
/// 根据参考计算局部位置后的矩形区域
pub fn get_local_rect(source_rect: Rect, target_rect: Rect) -> Rect {
    Rect::new(
        source_rect.x - target_rect.x,
        source_rect.y - target_rect.y,
        source_rect.width,
        source_rect.height,
    )
}

// TODO 注释补充
pub fn widget_set_panel(widget: &Arc<dyn Widget>, panel: &Arc<Panel>) {
    (*widget.panel().borrow_mut()) = Some(panel.clone());

    for child in widget.children().borrow().iter() {
        widget_set_panel(child, panel);
    }
}
