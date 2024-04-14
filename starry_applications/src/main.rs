use starry_apps::asset_manager::code::AssetManager;

fn main() {
    // set_terminal();

    // TODO
    let mut viewer = AssetManager::new();
    viewer.init();
    viewer.refresh();
    viewer.exec();
}
