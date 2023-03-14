use log::{info, trace, LevelFilter};
use simple_redis::{Client, init_logger};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger(env!("CARGO_BIN_NAME"));
    let mut client = Client::new("127.0.0.1:6379").expect("Failed to initialize the client!");
    trace!("client was initialised: {:?}", &client);
    let buf = [0; 7];
    let parsed_string = client.send::<7>(b"PING", buf)?;
    info!("Parsed str: {}", &parsed_string);
    Ok(())
}


