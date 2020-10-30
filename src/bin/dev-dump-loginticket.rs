use drcom::app::make_login_ticket;

fn main() {
    let d = make_login_ticket(
        &String::from("HelloWorld"),
        &String::from("HateDrcom"),
        b"\x00\x00\x00\x00",
        0x20,
        7,
        0x123456789012,
        &String::from("192.168.1.1"),
        1,
        &String::from("DRCOM Client"),
        &String::from("8.8.8.8"),
        &String::from("0.0.0.0"),
        &String::from("Windows 10"),
        b"\x0a\x00",
    );
    println!("In bytes({})", d.len());
    hexdump(d.as_slice());
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