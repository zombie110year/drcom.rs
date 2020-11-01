pub(crate) mod exception;
pub(crate) mod login;

use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

use md5::{Digest, Md5};
use rand::Rng;

use self::exception::*;
use self::login::make_login_ticket;
use crate::config::Config;

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
        pipe.set_read_timeout(Some(Duration::from_secs(10)))?;
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
        loop {
            if counter == max_retry {
                error!("达到最大重试次数 {}，终止程序", counter);
                std::process::exit(-1);
            }
            if let Err(e) = self.chanllenge().and_then(|_| self.send_login()) {
                counter += 1;
                let wait = DELAY * 2_u64.pow(counter as u32);
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
                    // KeepAlive 错误不会在这出现
                    _ => {}
                }
                thread::sleep(Duration::from_secs(wait));
            } else {
                break;
            }
        }
        info!("登录成功({})", &self.conf.account.username);
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
        let send_size = self.send(&knock)?;
        debug!("challenge sent({}) {:?}", send_size, &knock);

        let response = self.recv()?;
        debug!("challenge recv({}) {:?}", response.len(), &response);

        if !response.starts_with(b"\x02") {
            warn!("challenge err({}) {:?}", response.len(), &response);
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
        let send_size = self.send(&ticket)?;
        debug!("login sent({}) {:?}", send_size, &ticket);

        let response = self.recv()?;
        debug!("login recv({}) {:?}", response.len(), &response);

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

    pub fn empty_socket_buffer(&self) {
        loop {
            let mut buf = Vec::new();
            if let Err(_) = self.pipe.recv(&mut buf) {
                info!("套接字缓冲区已清空");
                break;
            } else {
                info!("正在清空套接字缓冲区");
            }
        }
    }

    pub fn keep_alive(&mut self) -> Result<(), DrcomException> {
        let max_retry = self.conf.behavior.max_retry;
        let mut counter = 0;
        loop {
            if counter == max_retry {
                error!("达到最大重试次数，终止程序");
                std::process::exit(-1);
            }
            counter += 1;
            let wait = DELAY * 2_u64.pow(counter as u32);
            match self.keep_alive_1() {
                Ok(()) => break,
                Err(DrcomException::KeepAlive1) => {
                    error!("keep_alive_1 error");
                    warn!("{} 秒后重试", wait);
                    thread::sleep(Duration::from_secs(wait));
                    continue;
                }
                Err(o) => return Err(o),
            }
        }

        let srv_num = 0;
        let srv_num = self.keep_alive_2(srv_num)?;
        let (srv_num, tail) = self.keep_alive_3(srv_num)?;
        let (srv_num, tail) = self.keep_alive_4(srv_num, tail)?;
        thread::sleep(Duration::from_secs(20));

        self.keep_alive_stable(srv_num, tail)
    }

    fn keep_alive_1(&self) -> Result<(), DrcomException> {
        let pack = make_keep_alive_packet_1(&self.salt, &self.conf.account.password, &self.token);
        let send_size = self.send(&pack)?;
        debug!("keep_alive_1 sent ({}) {:?}", send_size, &pack);

        let response = self.recv()?;
        debug!("keep_alive_1 recv({}) {:?}", response.len(), &response);

        if !response.starts_with(b"\x07") {
            Err(DrcomException::KeepAlive1)
        } else {
            Ok(())
        }
    }

    fn keep_alive_2(&self, srv_num: u8) -> Result<u8, DrcomException> {
        let pack: Vec<u8> = make_keep_alive_packet_2(srv_num);

        let send_size = self.send(&pack)?;
        debug!("keep_alive_2 sent ({}) {:?}", send_size, &pack);

        let response = self.recv()?;
        debug!("keep_alive_2 recv({}) {:?}", response.len(), &response);

        if !response.starts_with(b"\x07") {
            return Err(DrcomException::KeepAlive2);
        } else if response.starts_with(b"\x07\x00\x28\x00")
            || response.starts_with(&[0x07, srv_num, 0x28, 0x00])
            || (&response[2] == &0x10u8)
        {
            let srv_num = srv_num + 1;
            trace!("srv_num={}", srv_num);
        }
        return Ok(srv_num);
    }

    fn keep_alive_3(&self, srv_num: u8) -> Result<(u8, [u8; 4]), DrcomException> {
        let pack: Vec<u8> = make_keep_alive_packet_3(srv_num, &self.conf.signal.keep_alive_version);

        let send_size = self.send(&pack)?;
        debug!("keep_alive_3 sent ({}) {:?}", send_size, &pack);

        let response = self.recv()?;
        debug!("keep_alive_3 recv({}) {:?}", response.len(), &response);

        if !response.starts_with(b"\x07") {
            return Err(DrcomException::KeepAlive3);
        }
        let srv_num = (srv_num % 0x7f) + 1;
        let mut tail = [0; 4];
        tail.copy_from_slice(&response[16..20]);

        trace!("srv_num={}", srv_num);
        trace!("tail={:?}", &tail);

        Ok((srv_num, tail))
    }

    fn keep_alive_4(&self, srv_num: u8, tail: [u8; 4]) -> Result<(u8, [u8; 4]), DrcomException> {
        let pack: Vec<u8> = make_keep_alive_packet_4(
            srv_num,
            &tail,
            &self.conf.server.host_ip,
            &self.conf.signal.keep_alive_version,
        );

        let send_size = self.send(&pack)?;
        debug!("keep_alive_4 sent ({}) {:?}", send_size, &pack);

        let response = self.recv()?;
        debug!("keep_alive_4 recv({}) {:?}", response.len(), &response);

        if !response.starts_with(b"\x07") {
            return Err(DrcomException::KeepAlive4);
        }

        let srv_num = (srv_num % 0x7f) + 1;
        let mut tail = [0; 4];
        tail.copy_from_slice(&response[16..20]);
        trace!("srv_num={}", srv_num);
        trace!("tail={:?}", &tail);

        return Ok((srv_num, tail));
    }

    fn keep_alive_stable(&self, srv_num: u8, tail: [u8; 4]) -> Result<(), DrcomException> {
        let (mut srv_num, mut tail) = (srv_num, tail);
        info!("开始稳定心跳");
        loop {
            let (a, b) = self.keep_alive_3(srv_num)?;
            srv_num = a;
            tail.copy_from_slice(&b);

            let (a, b) = self.keep_alive_4(srv_num, tail)?;
            srv_num = a;
            tail.copy_from_slice(&b);
            thread::sleep(Duration::from_secs(20));
        }
    }

    fn send(&self, msg: &[u8]) -> Result<usize, DrcomException> {
        let send_size = self.pipe.send(msg)?;
        Ok(send_size)
    }

    fn recv(&self) -> Result<Vec<u8>, DrcomException> {
        let mut recv_buf = [0u8; 1024];
        let recv_size = self.pipe.recv(&mut recv_buf)?;
        let mut msg = Vec::with_capacity(recv_size);
        msg.extend_from_slice(&recv_buf[..recv_size]);
        Ok(msg)
    }
}

