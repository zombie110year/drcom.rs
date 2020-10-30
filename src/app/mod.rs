mod exception;
mod login;

use exception::*;

use std::net::UdpSocket;

use rand::Rng;

use crate::config::Config;
use std::thread;
use std::time::Duration;

pub use login::make_login_ticket;

const DELAY: u64 = 5;

fn find_available_udp() -> UdpSocket {
    for port in 3000..0xffff {
        if let Ok(us) = UdpSocket::bind(std::net::SocketAddr::from(([0, 0, 0, 0], port))) {
            return us;
        }
    }
    panic!("there were no port available: 3000-65535");
}

pub struct Drcom {
    conf: Config,
    pipe: UdpSocket,
    salt: [u8; 4],
    // 登录凭据
    token: [u8; 16],
}

impl Drcom {
    pub fn new(conf: Config) -> Result<Self, DrcomException> {
        let pipe = find_available_udp();
        pipe.connect(&conf.server.server)?;
        Ok(Self {
            conf,
            pipe,
            salt: [0; 4],
            token: [0; 16],
        })
    }
}

impl Drcom {
    pub fn login(&mut self) {
        let mut counter = 0;
        let max_retry = self.conf.behavior.max_retry;
        let delay_base: u64 = 2;
        loop {
            if counter == max_retry {
                error!("达到最大重试次数 {}，终止程序", counter);
                std::process::exit(-1);
            }
            if let Err(e) = self.chanllenge().and_then(|_| self.send_login()) {
                counter += 1;
                let wait = DELAY * delay_base.pow(counter as u32);
                match e {
                    DrcomException::AccountError => {
                        error!("帐号密码错误，请重新设置账户");
                        std::process::exit(-1);
                    }
                    DrcomException::AccountStopped => {
                        error!("帐号处于停机状态，请至 user.cqu.edu.cn 启用");
                        warn!("{} 秒后重试", wait);
                    }
                    DrcomException::AccountOutOfCost => {
                        error!("帐号欠费，请缴费后重新连接");
                        warn!("{} 秒后重试", wait);
                    }
                    DrcomException::StdIOError(ioe) => {
                        error!("std::io::Error {:?}", ioe);
                        warn!("{} 秒后重试", wait);
                    }
                    DrcomException::ChallengeRemoteDenied => {
                        error!("challenge 失败");
                        warn!("{} 秒后重试", wait);
                    }
                    DrcomException::LoginError => {
                        error!("未知的登录错误");
                        std::process::exit(-1);
                    }
                }
                thread::sleep(Duration::from_secs(wait));
            } else {
                break;
            }
        }
    }

    fn chanllenge(&mut self) -> Result<(), DrcomException> {
        let now: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("无法读取系统计时器")
            .as_secs();
        let rd: u64 = now + rand::thread_rng().gen_range(0xf, 0xff);
        let mut knock: [u8; 20] = [
            0x01, 0x02, 0, 0, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        &knock[2..=3].copy_from_slice(&(rd as u16).to_le_bytes()[..]);
        let send_size = self.pipe.send(&knock)?;
        debug!("challenge sent({}) {:?}", send_size, &knock);

        let mut response: Vec<u8> = Vec::with_capacity(1024);
        let recv_size = self.pipe.recv(&mut response)?;
        debug!("challenge recv({}) {:?}", recv_size, &response);

        if !response.starts_with(b"\x02") {
            warn!("challenge err({}) {:?}", recv_size, &response);
            return Err(DrcomException::ChallengeRemoteDenied);
        } else {
            let salt = &response[4..8];
            debug!("salt set(4) {:?}", &salt);
            self.salt.copy_from_slice(salt);
            return Ok(());
        }
    }

    fn send_login(&mut self) -> Result<(), DrcomException> {
        // todo build login packet
        let ticket = make_login_ticket(
            &self.conf.account.username,
            &self.conf.account.password,
            &self.salt,
            self.conf.signal.control_check_status,
            self.conf.signal.adapter_num,
            self.conf.server.mac,
            &self.conf.server.host_ip,
            self.conf.signal.ip_dog,
            &self.conf.server.host_name,
            &self.conf.server.primary_dns,
            &self.conf.server.dhcp_server,
            &self.conf.server.host_os,
            &self.conf.signal.auth_version,
        );
        let send_size = self.pipe.send(&ticket)?;
        debug!("login sent({}) {:?}", send_size, &ticket);

        let mut response: Vec<u8> = Vec::new();
        let recv_size = self.pipe.recv(&mut response)?;
        debug!("login recv({}) {:?}", recv_size, &response);

        if response.starts_with(b"\x04") {
            let token = &response[23..39];
            self.token.copy_from_slice(&token);
            debug!("token set(16) {:?}", &token);
            return Ok(());
        } else {
            let header = &response[..5];
            return match header {
                b"\x05\x00\x00\x05\x03" => Err(DrcomException::AccountError),
                b"\x05\x00\x00\x05\x04" => Err(DrcomException::AccountOutOfCost),
                b"\x05\x00\x00\x05\x05" => Err(DrcomException::AccountStopped),
                _ => Err(DrcomException::LoginError),
            };
        }
    }
}
