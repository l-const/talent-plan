use clap::{App, Arg};


#[derive(Debug)]
struct ParsedEnvVars {
    author: &'static str,
    app_name: &'static str,
    version: &'static str,
    description: &'static str,
}

fn main() {
    println!("Hello from kvs-server!");
    let env_vars = init_env_vars();
    let matches = App::new(env_vars.app_name)
        .author(env_vars.author)
        .version(env_vars.version)
        .about(env_vars.description)
        .arg(Arg::with_name("addr")
                .long("addr")
                .help("IP-PORT")
                .default_value("127.0.0.1:4000")
                .required(true))
        .arg(Arg::with_name("engine")
                .long("engine")
                .default_value("kvs")
                .help("ENGINE-NAME")
                .validator(|s| {
                    if !s.eq("kvs") && !s.eq("sled") {
                        Err(String::from("Engine name not part of possible engine names."))
                    } else  {
                        Ok(())
                    }
                })
                .required(true)
            )
        .get_matches();
    let addr = matches.value_of("addr").unwrap();
    let engine = matches.value_of("engine").unwrap();
    dbg!(addr);
    dbg!(engine);
}


const fn init_env_vars() -> ParsedEnvVars {
    let author = env!("CARGO_PKG_AUTHORS");
    let app_name = env!("CARGO_BIN_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let description = env!("CARGO_PKG_DESCRIPTION");
    ParsedEnvVars {
        author,
        app_name,
        version,
        description,
    }
}