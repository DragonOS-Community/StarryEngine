use std::io;

use starry_apps::{asset_manager::code::AssetManager, set_tty};

fn main() -> io::Result<()> {
    set_tty()?;

    let mut viewer = AssetManager::new();
    viewer.init();
    viewer.refresh();
    viewer.exec();

    Ok(())
}
