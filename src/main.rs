use std::env;
use reqwest;
use scraper::{Html, Selector};
use std::thread;
use std::time::Duration;
use url::Url;

enum CrawlState {
    Found,
    Continue,
    MaxSteps,
    Loop,
}

fn get_base_url(url: &String) -> Url {
    let mut url_obj = Url::parse(url).unwrap();
    match url_obj.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => return Url::parse("https://en.wikipedia.org/").unwrap(),
    }
    url_obj.set_query(None);
    url_obj
}

fn find_first_link(url: &String) -> Option<String> {
    let resp = reqwest::blocking::get(url).unwrap();
    if !resp.status().is_success() {
        return None;
    }

    let html = resp.text().unwrap();
    let document = Html::parse_document(&html);
    let p_selector = Selector::parse("p").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    for p in document.select(&p_selector) {
        match p.select(&a_selector).next() {
            // First link of first the paragraph
            Some(a) => match a.value().attr("href") {
                Some(href) => {
                    let href = href.to_owned();
                    // Skip citation
                    if !href.starts_with("#") {
                        let base_url = get_base_url(url);
                        let link = base_url.join(href.as_str()).unwrap();
                        return Some(link.as_str().to_owned());
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
    None
}

fn check_crawl_state(
    search_history: &Vec<String>,
    target_url: &String,
    max_steps: usize,
) -> CrawlState {
    match search_history.last() {
        None => CrawlState::Continue,
        Some(url) => {
            if url == target_url {
                CrawlState::Found
            } else {
                if search_history.len() > max_steps {
                    CrawlState::MaxSteps
                } else {
                    let count = search_history.iter().filter(|e| e == &url).count();
                    if count == 2 {
                        CrawlState::Loop
                    } else {
                        CrawlState::Continue
                    }
                }
            }
        }
    }
}

fn get_target_url() -> Option<String> {
    let mut args = env::args();
    args.next();
    args.next()
}

fn main() {
    let start_url = String::from("https://en.wikipedia.org/wiki/Special:Random");
    let target_url = get_target_url().unwrap_or(String::from("https://en.wikipedia.org/wiki/Philosophy"));
    let max_steps = 30;
    println!("Starting at: {}", start_url);
    println!("Target is: {}", target_url);
    println!("Starting...");

    let mut search_history = Vec::new();
    search_history.push(start_url);

    loop {
        match check_crawl_state(&search_history, &target_url, max_steps) {
            CrawlState::Found => {
                println!("Found the target article!");
                break;
            }
            CrawlState::MaxSteps => {
                println!("The search has gone on far too long. (abort)");
                break;
            }
            CrawlState::Loop => {
                println!("Got to an article visited before. (abort)");
                let current_url = search_history.last().unwrap();
                println!("> {}", current_url);
                break;
            }
            CrawlState::Continue => {
                let current_url = search_history.last().unwrap();
                println!("{}", current_url);
                let first_link = find_first_link(&current_url);
                match first_link {
                    None => {
                        println!("Got to an article with no links to go by. (aborting)");
                        break;
                    }
                    Some(link_url) => {
                        search_history.push(link_url.to_owned());
                        thread::sleep(Duration::from_secs(2));
                    }
                }
            }
        };
    }
}
