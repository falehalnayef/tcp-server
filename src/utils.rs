pub fn log_request(method: &str, path: &str, peer_addr: &str) {
    println!("{} {} from {}", method, path, peer_addr);
}