fn make_keep_alive_packet_1(salt: &[u8; 4], password: &String, token: &[u8; 16]) -> Vec<u8> {
    let mut packet = Vec::with_capacity(44);

    let now: u16 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("无法读取系统计时器")
        .as_secs() as u16;

    // 1, 1
    packet.push(0xff);
    // 16, 17
    let mut buf = Md5::new();
    buf.update(b"\x03\x01");
    buf.update(&salt);
    buf.update(password.as_bytes());
    let md5sum = buf.finalize();
    packet.extend_from_slice(&md5sum.as_slice());
    // 3, 20
    packet.extend_from_slice(&[0; 3]);
    // 16, 36
    packet.extend_from_slice(token);
    // 4, 40
    packet.extend_from_slice(&now.to_be_bytes());
    // 4, 44
    packet.extend_from_slice(&[0; 4]);

    return packet;
}

fn make_keep_alive_packet_2(srv_num: u8) -> Vec<u8> {
    let mut buf = Vec::with_capacity(40);
    buf.extend_from_slice(&[
        0x07, srv_num, 0x28, 0x00, 0x0b, 0x01, 0x0f, 0x27, 0x2f, 0x12,
    ]);
    buf.extend_from_slice(&[0; 30]);
    return buf;
}

fn make_keep_alive_packet_3(srv_num: u8, keep_alive_version: &[u8; 2]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&[0x07, srv_num, 0x28, 0x00, 0x0b, 0x01]);
    buf.extend_from_slice(keep_alive_version);
    buf.extend_from_slice(&[0x2f, 0x12]);
    buf.extend_from_slice(&[0; 30]);
    return buf;
}

fn make_keep_alive_packet_4(
    srv_num: u8,
    tail: &[u8; 4],
    host_ip: &String,
    keep_alive_version: &[u8; 2],
) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&[0x07, srv_num, 0x28, 0x00, 0x0b, 0x03]);
    buf.extend_from_slice(keep_alive_version);
    buf.extend_from_slice(&[0x2f, 0x12]);
    buf.extend_from_slice(&[0; 6]);
    buf.extend_from_slice(tail);
    buf.extend_from_slice(&[0; 8]);
    let host_ip: Vec<u8> = host_ip
        .split(".")
        .map(|pat| pat.parse::<u8>().unwrap())
        .take(4)
        .collect();
    buf.extend_from_slice(&host_ip);
    buf.extend_from_slice(&[0; 8]);
    return buf;
}
