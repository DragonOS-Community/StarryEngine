use std::{fs::File, io::Read};

use log::debug;
use serde_derive::Deserialize;

/// TODO: 了解serde_derive::Deserialize及依赖
/// 配置信息
#[derive(Clone, Deserialize)]
pub struct Config {
    // TODO: 补充注释
    pub normal: String,
    pub bottom_left_corner: String,
    pub bottom_right_corner: String,
    pub bottom_side: String,
    pub left_side: String,
    pub right_side: String,
    pub window_max: String,
    pub window_max_unfocused: String,
    pub window_close: String,
    pub window_close_unfocused: String,
    // TODO: 实现Color反序列化
    // #[serde(default = "background_color_default")]
    // pub background_color: Color,
    // #[serde(default = "bar_color_default")]
    // pub bar_color: Color,
    // #[serde(default = "bar_highlight_color_default")]
    // pub bar_highlight_color: Color,
    // #[serde(default = "text_color_default")]
    // pub text_color: Color,
    // #[serde(default = "text_highlight_color_default")]
    // pub text_highlight_color: Color,
}

// fn background_color_default() -> Color { Color::rgb(0, 0, 0) }
// fn bar_color_default() -> Color { Color::rgba(47, 52, 63, 224) }
// fn bar_highlight_color_default() -> Color { Color::rgba(80, 86, 102, 224) }
// fn text_color_default() -> Color { Color::rgb(204, 210, 224) }
// fn text_highlight_color_default() -> Color { Color::rgb(204, 210, 224) }

impl Default for Config {
    fn default() -> Self {
        Config {
            normal: String::default(),
            bottom_left_corner: String::default(),
            bottom_right_corner: String::default(),
            bottom_side: String::default(),
            left_side: String::default(),
            right_side: String::default(),
            window_max: String::default(),
            window_max_unfocused: String::default(),
            window_close: String::default(),
            window_close_unfocused: String::default(),
            // background_color: background_color_default(),
            // bar_color: bar_color_default(),
            // bar_highlight_color: bar_highlight_color_default(),
            // text_color: text_color_default(),
            // text_highlight_color: text_highlight_color_default(),
        }
    }
}

impl Config {
    /// 通过字符串解析配置
    fn config_from_string(config: &str) -> Config {
        match toml::from_str(config) {
            Ok(config) => config,
            Err(err) => {
                println!("[Error] Config failed to parse config '{}'", err);
                Config::default()
            }
        }
    }

    /// 通过文件路径解析配置
    pub fn config_from_path(path: &str) -> Config {
        let mut string = String::new();

        match File::open(path) {
            Ok(mut file) => match file.read_to_string(&mut string) {
                Ok(_) => debug!("[Info] Reading config from path: '{}'", path),
                Err(err) => debug!("[Error] Config failed to read config '{}': {}", path, err),
            },
            Err(err) => debug!("[Error] Config failed to open config '{}': {}", path, err),
        }

        Self::config_from_string(&string)
    }
}
