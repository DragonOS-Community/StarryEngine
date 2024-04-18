use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    sync::{Arc, Weak},
};

use starry_client::base::renderer::Renderer;

use crate::{
    base::{event::Event, rect::Rect, vector2::Vector2},
    traits::{enter::Enter, focus::Focus},
    widgets::{PivotType, Widget},
};

/// 网格排列方式
#[derive(PartialEq, Copy, Clone)]
pub enum GridArrangeType {
    /// 优先横向排列
    Horizontal,
    /// 优先纵向排列
    Vertical,
}

pub struct Grid {
    self_ref: RefCell<Weak<Grid>>,
    pub rect: Cell<Rect>,
    pivot: Cell<PivotType>,
    pivot_offset: Cell<Vector2>,
    children: RefCell<Vec<Arc<dyn Widget>>>,
    parent: RefCell<Option<Arc<dyn Widget>>>,
    panel_rect: Cell<Option<Rect>>,
    /// x坐标间隔
    space_x: Cell<i32>,
    /// y坐标间隔
    space_y: Cell<i32>,
    /// 每行/列的最大元素数
    upper_limit: Cell<usize>,
    /// 当前行数
    pub current_row: Cell<usize>,
    /// 当前列数
    pub current_column: Cell<usize>,
    /// 当前最大行数
    pub max_row: Cell<usize>,
    /// 当前最大列数
    pub max_column: Cell<usize>,
    /// 元素字典
    pub elements: RefCell<BTreeMap<(usize, usize), Arc<dyn Widget>>>,
    /// 当前选中的元素id(行列号)
    pub focused_id: Cell<Option<(usize, usize)>>,
    /// 当前聚焦的widget
    pub focused_widget: RefCell<Option<Arc<dyn Widget>>>,
    /// 优先排列方式
    arrange_type: Cell<GridArrangeType>,
    /// 键盘输入回调
    enter_callback: RefCell<Option<Arc<dyn Fn(&Self, char, &mut bool)>>>,
}

impl Grid {
    pub fn new() -> Arc<Self> {
        let grid = Arc::new(Grid {
            self_ref: RefCell::new(Weak::default()),
            rect: Cell::new(Rect::default()),
            pivot: Cell::new(PivotType::TopLeft),
            pivot_offset: Cell::new(Vector2::new(0, 0)),
            children: RefCell::new(vec![]),
            parent: RefCell::new(None),
            panel_rect: Cell::new(None),
            space_x: Cell::new(0),
            space_y: Cell::new(0),
            upper_limit: Cell::new(0),
            current_row: Cell::new(0),
            current_column: Cell::new(0),
            max_row: Cell::new(0),
            max_column: Cell::new(0),
            elements: RefCell::new(BTreeMap::new()),
            focused_id: Cell::new(None),
            focused_widget: RefCell::new(None),
            arrange_type: Cell::new(GridArrangeType::Vertical),
            enter_callback: RefCell::new(None),
        });

        (*grid.self_ref.borrow_mut()) = Arc::downgrade(&grid);

        return grid;
    }

    /// 设置每行/列最大元素数量(取决于行/列优先排列)
    pub fn set_upper_limit(&self, columns: usize) -> &Self {
        self.upper_limit.set(columns);
        self
    }

    pub fn set_arrange_type(&self, arrange_type: GridArrangeType) -> &Self {
        self.arrange_type.set(arrange_type);
        self
    }

    pub fn add<T: Widget>(&self, element: &Arc<T>) -> (usize, usize) {
        self.find_next_slot();
        self.elements.borrow_mut().insert(
            (self.current_row.get(), self.current_column.get()),
            element.clone(),
        );
        let res = (self.current_row.get(), self.current_column.get());
        self.move_index();
        self.arrange_elements(false);
        return res;
    }

    /// 找到下一个可放置元素的位置
    fn find_next_slot(&self) {
        let elements = self.elements.borrow();
        while elements.contains_key(&(self.current_row.get(), self.current_column.get())) {
            self.move_index();
        }
    }

    fn move_index(&self) {
        match self.arrange_type.get() {
            GridArrangeType::Horizontal => {
                self.current_column.set(self.current_column.get() + 1);

                if self.current_column.get() == self.upper_limit.get() {
                    self.current_row.set(self.current_row.get() + 1);
                    self.current_column.set(0);
                }
            }
            GridArrangeType::Vertical => {
                self.current_row.set(self.current_row.get() + 1);

                if self.current_row.get() == self.upper_limit.get() {
                    self.current_column.set(self.current_column.get() + 1);
                    self.current_row.set(0);
                }
            }
        }
    }

