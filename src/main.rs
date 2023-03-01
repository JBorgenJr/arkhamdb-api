use reqwest;
use std::env;
use std::fs::File;
use std::io::Write;

// Retrieve all cards from ArkhamDB
async fn get_all_cards(filename: &str) -> Result<(), reqwest::Error> {
    let response: serde_json::Value = reqwest::get("https://arkhamdb.com/api/public/cards/")
        .await?
        .json::<serde_json::Value>()
        .await?;
    // println!("{}", response);

    write_to_file(&response, filename);

    Ok(())
}

// Retrieve single card from ArkhamDB
async fn get_card(value: &str, filename: &str) -> Result<(), reqwest::Error> {
    let url: String = format!("https://arkhamdb.com/api/public/card/{}", value);
    let response: serde_json::Value = reqwest::get(&url)
        .await?
        .json::<serde_json::Value>()
        .await?;
    // println!("{}", response);

    write_to_file(&response, filename);

    Ok(())
}

//TODO: Add additional functions or generic function to handle different arguments

// Write results to file
fn write_to_file(json_response: &serde_json::Value, filename: &str) -> std::io::Result<()> {
    let mut file: File = File::create(filename)?;
    file.write_all(json_response.to_string().as_bytes())?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let filename: &str = "output.json";

    // get arguments from terminal
    let args: Vec<String> = env::args().collect();

    // TODO: Acceptable arg strings
    // all-cards
    // card-[card_number]

    // if there was a card code specified get that card else get all cards
    // TODO: This functionality is currently broken, always goes to else case
    if args.len() < 1 {
        let value: &String = &args[1];
        get_card(value, &filename).await?;
    } else {
        get_all_cards(&filename).await?;
    }

    Ok(())
}
