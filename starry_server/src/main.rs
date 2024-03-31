use std::rc::Rc;


use starry_server::{base::display::Display, config::Config, core::{starry_server, StarryServer, SCREEN_HEIGHT, SCREEN_WIDTH}};


fn main() {
    // TODO 考虑多个显示器
    let mut displays : Vec<Display> = Vec::new();
    displays.push(Display::new(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32));

    // TODO 暂时不考虑配置文件
    let config: Rc<Config> = Rc::new(Config::default());

    //开启Starry Server
    StarryServer::new(config, displays);
    let server = starry_server().unwrap();
    server.run();
}
