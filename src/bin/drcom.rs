#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
    info!("drcom start");

    println!(
        "{:#?}",
        drcom::load_config("drcom.default.toml").unwrap()
    );
}
