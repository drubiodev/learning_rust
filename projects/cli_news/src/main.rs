use serde::Deserialize;
use std::error::Error;

const API_URL:&str = "https://newsapi.org/v2/top-headlines?country=us&category=business&apiKey=7c13465f9524488ca298a43871b1d4c7";

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}
#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(API_URL).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}

fn main() {
    let articles = get_articles(API_URL);
    dbg!(articles);
}
