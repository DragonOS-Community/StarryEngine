use std::cell::Cell;

pub trait Enter {
    /// 调用键盘输入回调
    fn emit_enter(&self, char: char, redraw: &Cell<bool>);
    /// 设置回调函数
    fn set_enter_callback<T: Fn(&Self, char, &Cell<bool>) + 'static>(&self, func: T);
}
