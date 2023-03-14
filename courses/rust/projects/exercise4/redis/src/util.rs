use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};


pub fn init_logger(logfile_name: &str) {
    let level = log::LevelFilter::Info;
    let cur_dir  = std::env::current_dir().expect("failed to load current_dir");
    let temp_dir =  cur_dir.join("tmp/");
    std::fs::create_dir_all(&temp_dir).expect("Failed to create tmp directory!");
    let log_file_path = temp_dir.join(format!("{}.log", logfile_name));
    // Build a stdout logger
    let stdout = ConsoleAppender::builder().target(Target::Stdout).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::default()))
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