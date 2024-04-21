use self::asset_item::AssetItem;
use crate::starry_toolkit::traits::focus::Focus;
use starry_client::base::color::Color;
use starry_server::base::image::Image as ImageResource;
use starry_server::core::{SCREEN_HEIGHT, SCREEN_WIDTH};
use starry_toolkit::{
    base::{panel::Panel, rect::Rect},
    layout::grid::{Grid, GridArrangeType},
    traits::enter::Enter,
    widgets::image::Image,
};
use std::{collections::BTreeMap, fs, sync::Arc};
pub mod asset_item;

const DESKTOP_BG_PATH: &[u8] = include_bytes!("../resource/desktop_bg.png");
const LOADING_IMG_PATH: &[u8] = include_bytes!("../resource/loading.png");

pub struct AssetManager {
    cur_path: String,
    asset_grid: Arc<Grid>,
    items: BTreeMap<(usize, usize), Arc<AssetItem>>,
    panel: Arc<Panel>,
    // 原则上一个应用程序对应一个Panel和Window
    // 这里额外创建一个Panel用于Loading图标优先显示
    // 后续通过Server来显示Loading或不显示Loading
    loading_panel: Arc<Panel>,
    init_show: bool,
}

impl AssetManager {
    pub fn new() -> Self {
        AssetManager {
            cur_path: String::from("/"),
            asset_grid: Grid::new(),
            items: BTreeMap::new(),
            panel: Panel::new(
                Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
                "Title",
                Color::rgb(0, 0, 0),
            ),
            loading_panel: Panel::new(
                Rect::new(SCREEN_WIDTH as i32 - 64, SCREEN_HEIGHT as i32 - 64, 64, 64),
                "Loading",
                Color::rgb(255, 255, 255),
            ),
            init_show: true,
        }
    }

    pub fn init(&mut self) {
        self.init_loading_panel();

        let grid = self.asset_grid.clone();
        grid.set_upper_limit(8);
        grid.set_space(20, 20);
        grid.set_arrange_type(GridArrangeType::Horizontal);

        // 处理输入回调
        let self_ptr = self as *mut AssetManager;
        grid.set_enter_callback(move |grid, char, redraw| {
            let asset_manager: &mut AssetManager = unsafe { &mut *self_ptr };

            if char == '\n' {
                if let Some(item) = asset_manager.items.get(&grid.focused_id.get().unwrap()) {
                    // 判断是否是文件夹
                    if item.is_dir.get() == false {
                        return;
                    }

                    // 返回上级目录
                    if item.file_path.borrow().eq(&"..".to_string()) {
                        if asset_manager.cur_path.len() == 1 {
                            return;
                        } else {
                            let split_path =
                                &asset_manager.cur_path[..asset_manager.cur_path.len() - 1];
                            let slash_pos = split_path.rfind('/').unwrap();
                            let _ = asset_manager.cur_path.split_off(slash_pos + 1);
                        }
                    } else {
                        // 打开文件夹
                        asset_manager.cur_path.push_str(&item.file_path.borrow());
                        asset_manager.cur_path.push_str(&"/");
                    }
                    asset_manager.refresh();
                }

                return;
            }

            let row_offset: i32 = match char {
                'a' => 0,
                'A' => 0,
                'd' => 0,
                'D' => 0,
                'w' => -1,
                'W' => -1,
                's' => 1,
                'S' => 1,
                _ => 0,
            };

            let col_offset: i32 = match char {
                'a' => -1,
                'A' => -1,
                'd' => 1,
                'D' => 1,
                'w' => 0,
                'W' => 0,
                's' => 0,
                'S' => 0,
                _ => 0,
            };

            if row_offset == 0 && col_offset == 0 {
                return;
            }
            let mut nxt_row = grid.focused_id.get().unwrap().0 as i32 + row_offset;
            let mut nxt_col = grid.focused_id.get().unwrap().1 as i32 + col_offset;
            loop {
                if nxt_row < 0
                    || nxt_row >= grid.max_row.get() as i32
                    || nxt_col < 0
                    || nxt_col >= grid.max_column.get() as i32
                {
                    return;
                }

                if grid
                    .elements
                    .borrow()
                    .contains_key(&(nxt_row as usize, nxt_col as usize))
                {
                    break;
                }

                nxt_row += row_offset;
                nxt_col += col_offset;
            }

            grid.focus(
                grid.elements
                    .borrow()
                    .get(&(nxt_row as usize, nxt_col as usize))
                    .unwrap(),
            );

            asset_manager.loading_panel.draw();
            redraw.set(true);
        });

        self.panel.add_child(&Image::new_from_image(
            ImageResource::from_path(DESKTOP_BG_PATH).unwrap(),
        ));

        self.panel.add_child(&(self.asset_grid));
    }

    fn init_loading_panel(&mut self) {
        self.loading_panel.add_child(&Image::new_from_image(
            ImageResource::from_path(LOADING_IMG_PATH).unwrap(),
        ));
    }

    pub fn refresh(&mut self) {
        self.items.clear();
        self.asset_grid.clear();

        // 父目录
        let parent_asset_item = AssetItem::new("..", true);
        let (row, col) = self.asset_grid.add_element(&parent_asset_item);
        self.items.insert((row, col), parent_asset_item.clone());

        // 读取目录中的文件列表
        if let Ok(entries) = fs::read_dir(&self.cur_path) {
            for entry in entries {
                if let Ok(item) = entry {
                    let is_dir = if let Ok(metadata) = item.metadata() {
                        metadata.is_dir()
                    } else {
                        false
                    };

                    let asset_item = AssetItem::new(item.file_name().to_str().unwrap(), is_dir);
                    let (row, col) = self.asset_grid.add_element(&asset_item);
                    self.items.insert((row, col), asset_item.clone());
                }
            }
        } else {
            println!(
                "[Error] AssetManager failed to read dir {:?}",
                self.cur_path
            );
        }

        let grid = self.asset_grid.clone();
        if let Some(widget) = grid.elements.borrow().get(&(0, 0)) {
            grid.focused_id.set(Some((0, 0)));
            grid.focus(widget);
        }

        if self.init_show == true {
            self.init_show = false
        } else {
            self.loading_panel.draw();
        }
        self.panel.draw();
    }

    pub fn exec(&mut self) {
        self.panel.exec();
    }
}
