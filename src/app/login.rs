use md5::{Digest, Md5};

const MD5_LEN: usize = 16;
const ACCOUNT_MAX_LEN: usize = 36;
const MAC_LEN: usize = 6;
const HOST_MAX_IP_NUM: usize = 4;
const HOST_NAME_MAX_LEN: usize = 32;

pub fn make_login_ticket(
    username: &String,
    password: &String,
    salt: &[u8; 4],
    control_check_status: u8,
    adapter_num: u8,
    mac: u64,
    host_ip: &String,
    ip_dog: u8,
    host_name: &String,
    dns: &String,
    dhcp: &String,
    host_os: &String,
    auth_version: &[u8; 2],
) -> Vec<u8> {
    let mut ticket = Vec::with_capacity(330);

    // 4, 4
    let header = (username.len() as u32 + 20) << 24 + 0x0103;
    ticket.extend_from_slice(&header.to_le_bytes());

    // 16, 20
    let password_md5 = {
        let mut password_md5 = Md5::new();
        password_md5.update(b"\x03\x01");
        password_md5.update(&salt);
        password_md5.update(password.as_bytes());
        password_md5.finalize().as_slice()
    };
    ticket.extend_from_slice(password_md5);

    // 36, 56
    let account: [u8; ACCOUNT_MAX_LEN] = {
        let mut buf = [0; ACCOUNT_MAX_LEN];
        buf.copy_from_slice(username.as_bytes());
        buf
    };
    ticket.extend_from_slice(&account);

    // 1, 57
    ticket.push(control_check_status);
    // 1, 58
    ticket.push(adapter_num);

    // 6, 64
    let xor_n = {
        let mut xor_n = [0; 8];
        xor_n[2..].copy_from_slice(&password_md5[..6]);
        u64::from_be_bytes(xor_n)
    };
    let xored_mac = (xor_n ^ mac).to_be_bytes();
    ticket.extend_from_slice(&xored_mac);

    // 16, 80
    let password_md5_2 = {
        let mut buf = Md5::new();
        buf.update(b"\x01");
        buf.update(password.as_bytes());
        buf.update(&salt);
        buf.update(&[0_u8; 4]);
        buf.finalize().as_slice()
    };
    ticket.extend_from_slice(&password_md5_2[..MD5_LEN]);

    // 1, 81
    let host_ip_num = 1;
    ticket.push(host_ip_num);

    // 16, 97
    let host_ip: Vec<u8> = host_ip
        .split(".")
        .map(|pat| pat.parse::<u8>().unwrap())
        .take(4)
        .collect();
    ticket.extend_from_slice(&host_ip);
    ticket.extend_from_slice(&[0; 12]);

    // 8, 105
    let half_md5 = {
        let mut buf = Md5::new();
        let previous_data = &ticket[..97];
        buf.update(previous_data);
        buf.update(b"\x14\x00\x07\x0b");
        &buf.finalize().as_slice()
    };
    ticket.extend_from_slice(&half_md5[..(MD5_LEN / 2)]);

    // 1, 106
    ticket.push(ip_dog);

    // 4, 110
    ticket.extend_from_slice(&[0; 4]);

    // 32, 142
    let mut buf = [0; 32];
    buf.copy_from_slice(host_name.as_bytes());
    ticket.extend_from_slice(&buf);

    // 4, 146
    let dns_ip: Vec<u8> = dns
        .split(".")
        .map(|pat| pat.parse::<u8>().unwrap())
        .take(4)
        .collect();
    ticket.extend_from_slice(&dns_ip);
    let dhcp_ip: Vec<u8> = dhcp
        .split(".")
        .map(|pat| pat.parse::<u8>().unwrap())
        .take(4)
        .collect();
    ticket.extend_from_slice(&dhcp_ip);

    // 4*3=12, 162
    ticket.extend_from_slice(&[0; 12]);

    // 4, 166
    ticket.extend_from_slice(&148_u32.to_le_bytes());
    // 4, 170
    ticket.extend_from_slice(&5_u32.to_le_bytes());
    // 4, 174
    ticket.extend_from_slice(&1_u32.to_le_bytes());
    // 4, 178
    ticket.extend_from_slice(&0x0a28_u32.to_le_bytes());
    // 4, 182
    ticket.extend_from_slice(&2_u32.to_le_bytes());

    // 128, 310
    let mut service_pack = [0; 128];
    service_pack.copy_from_slice(host_os.as_bytes());
    ticket.extend_from_slice(&service_pack);

    // 2, 312
    ticket.extend_from_slice(auth_version);

    // 1, 313
    ticket.push(2);
    // 1, 314
    ticket.push(12);

    // 4, 318
    let mut crc_buf = [0; 326];
    crc_buf.copy_from_slice(&ticket[..314]);
    crc_buf[314..320].copy_from_slice(b"\x01\x26\x07\x11\x00\x00");
    crc_buf[320..].copy_from_slice(&mac.to_be_bytes()[..6]);
    let checksum = crc_sum(&crc_buf);
    ticket.extend_from_slice(&checksum.to_le_bytes());

    // 2, 320
    ticket.extend_from_slice(&[0; 2]);

    // 6, 326
    ticket.extend_from_slice(&mac.to_le_bytes()[..6]);

    // 1+1+2=4, 330
    ticket.extend_from_slice(b"\x00\x00\xe9\x13");

    return ticket;
}

pub(crate) fn crc_sum(buf: &[u8]) -> u32 {
    let chunks = buf.chunks(4);
    let mut sum = 1234;
    for chunk in chunks {
        let c = [0; 4];
        c.copy_from_slice(chunk);
        let num = u32::from_le_bytes(c);
        sum ^= num;
    }
    return sum * 1968;
}
