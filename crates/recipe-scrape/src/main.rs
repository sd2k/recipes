use std::env::args;

#[tokio::main]
async fn main() {
    let url = args()
        .nth(1)
        .expect("expected URL to scrape")
        .parse()
        .unwrap();
    let scraper = recipe_scrape::RecipeScraper::new();
    let recipe = scraper.scrape(url).await.unwrap();
    println!("{:#?}", recipe);
}
