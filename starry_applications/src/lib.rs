extern crate starry_client;
extern crate starry_server;
extern crate starry_toolkit;
extern crate termios;

pub mod asset_manager;

use std::io;
use std::os::unix::io::AsRawFd;
use termios::{tcsetattr, Termios};

static mut STORED_SETTINGS: Option<Termios> = None;

/// 禁止tty回显 并设置为非阻塞模式
pub fn set_tty() -> io::Result<()> {
    let stdin = io::stdin().as_raw_fd();
    let cur_settings = Termios::from_fd(stdin)?;

    let mut new_settings = cur_settings.clone();
    new_settings.c_lflag &= !(termios::ICANON) & !(termios::ECHO);
    new_settings.c_cc[termios::VMIN] = 1;
    new_settings.c_cc[termios::VTIME] = 0;

    tcsetattr(stdin, termios::TCSANOW, &new_settings)?;

    unsafe {
        STORED_SETTINGS = Some(cur_settings);
    }

    Ok(())
}

/// 回退tty设置
pub fn reset_tty() -> io::Result<()> {
    let stdin = io::stdin().as_raw_fd();

    unsafe {
        tcsetattr(stdin, termios::TCSANOW, &STORED_SETTINGS.unwrap())?;
    }

    Ok(())
}
