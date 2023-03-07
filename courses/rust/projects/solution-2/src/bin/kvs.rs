use clap::{App, AppSettings, Arg, SubCommand};
use kvs::{KvStore, Result};

#[derive(Debug)]
struct ParsedEnvVars {
    author: &'static str,
    app_name: &'static str,
    version: &'static str,
    description: &'static str,
}

fn main() -> Result<()> {
    let mut kvs = KvStore::open("./out")?;
    let env_vars: ParsedEnvVars = init_env_vars();
    let matches = App::new(env_vars.app_name)
        .author(env_vars.author)
        .version(env_vars.version)
        .about(env_vars.description)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommands(vec![
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("A string value")
                        .required(true),
                ),
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        ])
        .get_matches();

    match matches.subcommand() {
        ("set", Some(arg_matches)) => {
            let key = arg_matches.value_of("KEY").unwrap();
            let value = arg_matches.value_of("VALUE").unwrap();
            let res = kvs.set(key.into(), value.into());
            let res = handle_result(res);
            res
        }
        ("get", Some(arg_matches)) => {
            let key = arg_matches.value_of("KEY").unwrap();
            let res = kvs.get(key.into());
            let res = handle_result(res.map(|val| {
                if let None = val {
                    println!("Key not found")
                }
            }));
            res
        }
        ("rm", Some(arg_matches)) => {
            let key = arg_matches.value_of("KEY").unwrap();
            let res = kvs.remove(key.into());
            let res = handle_result(res);

            if let Err(_) = res {
                std::process::exit(1);
            }
            res
        }
        _ => unreachable!(),
    }
}

fn handle_result(res: Result<()>) -> Result<()> {
    if res.is_err() {
        let error = res.unwrap_err();
        let error_msg = &error.msg;
        println!("{error_msg}");
        Err(error)
    } else {
        res
    }
}

fn init_env_vars() -> ParsedEnvVars {
    let author = env!("CARGO_PKG_AUTHORS");
    let app_name = env!("CARGO_BIN_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let description = env!("CARGO_PKG_DESCRIPTION");
    // dbg!(author, app_name, version, description);
    ParsedEnvVars {
        author,
        app_name,
        version,
        description,
    }
}
