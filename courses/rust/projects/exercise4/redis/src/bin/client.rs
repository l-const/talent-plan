use log::{info, trace};
use simple_redis::{init_logger, Client};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger(env!("CARGO_BIN_NAME"));
    let port = std::env::var("REDIS_PORT").unwrap_or("6379".into());
    let mut client =
        Client::new(format!("{}{}", "127.0.0.1:", port)).expect("Failed to initialize the client!");
    trace!("client was initialised: {:?}", &client);
    let buf = [0; 7];
    let parsed_string = client.send::<7>(b"PING", buf)?;
    info!("Parsed str: {}", &parsed_string);
    Ok(())
}
