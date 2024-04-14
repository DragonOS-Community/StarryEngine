extern crate starry_client;
extern crate starry_server;
extern crate starry_toolkit;
extern crate termios;

pub mod asset_manager;

use std::io;
use std::os::unix::io::AsRawFd;
use termios::{tcsetattr, Termios};

// TODO
#[allow(dead_code)]
pub fn set_terminal() -> io::Result<()> {
    let stdin = io::stdin().as_raw_fd();
    let stored_settings = Termios::from_fd(stdin)?;

    let mut new_settings = stored_settings.clone();
    new_settings.c_lflag &= !(termios::ICANON) & !(termios::ECHO);
    new_settings.c_cc[termios::VMIN] = 1;
    new_settings.c_cc[termios::VTIME] = 0;

    tcsetattr(stdin, termios::TCSANOW, &new_settings)?;

    Ok(())
}

// TODO
#[allow(dead_code)]
pub fn reset_terminal() -> io::Result<()> {
    let stdin = io::stdin().as_raw_fd();
    let stored_settings = Termios::from_fd(stdin)?;

    let mut new_settings = stored_settings.clone();
    new_settings.c_lflag &= (termios::ICANON) & (termios::ECHO);

    tcsetattr(stdin, termios::TCSANOW, &new_settings)?;

    Ok(())
}
