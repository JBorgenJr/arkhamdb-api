use reqwest;
use std::env;

// Retrieve all cards from ArkhamDB
async fn get_all_cards() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://arkhamdb.com/api/public/cards/")
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{:#?}", response);
    Ok(())
}

// Retrieve single card from ArkhamDB
async fn get_card(value: &str) -> Result<(), reqwest::Error> {
    let url = format!("https://arkhamdb.com/api/public/card/{}", value);
    let response = reqwest::get(&url)
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{:#?}", response);
    Ok(())
}

//TODO: Add additional functions or generic function to handle different arguments

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // get arguments from terminal
    let args: Vec<String> = env::args().collect();

    // TODO: Acceptable arg strings
    // all-cards
    // card-[card_number]

    // if there was a card code specified get that card else get all cards
    // TODO: This functionality is currently broken, always goes to else case
    if args.len() < 1 {
        let value = &args[1];
        get_card(value).await?;
    } else {
        get_all_cards().await?;
    }

    Ok(())
}
