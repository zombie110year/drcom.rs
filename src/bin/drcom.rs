use std::time::Duration;

use drcom::prelude::{load_config, Drcom, DrcomException};
use log::{error, warn};

fn main() {
    env_logger::init();

    let conf = load_config("drcom.dev.toml").unwrap();
    let mut app = Drcom::new(conf).unwrap();
    loop {
        app.login();
        app.empty_socket_buffer();
        match app.keep_alive() {
            Ok(_) => break,
            Err(DrcomException::KeepAlive1)
            | Err(DrcomException::KeepAlive2)
            | Err(DrcomException::KeepAlive3)
            | Err(DrcomException::KeepAlive4)
            | Err(DrcomException::StdIOError(_)) => {
                error!("KeepAlive Stable Error");
                warn!("20 秒后重启");
                std::thread::sleep(Duration::new(20, 0));
                continue;
            }
            Err(e) => {
                error!("其他错误 {:?}", e);
                panic!("{:?}", e);
            }
        }
    }
}
