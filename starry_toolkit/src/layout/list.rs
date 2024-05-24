use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::BTreeMap,
    sync::{Arc, Weak},
};

use starry_client::base::renderer::Renderer;

use crate::{
    base::{panel::Panel, rect::Rect, vector2::Vector2},
    traits::focus::Focus,
    widgets::{PivotType, Widget},
};

#[derive(PartialEq, Copy, Clone)]
pub enum ListArrangeType {
    /// 横向排列
    Horizontal,
    /// 纵向排列
    Vertical,
}

#[derive(PartialEq, Copy, Clone)]
pub enum ListElementPivotType {
    LeftOrTop,
    Center,
    RightOrBottom,
}

pub struct List {
    self_ref: RefCell<Weak<List>>,
    rect: Cell<Rect>,
    pivot: Cell<PivotType>,
    pivot_offset: Cell<Vector2>,
    children: RefCell<Vec<Arc<dyn Widget>>>,
    parent: RefCell<Option<Arc<dyn Widget>>>,
    panel: RefCell<Option<Arc<Panel>>>,

    space: Cell<u32>,
    current_index: Cell<usize>,
    elements: RefCell<BTreeMap<usize, Arc<dyn Widget>>>,
    focused_id: Cell<Option<usize>>,
    focused_widget: RefCell<Option<Arc<dyn Widget>>>,

    arrange_type: Cell<ListArrangeType>,
    element_pivot_type: Cell<ListElementPivotType>,
}

impl List {
    pub fn new() -> Arc<Self> {
        let list = Arc::new(List {
            self_ref: RefCell::new(Weak::default()),
            rect: Cell::new(Rect::default()),
            pivot: Cell::new(PivotType::TopLeft),
            pivot_offset: Cell::new(Vector2::new(0, 0)),
            children: RefCell::new(vec![]),
            parent: RefCell::new(None),
            panel: RefCell::new(None),
            space: Cell::new(0),
            current_index: Cell::new(0),
            elements: RefCell::new(BTreeMap::new()),
            focused_id: Cell::new(None),
            focused_widget: RefCell::new(None),
            arrange_type: Cell::new(ListArrangeType::Vertical),
            element_pivot_type: Cell::new(ListElementPivotType::Center),
        });

        (*list.self_ref.borrow_mut()) = Arc::downgrade(&list);

        return list;
    }

    pub fn set_arrange_type(&self, arrange_type: ListArrangeType) -> &Self {
        self.arrange_type.set(arrange_type);
        self
    }

    pub fn set_space(&self, space: u32) -> &Self {
        self.space.set(space);
        self
    }

    pub fn add_element<T: Widget>(&self, element: &Arc<T>) -> usize {
        self.add_child(element.self_ref());

        self.elements
            .borrow_mut()
            .insert(self.current_index.get(), element.clone());

        let res = self.current_index.get();
        self.current_index.set(res + 1);
        self.arrange_elements();
        return res;
    }

    // TODO 实现列表布局的元素排列
    pub fn arrange_elements(&self) {
        if self.elements.borrow().is_empty() {
            return;
        }

        self.arrange_self();

        // 遍历找到最大的长或宽值
        let mut max_size: u32 = 0;
        for (&_index, element) in self.elements.borrow().iter() {
            match self.arrange_type.get() {
                ListArrangeType::Horizontal => {
                    max_size = u32::max(max_size, element.rect().get().height);
                }
                ListArrangeType::Vertical => {
                    max_size = u32::max(max_size, element.rect().get().width);
                }
            }
        }

        let mut x_offset: u32 = 0;
        let mut y_offset: u32 = 0;

        for (&_index, element) in self.elements.borrow().iter() {
            let align_vector: Vector2;
            match self.arrange_type.get() {
                ListArrangeType::Horizontal => {
                    align_vector = match self.element_pivot_type.get() {
                        ListElementPivotType::LeftOrTop => {
                            Vector2::new(x_offset as i32, y_offset as i32)
                        }
                        ListElementPivotType::Center => Vector2::new(
                            x_offset as i32,
                            y_offset as i32 + (max_size - element.rect().get().height) as i32 / 2,
                        ),
                        ListElementPivotType::RightOrBottom => Vector2::new(
                            x_offset as i32,
                            y_offset as i32 + (max_size - element.rect().get().height) as i32,
                        ),
                    };
                }
                ListArrangeType::Vertical => {
                    align_vector = match self.element_pivot_type.get() {
                        ListElementPivotType::LeftOrTop => {
                            Vector2::new(x_offset as i32, y_offset as i32)
                        }
                        ListElementPivotType::Center => Vector2::new(
                            x_offset as i32 + (max_size - element.rect().get().width) as i32 / 2,
                            y_offset as i32,
                        ),
                        ListElementPivotType::RightOrBottom => Vector2::new(
                            x_offset as i32 + (max_size - element.rect().get().width) as i32,
                            y_offset as i32,
                        ),
                    }
                }
            }

            element.set_pivot_type(PivotType::TopLeft);
            element.set_pivot_offset(align_vector);
            element.arrange_all();

            match self.arrange_type.get() {
                ListArrangeType::Horizontal => {
                    x_offset += element.rect().get().width + self.space.get();
                }
                ListArrangeType::Vertical => {
                    y_offset += element.rect().get().height + self.space.get();
                }
            }
        }
    }
}

impl Widget for List {
    fn self_ref(&self) -> Arc<dyn Widget> {
        self.self_ref.borrow().upgrade().unwrap()
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn name(&self) -> &str {
        "List"
    }

    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn pivot(&self) -> &Cell<PivotType> {
        &self.pivot
    }

    fn pivot_offset(&self) -> &Cell<Vector2> {
        &self.pivot_offset
    }

    fn parent(&self) -> &RefCell<Option<Arc<dyn Widget>>> {
        &self.parent
    }

    fn children(&self) -> &RefCell<Vec<Arc<dyn Widget>>> {
        &self.children
    }

    fn panel(&self) -> &RefCell<Option<Arc<Panel>>> {
        &self.panel
    }

    fn draw(&self, renderer: &mut dyn Renderer, _focused: bool) {
        for (&_index, widget) in self.elements.borrow().iter() {
            widget.update();
            widget.draw(renderer, self.is_focused(widget));
        }
    }

    fn handle_event(
        &self,
        _event: crate::base::event::Event,
        _focused: bool,
        _redraw: &Cell<bool>,
        _caught: &Cell<bool>,
    ) -> bool {
        false
    }
}

impl Focus for List {
    fn focused_widget(&self) -> RefCell<Option<Arc<dyn Widget>>> {
        self.focused_widget.clone()
    }

    fn focus(&self, focused_widget: &Arc<dyn Widget>) {
        // 同时更新focused_id
        for (&index, widget) in self.elements.borrow().iter() {
            if Arc::ptr_eq(widget, focused_widget) {
                self.focused_id.set(Some(index));
                (*self.focused_widget.borrow_mut()) = Some(focused_widget.clone());
            }
        }
    }
}
