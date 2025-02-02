use llm_chatbot::run_chat_loop;

// reqwest for making HTTP requests
use reqwest::Client;

use std::env;


#[tokio::main]
async  fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();

    let api_key = match env::var("") {
        Ok(key) => key,
        Err(_) => {
            println!("Key not working");
            String::from("error from the key")
        }
    };

    let url = "";

    run_chat_loop(&client, &api_key, url).await?;
    
    Ok(())
}
