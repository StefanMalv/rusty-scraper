use scraper::{ Html, Selector };
use std::collections::{ VecDeque, HashSet };
use clap::error::ContextValue::String;
use reqwest::Url;
use crate::CommandType::HtmlPage;

enum WebPage {
    Header(String),
    Body(String),
    Footer(String),
    Links(String),
}

// Command for when the --html command is called
// Gives the raw html content of the page


pub fn get_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)
        .unwrap()
        .text()
        .unwrap();
    let html_document = Html::parse_document(&response);

    println!("{:?}", html_document.html());
    Ok(html_document.html())
}

pub fn get_file_structure(url: String) {
    //fucniton for getting file structure of website

    match crawl_webpage(&url) {
        Ok(_) => println!("Crawling"),
        Err(_) => println!("Crawl_failed")
    }
}

fn crawl_webpage(url: &str) {
    // Initialize structure for keeping track of visited
    // Keeps track of alle visited pages so that I don't repeat a visit
    let mut visited = HashSet::new();
    // Keeps track of all the links in the page
    let mut queue = VecDeque::new();
    queue.push_front(url);

    while let Some(url) = queue.pop_front() {
        if visited.contains(url) {
            continue;
        }
    }

    // Visit page
    println!("Visiting: {}", url);
    visited.push(url.clone());
    // Get links
    let page = get_html(url).unwrap();
    let links = get_links(url, &page);

    // Push all links from page into the queue
    get_links(url, page);
}

fn get_links(url: &str, page_content: &str) -> Result<Vec<String>, String>{
    // get the page content
    let page_content = Html::parse_document(page_content);
    // List of all the links gathered from the page
    let mut links: Vec<String> = Vec::new();
    // Error message
    let error_message: String = "Could not get links".into_string();

    // base URL
    let base_url = Url::parse(url).expect("Invalid base URL");
    // selector
    let selector = Selector::parse("a").unwrap();

    for link in page_content.select(&selector) {
        if let Some(href_tag) = link.value().attr("href") {
            if let Ok(full_url) = base_url.join(href_tag) {
                links.push(full_url.to_string())
            }
        }
    }

    match Ok(links){
        Ok(links) => Ok(links),
        Err(E) => error_message,
    }
}


