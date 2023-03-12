use std::{fs, io::{Write, Read}};
use log::{info,warn, LevelFilter};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

use simple_redis::Client;

fn main() {
    println!("Hello, world!");
    init_logger();
    info!("Hello from the redis client!");
    let mut client = Client::new("127.0.0.1:6379").expect("failed to initialize the client!");
    info!("client was initialised: {:?}", &client);
    client.write(b"PING\r\n");
    client.flush().expect("failed to flush the client");
    let mut buf =  [0 as u8; 6];
    info!("bytes read: {:?}", &buf); 
    loop {
        
    }
    if let Err(e) = client.read_exact(&mut buf) {
        warn!("Error reading {:?}", e);
    }

   
    info!("bytes read: {:?}", &buf); 
}


fn init_logger() {
    let level = log::LevelFilter::Info;
    fs::create_dir_all("./tmp").expect("Failed to create tmp directory!");
    let log_file_path = "./tmp/client.log";

    // Build a stdout logger
    let stdout = ConsoleAppender::builder().target(Target::Stdout).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(log_file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stdout", Box::new(stdout)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stdout")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config).expect("failed to init config");

    
}
