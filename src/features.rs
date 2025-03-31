use std::string::String;
use scraper::{ Html, Selector };
use std::collections::{ VecDeque, HashSet };
use futures::future::join_all;
use reqwest::{Client, Error, Url};
use tokio::{ task };

// Get html page for a url
pub async fn get_html(url: &str, client: &Client) -> Result<String, Error> {
    // client
    let response = client.get(url)
        .send()
        .await?;
    // html content
    let html = response.text().await?;
    Ok(html)
}

// Function for crawg webpages
pub async fn crawl_webpage(url: &String, client: &Client) -> HashSet<String> {
    // Limit of sites crawled
    let limit: u32 = 100;

    // List of visited URLs
    let mut visited: HashSet<String> = HashSet::new();

    // Queue for URLs to visit
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(url.clone());

    // Loop for getting webpages
    while !queue.is_empty() && visited.len() < limit as usize {
        // List off all tasks spawned
        let mut tasks = Vec::new();

        // Get the unvisited url
        while let Some(current_url) = queue.pop_front() {
            if visited.contains(&current_url) {
                continue;
            }
            // Marks it as visited
            visited.insert(current_url.clone());

            // Spawn async task to fetch HTML
            let client_clone = client.clone();
            let task = task::spawn(async move {
                get_html(&current_url, &client_clone).await.map(|page| (current_url, page))
            });

            // Push all tasks to list of tasks
            tasks.push(task);
        }

        // Wait for all fetches to complete
        let results = join_all(tasks).await;

        // goes through all the tasks in the list of tasks
        for result in results {
            // Not pretty, but it works, this nested result is because it comes from the async tokio task
            // and the inner Result comes from the unpacking of the HTTP request itself
            if let Ok(Ok((url, page))) = result {
                // gets all links from page
                let links = get_links(&url, &page).await;
                // This loop adds all links returned form result to the queue to be checked
                for link in links {
                    // adds the link to the queue if not visited
                    if !visited.contains(&link) {
                        queue.push_back(link);
                    }
                }
            }
        }
    }

    // Return the list of visited URLs
    println!("{} sites visited", visited.len());
    for url in &visited {
        println!("{}", url);
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