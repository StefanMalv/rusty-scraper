extern crate core;

mod crawler;
mod httpserver;

use tokio;
use std::io::{ ErrorKind };
use clap::{ Command, Arg };
use reqwest::Client;
use clap::{ArgMatches};

// Struct for creating an argument
struct Argument {
    command: CommandType,
    input_argument: String,
}

// Enum for the type of command being given
// Possible to add more types for when I add new functionality
enum CommandType {
    HtmlPage,
    FileStructure,
    HttpServer,
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
                .about("Crawl all")
                .arg(
                    Arg::new("url")
                        .required(true)
                        .help("The URL to analyze"),
                ))
        .subcommand(
            Command::new("--http-server")
                .about("Spin up your own http server on localhost")
                .arg(
                    Arg::new("port")
                        .required(true)
                        .help("The port to use"),
                ))
        .get_matches();
    // this part of the main function organizes the argument given into its individual
    // parts: main command, subcommands, url, flags
    // Note: considering on breaking this part into a command parsing function

    let subcommand = handle_subcommands(arguments);

    run_commands(subcommand.await, &client).await;
}


// Processes the argument and runs the respective function from the crawler
async fn run_commands(argument: Argument, client: &Client) {
    match argument.command {
        CommandType::HtmlPage => {
            // Try to fetch HTML page
            let html_page = crawler::get_html(&argument.input_argument, client)
                .await.unwrap_or_else(
                |err| format!("Failed to fetch HTML: {}", err
                ));
            println!("{}", html_page)
        }

        CommandType::FileStructure => {
            // Crawl webpage and get a set of links
            let sites = crawler::crawl_webpage(&argument.input_argument, client).await;

            // Get one link to display, or default message
            match sites.iter().next() {
                Some(link) => println!("{}", link),
                None => panic!("No links found during crawl."),
            }
        }

        CommandType::HttpServer => {
            httpserver::create_server()
        }

        CommandType::ErrCommand(err) => {
            panic!("Invalid command: {:?}", err)
        }
    }
}


//see line 67
async fn match_command(command: &str) -> CommandType {
    let matched = match command {
        "--html" => CommandType::HtmlPage,
        "--tree" => CommandType::FileStructure,
        "--http-server" => CommandType::HttpServer,
        _ => CommandType::ErrCommand(ErrorKind::InvalidData),
    };
    matched
}

async fn handle_subcommands(argument: ArgMatches) -> Argument {
    // commands and subcommands
    let (command, sub_arguments) = argument
        .subcommand()
        .ok_or(ErrorKind::InvalidInput).unwrap();

    let sub_args = match command {
        "--html" | "--tree" => {
            sub_arguments
                .get_one::<String>("url")
                .expect("Value None")
        }

        "--http-server" => {
            sub_arguments
                .get_one::<String>("port")
                .expect("Value None")
        }

        _ => {
            panic!("Unknown subcommand: {}", command);
        }
    };

    // the main command
    let main_command = match_command(command).await;

    // the whole argument
    let argument = Argument {
        command: main_command,
        input_argument: sub_args.to_string(),
    };

    argument
}

