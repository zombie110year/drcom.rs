fn main() {
    env_logger::init();

    let conf = drcom::load_config("drcom.dev.toml").unwrap();
    let mut app = drcom::app::Drcom::new(conf).unwrap();
    app.login();
    app.empty_socket_buffer();
    app.keep_alive().unwrap();
}
