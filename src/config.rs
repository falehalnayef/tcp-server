use dotenv::dotenv;
use std::env;

pub fn get_server_address() -> String {
    dotenv().ok();
    env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string())
}
