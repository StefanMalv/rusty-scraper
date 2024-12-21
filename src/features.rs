use scraper::{ Html };
enum WebPage {
    Header(String),
    Body(String),
    Footer(String),
    Links(String),
}

//Default output for when rusty-scraper is called without any additional flags
pub fn get_webpage_info(url: &str, flag: &Vec<String>) -> String {
    let document = match get_html(url) {
        Ok(doc) => doc,
        Err(e) => return format!("Error fetching HTML: {}", e),
    };
    let html_page = Html::parse_document(&document);
    let flags: Vec<String> = Vec::new();

    // let body = Selector::parse("body").unwrap();
    // for body in html_page.select(&body) {
    //     println!("This is the body of the webpage \
    //              {}", body.inner_html());
    // }
    todo!()
    // Finish this method so that it returns all the metadata and connect the methods to main
    // try not using redundant for loops
}

// Command for when the --html command is called
// Gives the raw html content

pub fn get_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)
        .unwrap()
        .text()
        .unwrap();
    let html_document = Html::parse_document(&response);

    Ok(html_document.html())
}

