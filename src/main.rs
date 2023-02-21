use std::fmt::format;

use reqwest;

async fn get_all_cards() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://arkhamdb.com/api/public/cards/")
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{:#?}", response);
    Ok(())
}
async fn get_card(value: &str) -> Result<(), reqwest::Error> {
    let url = format!("https://arkhamdb.com/api/public/card/{}", value);
    let response = reqwest::get(&url)
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{:#?}", url);
    println!("{:#?}", response);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // get_all_cards().await?;
    get_card("01001").await?;
    Ok(())
}
