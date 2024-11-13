mod features;
use clap::{command, Command, Parser, Subcommand, Arg};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("HTML")
                .about("Get the html content form the first page in the target URL")
                .arg(Arg::new("url").required(true).help("The URL to analyze"))
                .long_flag("html")
        )
        .get_matches();

    // arguments.subcommand() returns a Option<(&str, ArgMatches)>
    // We then match the "argument" and the value from ArgMatches (sub) and call the respective
    // method from features.rs
    match arguments.subcommand() {
        Some(("HTML", sub)) =>
            features::get_html(sub.get_one::<String>("url")
            .unwrap()),
        _ => unreachable!(),
    }
}
