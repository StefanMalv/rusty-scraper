mod features;
use tokio;
use std::io::{ ErrorKind };
use clap::{ Command, Arg };
use reqwest::Client;

// Struct for creating an argument
struct Argument {
    command: CommandType,
    url: String,
    flags: Vec<String>,
}

// Enum for the type of command being given
// Possible to add more types for when I add new functionality
enum CommandType {
    HtmlPage,
    FileStructure,
    ErrCommand(ErrorKind),
}

// Main function for creating and handling arguments given
#[tokio::main]
async fn main() {
    // Create client
    let client = Client::builder()
        .pool_max_idle_per_host(10)
        .build()
        .unwrap();

    // This creates the different arguments that can be given and its subcommands
    let arguments = Command::new("rusty-scraper")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("--html")
                .about("Get the HTML content from the first page in the target URL")
                .arg(
                    Arg::new("url")
                        .required(true)
                        .help("The URL to analyze"),
                ))
        .subcommand(
            Command::new("--tree")
                .about("Get file structure of website")
                .arg(
                    Arg::new("url")
                        .required(true)
                        .help("The URL to analyze"),
                ))
        .get_matches();
    // this part of the main function organizes the argument given into its individual
    // parts: main command, subcommands, url, flags
    // Note: considering on breaking this part into a command parsing function

    // commands and subcommands
    let (command, sub_arguments) = arguments
        .subcommand()
        .ok_or(ErrorKind::InvalidInput).unwrap();

    // Given url
    let url = sub_arguments
        .get_one::<String>("url")
        .ok_or(ErrorKind::InvalidInput)
        .unwrap()
        .to_string();

    // all the flags
    let flags: Vec<String> = vec![command.to_string()];

    // the main command
    let main_command = match_command(command).await;

    // the whole argument
    let argument = Argument {
        command: main_command,
        url,
        flags,
    };

    let result = run_commands(argument, &client).await;
    println!("{:?}", result);
}


// Processes the argument given and runs the respective functions from features.rs
async fn run_commands(argument: Argument, client: &Client) -> String {
    match argument.command {
        CommandType::HtmlPage => {
            features::get_html(&argument.url, client).await.unwrap_or_else(|err| {
                format!("Failed to fetch HTML: {}", err)
            })
        }
        CommandType::FileStructure => {
            let sites = features::crawl_webpage(&argument.url, client);
            sites
                .await
                .iter()
                .next()
                .unwrap()
                .to_string()
        }
        CommandType::ErrCommand(err) => {
            format!("Invalid command: {:?}", err)
        }
    }
}

//see line 67
async fn match_command(command: &str) -> CommandType {
    let matched = match command {
        "--html" => CommandType::HtmlPage,
        "--tree" => CommandType::FileStructure,
        _ => CommandType::ErrCommand(ErrorKind::InvalidData),
    };
    matched
}
