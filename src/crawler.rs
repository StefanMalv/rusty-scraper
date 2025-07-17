use std::string::String;
use scraper::{ Html, Selector };
use std::collections::{VecDeque, BTreeSet};
use reqwest::{Client, Error, Url};
use std::time::{ Instant, Duration };
use futures::stream::{FuturesUnordered, StreamExt};



// Get html page for an url
pub async fn get_html(url: &str, client: &Client) -> Result<String, Error> {
    // client
    let response = client.get(url)
        .send()
        .await?;
    // html content
    let html = response.text().await?;
    Ok(html)
}

// Function for crawling webpages
pub async fn crawl_webpage(url: &String, client: &Client) -> BTreeSet<String> {
    // time the crawl
    let now = Instant::now();
    // Limit for links to be crawled
    let limit: usize = 100;

    // Use binary tree set to get sorted set of links
    let mut visited: BTreeSet<String> = BTreeSet::new();
    // Queue for links to be visited
    let mut queue: VecDeque<String> = VecDeque::new();
    // Links waiting to be visited
    // I use FuturesUnordered to process links concurrently
    let mut in_progress: FuturesUnordered<_> = FuturesUnordered::new();

    // Add links to the queue to be processed
    queue.push_back(url.clone());

    // Main loop for scraping
    while visited.len() < limit {
        // Fill up in_progress with tasks if queue is not empty
        while let Some(current_url) = queue.pop_front() {
            // Check if visited
            if visited.contains(&current_url) {
                continue;
            }

            visited.insert(current_url.clone());
            // Client for making requests
            let client_clone = client.clone();

            // Add fetch task to stream
            in_progress.push(async move {
                get_html(&current_url, &client_clone).await
                    .map(|page| (current_url, page))
            });

            // Limit the number of concurrent requests
            if in_progress.len() >= 5 {
                break;
            }
        }

        // Wait for any task to complete
        if let Some(result) = in_progress.next().await {
            if let Ok((url, page)) = result {
                let links = get_links(&url, &page).await;
                for link in links {
                    if !visited.contains(&link) {
                        queue.push_back(link);
                    }
                }
            }
        } else if queue.is_empty() {
            break;
        }
    }

    let elapsed = now.elapsed();
    print_summary(&visited, elapsed, url).await;
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

pub async fn print_summary(links: &BTreeSet<String>, time: Duration, start_url: &String) {
    println!("SUMMARY OF CRAWL \n");
    println!("{} Sites visited", links.len());
    for url in links.iter() {
        println!("{}", url);
    }
    println!("\n");
    println!("Start link: {}", start_url);
    println!("Crawling time {:?}", time);
    println!("Number of links crawled {}", links.len());
}