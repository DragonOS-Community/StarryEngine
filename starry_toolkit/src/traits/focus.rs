use std::{cell::RefCell, sync::Arc};

use crate::widgets::Widget;

pub trait Focus {
    /// 返回当前聚焦的Widget
    fn focused_widget(&self) -> RefCell<Option<Arc<dyn Widget>>>;

    /// 聚焦于给定Widget
    fn focus(&self, widget: &Arc<dyn Widget>);

    /// 判断当前是否聚焦于给定Widget
    fn is_focused(&self, widget: &Arc<dyn Widget>) -> bool {
        if let Some(ref focused_widget) = *self.focused_widget().borrow_mut() {
            if Arc::ptr_eq(&widget, &focused_widget) {
                return true;
            }
        }

        false
    }
}
