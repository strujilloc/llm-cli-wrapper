extern crate clap;
extern crate reqwest;

use clap::Parser;
use dotenv;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "A basic example",
    long_about = "A basic example with long about"
)]
struct Args {
    query: String,
    #[arg(short, long, default_value = "grammar")]
    context: String,
}
#[derive(Serialize, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}
#[derive(Serialize, Deserialize)]
struct Choice {
    message: Message,
}
#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}
fn validate_context(value: &str) -> Result<(), String> {
    let valid_contexts = ["grammar", "assistant"];
    if valid_contexts.contains(&value) {
        Ok(())
    } else {
        Err(format!(
            "Invalid context: {}. Valid contexts are: [{}]",
            value,
            valid_contexts.join(", ")
        ))
    }
}
fn get_system_prompt(context: &str) -> String {
    match context {
        "grammar" => "You are a grammar assistant, respond with the correct grammar. Maintain the same sentiment of the original query. Do not add additional length".to_string(),
        "assistant" => "You are a helpful assistant".to_string(),
        _ => panic!("Invalid context"),
    }
}

async fn request_completion(query: &str, system_prompt: &str) -> Result<String, String> {
    let token_api = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let llm_model = std::env::var("LLM_MODEL").expect("LLM_MODEL not set");
    let client = reqwest::Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", token_api).parse().unwrap(),
    );

    let body = json!({
      "model": llm_model,
      "messages": [
        {
          "role": "system",
          "content": system_prompt
        },
        {
          "role": "user",
          "content": query
        }
      ]
    });
    let response = client
        .post(url)
        .headers(headers)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Error: {}", e))?;
    let response_text = response.text().await.map_err(|e| format!("Error: {}", e))?;
    // Deserialize response
    let parsed_response: OpenAIResponse = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse JSON response: {}", e))?;
    if let Some(choice) = parsed_response.choices.first() {
        Ok(choice.message.content.clone())
    } else {
        Err("No response from OpenAI".to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv::dotenv().ok();
    let args = Args::parse();
    let context = args.context;
    // validate Arguments
    validate_context(&context)?;

    // get system prompt
    let system_prompt = get_system_prompt(&context);
    let response = request_completion(&args.query, &system_prompt).await?;
    println!("{}", response);
    Ok(())
}