    pub fn insert<T: Widget>(&self, column: usize, row: usize, element: &Arc<T>) {
        self.elements
            .borrow_mut()
            .insert((row, column), element.clone());

        self.arrange_elements(false);
    }

    pub fn remove(&self, column: usize, row: usize) {
        self.elements.borrow_mut().remove(&(row, column));
    }

    pub fn set_space(&self, x: i32, y: i32) -> &Self {
        self.space_x.set(x);
        self.space_y.set(y);
        self
    }

    pub fn focus_by_id(&self, (row, col): (usize, usize)) {
        if let Some(widget) = self.elements.borrow().get(&(row, col)) {
            (*self.focused_widget.borrow_mut()) = Some(widget.clone());
            self.focused_id.set(Some((row, col)));
        }
    }

    // TODO 注释补充
    pub fn arrange_elements(&self, resize_children: bool) {
        if self.elements.borrow().is_empty() {
            return;
        }

        self.arrange_self();

        let mut cols = Vec::new();
        let mut rows = Vec::new();
        for (&(row, col), entry) in self.elements.borrow().iter() {
            while col >= cols.len() {
                cols.push(Rect::default());
            }
            while row >= rows.len() {
                rows.push(Rect::default());
            }
            let rect = entry.rect().get();
            if rect.width >= cols[col].width {
                cols[col as usize].width = rect.width;
            }
            if rect.width >= rows[row].width {
                rows[row as usize].width = rect.width;
            }
            if rect.height >= cols[col].height {
                cols[col as usize].height = rect.height;
            }
            if rect.height >= rows[row].height {
                rows[row as usize].height = rect.height;
            }
        }

        self.max_row.set(rows.len());
        self.max_column.set(cols.len());

        let space_x = self.space_x.get();
        let space_y = self.space_y.get();

        let mut x = 0;
        for col in cols.iter_mut() {
            col.x = x;
            x += col.width as i32 + space_x;
        }

        let mut y = 0;
        for row in rows.iter_mut() {
            row.y = y;
            y += row.height as i32 + space_y;
        }

        let grid_width = cols.len() as i32 * (cols[0].width as i32 + space_x) - space_x;
        let grid_height = rows.len() as i32 * (rows[0].width as i32 + space_y) - space_y;
        self.resize(grid_width as u32, grid_height as u32);

        for (&(row, col), child) in self.elements.borrow().iter() {
            child.set_pivot_type(PivotType::TopLeft);
            child.set_pivot_offset(Vector2::new(cols[col].x, rows[row].y));
            if resize_children {
                child.resize(cols[col].width, rows[row].height);
            }

            child.arrange_all();
        }
    }

    pub fn clear(&self) {
        self.children.borrow_mut().clear();
        self.elements.borrow_mut().clear();
        self.current_column.set(0);
        self.current_row.set(0);
    }
}

impl Widget for Grid {
    fn self_ref(&self) -> Arc<dyn Widget> {
        self.self_ref.borrow().upgrade().unwrap()
    }

    fn name(&self) -> &str {
        "Grid"
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

    fn panel_rect(&self) -> &Cell<Option<Rect>> {
        &self.panel_rect
    }

    fn draw(&self, renderer: &mut dyn Renderer, _focused: bool) {
        for (&(_row, _col), widget) in self.elements.borrow().iter() {
            widget.update();
            widget.draw(renderer, self.is_focused(widget));
        }
    }

    fn handle_event(
        &self,
        event: Event,
        _focused: bool,
        redraw: &mut bool,
        caught: &mut bool,
    ) -> bool {
        match event {
            Event::KeyPressed { character, .. } => {
                if let Some(character) = character {
                    self.emit_enter(character, redraw);
                }

                *caught = true;
            }
            // TODO
            _ => {}
        }
        false
    }
}

impl Focus for Grid {
    fn focused_widget(&self) -> RefCell<Option<Arc<dyn Widget>>> {
        self.focused_widget.clone()
    }

    fn focus(&self, focused_widget: &Arc<dyn Widget>) {
        // 同时更新focused_id
        for ((row, col), widget) in self.elements.borrow().iter() {
            if Arc::ptr_eq(widget, focused_widget) {
                self.focused_id.set(Some((*row, *col)));
                (*self.focused_widget.borrow_mut()) = Some(focused_widget.clone());
            }
        }
    }
}

impl Enter for Grid {
    fn emit_enter(&self, char: char, redraw: &mut bool) {
        if let Some(ref enter_callback) = *self.enter_callback.borrow() {
            enter_callback(self, char, redraw);
        }
    }

    fn set_enter_callback<T: Fn(&Self, char, &mut bool) + 'static>(&self, func: T) {
        (*self.enter_callback.borrow_mut()) = Some(Arc::new(func));
    }
}
