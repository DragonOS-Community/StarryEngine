use std::io;

use starry_apps::{asset_manager::code::AssetManager, set_tty};

fn main() -> io::Result<()> {
    set_tty()?;

    let mut asset_manager = AssetManager::new();
    asset_manager.init();
    asset_manager.refresh();
    asset_manager.exec();

    Ok(())
}
