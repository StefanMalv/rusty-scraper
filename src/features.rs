use scraper::Selector;

//Default output for when rusty-scraper is called without any additional flags
pub fn get_webpage_info(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let document = get_html(url);
    let html_page = scraper::Html::parse_document(&document?);

    todo!()
    // Implement the rest for the functionality
    // get:
    // header, Body, links, footer, etc
}

// Command for when the --html command is called
// Gives the raw html content
pub fn get_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)
        .unwrap()
        .text()
        .unwrap();
    let html_document = scraper::Html::parse_document(&response);

    Ok(html_document.html())
}

