#[macro_use]
extern crate log;

mod config;
mod app;

pub use config::{default_config, load_config};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
