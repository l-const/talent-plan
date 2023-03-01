use clap::{Arg, App, SubCommand};

#[derive(Debug)]
struct ParsedEnvVars {
    author: &'static str,
    app_name: &'static str,
    version: &'static str,
    description: &'static str,
}


fn main() {
    let args: ParsedEnvVars = init_env_vars();
    let matches = App::new(args.app_name)
                        .author(args.author)
                        .version(args.version)
                        .about(args.description)
                        .get_matches();
                        
}

fn init_env_vars() -> ParsedEnvVars {
    let author = env!("CARGO_PKG_AUTHORS");
    let app_name = env!("CARGO_BIN_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let description = env!("CARGO_PKG_DESCRIPTION");
    dbg!(author, app_name, version, description);
    ParsedEnvVars { author, app_name, version, description }
}
