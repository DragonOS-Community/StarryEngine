use std::io;

use starry_apps::{asset_manager::code::AssetManager, set_tty};

fn main() -> io::Result<()> {
    set_tty()?;

    let mut asset_manager = AssetManager::new();
    asset_manager.init_grid();
    asset_manager.refresh_grid();
    // asset_manager.init_list();
    // asset_manager.refresh_list();
    asset_manager.exec();

    Ok(())
}
