use log::{info, trace};
use simple_redis::init_logger;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger(env!("CARGO_BIN_NAME"));
    info!("Hello from server");
    let port = std::env::var("REDIS_PORT").unwrap_or("6379".into());
    let listener = TcpListener::bind(format!("{}{}", "127.0.0.1:", port))?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    trace!("Recieved client connection from stream: {:?}", stream);
    stream.set_nonblocking(true)?;
    stream.set_read_timeout(Some(std::time::Duration::from_millis(200)))?;
    let mut buf = [0; 7];
    let mut n = 0;
    let mut sleep_duration = std::time::Duration::from_nanos(10);
    loop {
        if let Err(e) = stream.read(&mut buf) {
            trace!("Error reading {:?}", e);
            if n > 100 {
                break;
            }
        }

        if let Ok(()) = stream.read_exact(&mut buf) {
            info!("Read bytes!");
            break;
        }
        // thread sleep backop until socket is ready!
        // wait until network socket is ready, typically implemented
        // via platform-specific APIs such as epoll or IOCP
        n +=1;
        trace!("About to sleep for {:?} ms.", sleep_duration);
        std::thread::sleep(sleep_duration);
        sleep_duration = std::time::Duration::from_nanos(2 * n);
    }

    let bytes_read_trimmed = std::str::from_utf8(&buf)
        .unwrap()
        .trim_end_matches(&['\r', '\n', '\0']);
    info!("Read bytes: {:?}", bytes_read_trimmed);
    stream.write(b"+PONG\r\n")?;
    stream.flush()?;
    stream.shutdown(std::net::Shutdown::Both)?;
    Ok(())
}


