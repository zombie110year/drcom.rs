use drcom::app::LoginTicket;
use std::mem::size_of;
fn main() {
    let d = LoginTicket::default();
    println!("LoginTicket({})", size_of::<LoginTicket>());
    let dumped = d.to_bytes();
    println!("In bytes({})", dumped.len());
}
