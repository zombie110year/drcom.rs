use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub(crate) account: Account,
    pub(crate) behavior: Behavior,
    pub(crate) server: Server,
    pub(crate) signal: Signal,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub(crate) struct Account {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Behavior {
    pub(crate) log_level: String,
    pub(crate) log_file: String,
    pub(crate) ror_version: bool,
    pub(crate) max_retry: i64,
}

impl Default for Behavior {
    fn default() -> Self {
        Self {
            log_level: "trace".into(),
            log_file: "./drcom.log".into(),
            ror_version: false,
            max_retry: 10,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Server {
    pub(crate) dhcp_server: String,
    pub(crate) host_ip: String,
    pub(crate) host_name: String,
    pub(crate) host_os: String,
    pub(crate) mac: u64,
    pub(crate) primary_dns: String,
    pub(crate) server: String,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            dhcp_server: "0.0.0.0".into(),
            host_ip: "".into(),
            host_name: "HOME".into(),
            host_os: "Windows".into(),
            mac: 0x123456789012,
            primary_dns: "127.0.0.1".into(),
            server: "gate.cqu.edu.cn".into(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Signal {
    pub(crate) adapter_num: u8,
    pub(crate) auth_version: [u8; 2],
    pub(crate) control_check_status: u8,
    pub(crate) ip_dog: u8,
    pub(crate) keep_alive_version: [u8; 2],
}

impl Default for Signal {
    fn default() -> Self {
        Self {
            adapter_num: 0x07,
            auth_version: [0x0a, 0x00],
            control_check_status: 0x20,
            ip_dog: 0x01, // 或者 0x07
            keep_alive_version: [0xdc, 0x02],
        }
    }
}

pub fn default_config() -> Config {
    Config::default()
}

/// read config from specified file
pub fn load_config<P: AsRef<Path>>(fp: P) -> Result<Config, std::io::Error> {
    let buf = fs::read_to_string(fp)?;
    let conf = toml::from_str(buf.as_str())?;
    return Ok(conf);
}
