fn main() {
    env_logger::init();

    let conf = drcom::load_config("drcom.dev.toml").unwrap();
    let mut app = drcom::app::Drcom::new(conf).unwrap();
    app.login();
    app.keep_alive().unwrap();
}
