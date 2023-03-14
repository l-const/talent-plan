use log::info;
use simple_redis::init_logger;
fn main() {
    init_logger(env!("CARGO_BIN_NAME"));
    info!("Hello from server")
}