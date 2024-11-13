
// Default output for when rusty-scraper is called without any additional flags
// pub fn get__info(url: &str) -> Result<(), Box<dyn std::error::Error>> {
     // todo
// }

// Command for when the --html command is called
// Gives the raw html content
pub fn get_html(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)
        .unwrap()
        .text()
        .unwrap();
    let html_document = scraper::Html::parse_document(&response);

    // todo
    // Append it to a .html file so that the user gets the html file
    println!("This is the Html page for {}\
    \
    \
    {}", url, html_document.html());

    Ok(())
}

