mod features;

use std::io::{ ErrorKind };
use clap::{Command, Arg, ArgMatches};

struct Argument {
    command: CommandType,
    flags:  Vec<(&'static str, &'static ArgMatches)>,
}

enum CommandType {
    Html((String, ArgMatches)),
    Meta((String, ArgMatches)),
    ErrCommand(ErrorKind),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = Command::new("MyApp")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("html")
                .about("Get the HTML content from the first page in the target URL")
                .arg(
                    Arg::new("url")
                        .required(true)
                        .help("The URL to analyze"),
                ),
        )
        .subcommand(
            Command::new("meta")
                .about("Get metadata about the page")
                .arg(
                    Arg::new("url")
                        .required(true)
                        .help("The URL to analyze"),
                ),
        )
        .get_matches();


    let current_argument = Argument {
        command: match arguments.subcommand() {
            Some(("html", sub)) => CommandType::Html(("html".to_string(), sub.clone())),
            Some(("meta", sub)) => CommandType::Meta(("meta".to_string(), sub.clone())),
            _ => CommandType::ErrCommand(ErrorKind::InvalidData),
        },
        flags: vec![
            arguments.subcommand().unwrap_or_default()
        ],
    };

    todo!()
    // Finnish matching the commands that have been called to the relevant methods in features.rs
    // in the run commands function :)
}

fn run_commands(argument: CommandType, flag: Vec<(&'static str, &'static ArgMatches)>) -> String {
    todo!()
}