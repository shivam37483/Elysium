use reqwest::{header, Client};
use serde_json::json;
use serde_json::Value;
use std::io;
use std::io::Write;


pub async fn run_chat_loop(client: &Client, api_key: &str, url: &str) -> Result<(), reqwest::Error> {
    let mut conversation = String::from("The following is a conversation with an AI assistant. The assistant is helpful, creative, clever, and very friendly.\n");
    
    loop {
        print!("Human: ");
        match io::stdout().flush() {
            Ok(_) => {},
            Err(_) => print!("Flushing did not work")
        }
        
        let user_input = read_user_input();
        
        
        if user_input.to_lowercase() == "quit" || user_input.to_lowercase() == "exit" {
            break;
        }
        
        
        conversation.push_str("Human: ");
        conversation.push_str(&user_input);
        conversation.push_str("\nAI: ");
        
        let json = json!({
            "model": "text-davinci-003",
            "prompt": conversation,
            "temperature": 0.9,
            "max_tokens": 150,
            "top_p": 1.0,
            "frequency_penalty": 0.0,
            "presence_penalty": 0.6,
            "stop": [" Human:", " AI:"]
        });
        
        
        let body = call_api(client, api_key, url, json).await?;
        
        let ai_response = get_ai_response(&body);
        
        println!("AI: {ai_response}");
        
        conversation.push_str(ai_response);
        conversation.push('\n');
    }
    
    Ok(())
}


pub fn read_user_input() -> String {
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {},
        Err(_) => print!("Error in reading input"),
    };

    user_input.trim().to_string()    
}


pub fn get_ai_response(body: &Value) -> &str {
    if let Some(response) = body["choices"][0]["text"].as_str() {
        response.trim()
    } else {
        "No choice or text in json"
    }
}


pub async fn call_api(client: &Client, api_key: &str, url: &str, json: serde_json::Value) -> Result<Value, reqwest::Error> {
    let response= client
                                .post(url)
                                .header(header:: AUTHORIZATION, format!("Bearer {}", api_key))
                                .header(header:: CONTENT_TYPE, "application.json")
                                .json(&json)
                                .send()
                                .await?;

    let body = response.json().await?;

    Ok(body)
}

