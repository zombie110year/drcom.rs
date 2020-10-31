use drcom::app::make_login_ticket;
use std::fs::write;

fn main() {
    let d = make_login_ticket(
        &String::from("20172744"),
        &String::from("zxcvbnmasdf"),
        b"/\xc3|\x00",
        0x20,
        7,
        0xEC4118D666AF,
        &String::from("172.25.148.82"),
        1,
        &String::from("XiaoMi Router"),
        &String::from("8.8.8.8"),
        &String::from("0.0.0.0"),
        &String::from("Linux"),
        b"\x0a\x00",
    );
    println!("In bytes({})", d.len());
    hexdump(d.as_slice());
    write("./rust.bin", d).unwrap();
}

fn hexdump(buf: &[u8]) {
    for chunk in buf.chunks(16) {
        print!("\n{:02x}", chunk[0]);
        for c in chunk[1..].iter() {
            print!(" {:02x}", c);
        }
    }
    println!();
}