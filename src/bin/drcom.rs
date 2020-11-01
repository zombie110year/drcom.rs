use std::env::args;
use std::path::{Path, PathBuf};
use std::time::Duration;

use drcom::prelude::{load_config, Drcom, DrcomException};
use log::{error, info, warn, LevelFilter};

fn main() {
    let conf_path = args().nth(1);
    let conf_path = if conf_path.is_none() {
        find_config().expect("找不到可用的配置文件")
    } else {
        Path::new(conf_path.unwrap().as_str()).to_owned()
    };
    let conf = load_config(&conf_path).unwrap();

    env_logger::builder()
        .filter_level(match conf.behavior.log_level.as_str() {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => LevelFilter::Info,
        })
        .init();

    info!("使用配置文件 {:?}", conf_path);
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

/// 寻找可用的配置文件，寻找顺序：
/// 1. `./drcom.toml`
/// 2. `$XDG_CONFIG_HOME/drcom/drcom.toml`
/// 3. 如果都找不到则返回 None
fn find_config() -> Option<PathBuf> {
    let choice = [
        Some(Path::new("drcom.toml").into()),
        dirs::config_dir().and_then(|d| Some(d.join("drcom/").join("drcom.toml"))),
    ];
    for p in choice.iter() {
        if let Some(p) = p {
            if p.exists() {
                return Some(p.to_owned());
            }
        }
    }

    None
}
