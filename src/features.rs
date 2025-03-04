use std::string::String;
use scraper::{ Html, Selector };
use std::collections::{ VecDeque };
use reqwest::Url;

// enum WebPage {
//     Header(String),
//     Body(String),
//     Footer(String),
//     Links(String),
// }

// Command for when the --html command is called
// Gives the raw html content of the page


pub async fn get_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?.text().await?;
    let html_document = Html::parse_document(&response);

    Ok(html_document.html())
}

pub async fn get_file_structure(url: String) -> String {
    //function for getting file structure of website
    let crawler = crawl_webpage(&url).await;

    crawler
        .into_iter()
        .next()
        .unwrap_or_else(|| "No structure found".to_string())
}

pub async fn crawl_webpage(url: &String) -> Vec<String> {
    // limit of sites crawled
    let limit: u32 = 100;
    // List of visited URLs
    let mut visited: Vec<String> = Vec::new();
    // Queue for URLs to visit
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(url.clone());

    // Process the queue until it's empty
    while let Some(current_url) = queue.pop_front() {
        // Skip if the URL has already been visited
        if visited.contains(&current_url) {
            continue;
        }
        if visited.len() == limit as usize {
            break
        }

        // Mark the URL as visited
        visited.push(current_url.clone());

        // Fetch the HTML content of the current URL
        if let Ok(page) = get_html(&current_url).await {
            // Extract links from the page
            let links = get_links(&current_url, &page).await;
            for link in links {
                // Add new links to the queue
                if !visited.contains(&link) {
                    queue.push_back(link.clone());
                }
            }
        }
    }
    // Return the list of visited URLs
    println!("{} sites visited", visited.len());
    for urls in visited.iter() {
        println!("{}", urls)
    }
    visited
}


pub async fn get_links(url: &str, page_content: &str) -> Vec<String> {
        // get the page content
        let page_content = Html::parse_document(page_content);
        // List of all the links gathered from the page
        let mut links: Vec<String> = Vec::new();

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

       links
}


