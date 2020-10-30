mod exception;
mod login;

use exception::*;

use std::net::UdpSocket;

use rand::Rng;

use crate::config::Config;
use std::thread;
use std::time::Duration;

pub use login::LoginTicket;

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
    fn login(&mut self) {
        let mut counter = 0;
        let max_retry = self.conf.behavior.max_retry;
        let delay_base: u64 = 2;
        while counter != max_retry {
            // todo chanllenge
            // todo sendlogin

            counter += 1;
            thread::sleep(Duration::from_secs(DELAY * delay_base.pow(counter as u32)));
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
        let ticket = [0_u8; 10];
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
                _ => Err(DrcomException::LoginError(header.to_vec())),
            };
        }
    }
}
