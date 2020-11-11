use std::path::{Path, PathBuf};
use std::time::Duration;

use drcom::prelude::{load_config, Drcom, DrcomException};
use log::{error, info, warn, LevelFilter};

use clap::{App, Arg, SubCommand};

fn main() {
    let argparser = App::new("drcom-cli").subcommands(vec![
        SubCommand::with_name("run")
            .about("启动 Drcom 连接程序")
            .arg(
                Arg::with_name("config")
                    .long("config")
                    .short("c")
                    .takes_value(true)
                    .help("指定要运行的配置文件"),
            ),
        SubCommand::with_name("default-toml").about("在当前目录下生成默认的 TOML 配置文件"),
    ]);
    let args = argparser.get_matches();
    if let Some(m) = args.subcommand_matches("run") {
        let conf_path = m
            .value_of("config")
            .and_then(|fp| Some(PathBuf::from(fp)))
            .or_else(find_config)
            .expect("找不到可用的配置文件");
        let conf =
            load_config(&conf_path).expect(format!("配置文件格式错误: {:?}", conf_path).as_str());
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
    } else if let Some(_) = args.subcommand_matches("default-toml") {
        gen_default_config()
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

/// 在当前目录下生成默认的配置文件
fn gen_default_config() {
    use drcom::prelude::default_config;
    use std::fs::write;

    let x = default_config();
    let toml: String = toml::to_string(&x).expect("将默认配置转换为 TOML 文本时出错");
    write("drcom.default.toml", &toml).unwrap();
    println!("默认配置保存为 './drcom.default.toml'");
}
