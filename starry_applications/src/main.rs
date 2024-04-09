use starry_apps::asset_manager::code::AssetViewer;

fn main() {
    let viewer = AssetViewer::new();
    viewer.init();
    viewer.refresh();
    viewer.draw();

    loop {}
}
