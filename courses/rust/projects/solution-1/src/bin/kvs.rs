use clap::{App, AppSettings, Arg, SubCommand};
use std::env;
use std::io::Write;
use std::process::exit;

#[derive(Debug)]
struct ParsedEnvVars {
    author: &'static str,
    app_name: &'static str,
    version: &'static str,
    description: &'static str,
}

fn main() {
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
            let key = arg_matches.value_of("KEY");
            let value = arg_matches.value_of("VALUE");
            dbg!(key, value);
            std::io::stderr()
                .write_all("unimplemented".as_bytes())
                .unwrap();
            exit(1);
        }
        ("get", Some(arg_matches)) => {
            let key = arg_matches.value_of("KEY");
            dbg!(key);
            std::io::stderr()
                .write_all("unimplemented".as_bytes())
                .unwrap();
            exit(1);
        }
        ("rm", Some(arg_matches)) => {
            let key = arg_matches.value_of("KEY");
            dbg!(key);
            std::io::stderr()
                .write_all("unimplemented".as_bytes())
                .unwrap();
            exit(1);
        }
        _ => unreachable!(),
    }
}

fn init_env_vars() -> ParsedEnvVars {
    let author = env!("CARGO_PKG_AUTHORS");
    let app_name = env!("CARGO_BIN_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let description = env!("CARGO_PKG_DESCRIPTION");
    dbg!(author, app_name, version, description);
    ParsedEnvVars {
        author,
        app_name,
        version,
        description,
    }
}
