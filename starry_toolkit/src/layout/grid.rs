use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    sync::Arc,
};

use starry_client::base::renderer::Renderer;

use crate::{
    base::{point::Point, rect::Rect},
    traits::{focus::Focus, transform::Transform},
    widgets::{HorizontalPlacement, VerticalPlacement, Widget},
};

pub struct Grid {
    pub rect: Cell<Rect>,
    local_position: Cell<Point>,
    vertical_placement: Cell<VerticalPlacement>,
    horizontal_placement: Cell<HorizontalPlacement>,
    children: RefCell<Vec<Arc<dyn Widget>>>,
    /// x坐标间隔
    space_x: Cell<i32>,
    /// y坐标间隔
    space_y: Cell<i32>,
    /// 每行的最大列数
    max_columns: Cell<usize>,
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
}

impl Grid {
    pub fn new() -> Arc<Self> {
        Arc::new(Grid {
            rect: Cell::new(Rect::default()),
            local_position: Cell::new(Point::new(0, 0)),
            vertical_placement: Cell::new(VerticalPlacement::Absolute),
            horizontal_placement: Cell::new(HorizontalPlacement::Absolute),
            children: RefCell::new(vec![]),
            space_x: Cell::new(0),
            space_y: Cell::new(0),
            max_columns: Cell::new(0),
            current_row: Cell::new(0),
            current_column: Cell::new(0),
            elements: RefCell::new(BTreeMap::new()),
            focused_id: Cell::new(None),
            focused_widget: RefCell::new(None),
        })
    }

    /// 设置最大列数
    pub fn set_max_columns(&self, columns: usize) -> &Self {
        self.max_columns.set(columns);
        self
    }

    pub fn add<T: Widget>(&self, element: &Arc<T>) {
        if self.current_column.get() == self.max_columns.get() {
            self.current_row.set(self.current_row.get() + 1);
            self.current_column.set(0);
        }

        self.elements.borrow_mut().insert(
            (self.current_row.get(), self.current_column.get()),
            element.clone(),
        );
        self.current_column.set(self.current_column.get() + 1);
        self.arrange(false);
    }

    pub fn insert<T: Widget>(&self, column: usize, row: usize, element: &Arc<T>) {
        self.elements
            .borrow_mut()
            .insert((row, column), element.clone());

        self.arrange(false);
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

    pub fn arrange(&self, resize_children: bool) {
        let mut cols = Vec::new();
        let mut rows = Vec::new();
        for (&(col, row), entry) in self.elements.borrow().iter() {
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

        let rect = self.rect.get();
        let space_x = self.space_x.get();
        let space_y = self.space_y.get();

        let mut x = rect.x;
        for col in cols.iter_mut() {
            col.x = x;
            x += col.width as i32 + space_x;
        }

        let mut y = rect.y;
        for row in rows.iter_mut() {
            row.y = y;
            y += row.height as i32 + space_y;
        }

        for (&(col, row), child) in self.elements.borrow().iter() {
            let mut rect = child.rect().get();
            rect.x = cols[col].x;
            rect.y = rows[row].y;
            if resize_children {
                rect.width = cols[col].width;
                rect.height = rows[row].height;
            }
            child.rect().set(rect);

            child.arrange();
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

    fn local_position(&self) -> &Cell<Point> {
        &self.local_position
    }

    fn vertical_placement(&self) -> &Cell<VerticalPlacement> {
        &self.vertical_placement
    }

    fn horizontal_placement(&self) -> &Cell<HorizontalPlacement> {
        &self.horizontal_placement
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

impl Transform for Grid {
    fn reposition(&self, x: i32, y: i32) -> &Self {
        let mut rect = self.rect().get();
        rect.x = x;
        rect.y = y;
        self.rect.set(rect);

        self.arrange(false);

        self
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
