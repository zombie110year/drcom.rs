use drcom::default_config;
use std::fs::write;

fn main() {
    let x = default_config();
    let toml: String = toml::to_string(&x).unwrap();
    write("drcom.default.toml", &toml).unwrap();
    println!("{}", toml);
    println!("save default config as 'drcom.default.toml'");
}
