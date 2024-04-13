use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    sync::Arc,
};

use starry_client::base::renderer::Renderer;

use crate::{
    base::{rect::Rect, vector2::Vector2},
    traits::focus::Focus,
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
    pub rect: Cell<Rect>,
    pivot: Cell<PivotType>,
    pivot_offset: Cell<Vector2>,
    children: RefCell<Vec<Arc<dyn Widget>>>,
    parent: RefCell<Option<Arc<dyn Widget>>>,
    /// x坐标间隔
    space_x: Cell<i32>,
    /// y坐标间隔
    space_y: Cell<i32>,
    /// 每行/列的最大元素数
    upper_limit: Cell<usize>,
    /// 当前行数
    current_row: Cell<usize>,
    /// 当前列数
    current_column: Cell<usize>,
    /// 元素字典
    pub elements: RefCell<BTreeMap<(usize, usize), Arc<dyn Widget>>>,
    /// 当前选中的元素id(行列号)
    pub focused_id: Cell<Option<(usize, usize)>>,
    /// 当前聚焦的widget
    pub focused_widget: RefCell<Option<Arc<dyn Widget>>>,
    /// 优先排列方式
    arrange_type: Cell<GridArrangeType>,
}

impl Grid {
    pub fn new() -> Arc<Self> {
        Arc::new(Grid {
            rect: Cell::new(Rect::default()),
            pivot: Cell::new(PivotType::TopLeft),
            pivot_offset: Cell::new(Vector2::new(0, 0)),
            children: RefCell::new(vec![]),
            parent: RefCell::new(None),
            space_x: Cell::new(0),
            space_y: Cell::new(0),
            upper_limit: Cell::new(0),
            current_row: Cell::new(0),
            current_column: Cell::new(0),
            elements: RefCell::new(BTreeMap::new()),
            focused_id: Cell::new(None),
            focused_widget: RefCell::new(None),
            arrange_type: Cell::new(GridArrangeType::Vertical),
        })
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

    pub fn add<T: Widget>(&self, element: &Arc<T>) {
        self.find_next_slot();
        self.elements.borrow_mut().insert(
            (self.current_row.get(), self.current_column.get()),
            element.clone(),
        );
        self.move_index();
        self.arrange_elements(false);
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

    pub fn clear(&self) {
        self.elements.borrow_mut().clear();
    }

    pub fn remove(&self, column: usize, row: usize) {
        self.elements.borrow_mut().remove(&(row, column));
    }

    pub fn set_space(&self, x: i32, y: i32) -> &Self {
        self.space_x.set(x);
        self.space_y.set(y);
        self
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
}

impl Widget for Grid {
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

    fn draw(&self, renderer: &mut dyn Renderer, _focused: bool) {
        fn draw_widget(widget: &Arc<dyn Widget>, renderer: &mut dyn Renderer, focused: bool) {
            widget.update();
            widget.draw(renderer, focused);

            for child in widget.children().borrow().iter() {
                draw_widget(child, renderer, focused);
            }
        }

        for (&(_col, _row), widget) in self.elements.borrow().iter() {
            draw_widget(widget, renderer, self.is_focused(widget));
        }
    }
}

impl Focus for Grid {
    fn focused_widget(&self) -> RefCell<Option<Arc<dyn Widget>>> {
        self.focused_widget.clone()
    }

    fn focus(&self, widget: &Arc<dyn Widget>) {
        (*self.focused_widget.borrow_mut()) = Some(widget.clone());
    }
}
