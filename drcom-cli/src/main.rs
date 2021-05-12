use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use drcom::prelude::{load_config, Drcom, DrcomException};
use log::{debug, error, info, warn};

use clap::{crate_authors, crate_description, crate_name, crate_version};
use clap::{App, Arg, SubCommand};

fn main() {
    let argparser = App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommands(vec![
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
        drcom_run(m)
    } else if let Some(_) = args.subcommand_matches("default-toml") {
        gen_default_config()
    } else {
        // 没有输入子命令时认为 run
        drcom_run(&args)
    }
}

fn drcom_run(m: &clap::ArgMatches<'static>) {
    let conf_path = m
        .value_of("config")
        .and_then(|fp| Some(PathBuf::from(fp)))
        .or_else(find_config)
        .expect("找不到可用的配置文件");
    let conf = {
        let buf = std::fs::read(&conf_path).unwrap();
        let conf_text = String::from_utf8_lossy(&buf).to_string();
        load_config(&conf_text).expect(format!("配置文件格式错误: {:?}", conf_text).as_str())
    };
    env_logger::init();

    info!("使用配置文件 {:?}", conf_path);
    loop {
        let conf = conf.clone();
        debug!("配置文件 {:?}", &conf);
        let worker = thread::spawn(move || {
            // 每次重启时需要重新建立连接，而非仅重新登录
            let mut app = Drcom::new(conf).unwrap();
            app.login();
            app.empty_socket_buffer();
            let exit_code = match app.keep_alive() {
                Ok(_) => 0,
                Err(DrcomException::KeepAlive1)
                | Err(DrcomException::KeepAlive2)
                | Err(DrcomException::KeepAlive4)
                | Err(DrcomException::StdIOError(_)) => {
                    error!("KeepAlive Stable Error");
                    warn!("20 秒后重启");
                    std::thread::sleep(Duration::new(20, 0));
                    1
                }
                // KeepAlive3 常因检测到多设备而中断，见 DEVLOG.md:#2021-05-09
                Err(DrcomException::KeepAlive3) => {
                    error!("被检测到多设备，立刻重启！");
                    1
                }
                Err(e) => {
                    error!("其他错误 {:?}", e);
                    2
                }
            };
            match app.logout() {
                Ok(_) => (),
                Err(DrcomException::LogoutError) => error!("未知的登出错误"),
                Err(e) => error!("{:?}", e),
            }
            return exit_code;
        });
        let exit_code = worker.join().unwrap();
        match exit_code {
            0 => break,
            1 => {
                thread::sleep(Duration::new(20, 0));
                continue;
            }
            2 => break,
            _ => panic!(),
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

/// 在当前目录下生成默认的配置文件
fn gen_default_config() {
    use drcom::prelude::default_config;
    use std::fs::write;

    let x = default_config();
    let toml: String = toml::to_string(&x).expect("将默认配置转换为 TOML 文本时出错");
    write("drcom.default.toml", &toml).unwrap();
    println!("默认配置保存为 './drcom.default.toml'");
}
