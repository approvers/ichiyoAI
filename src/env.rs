use dotenvy::dotenv;
use std::env;

pub fn load_env() {
    dotenv().ok();
}

pub fn get_env(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(e) => panic!("{}: {}", e, key),
    }
}
