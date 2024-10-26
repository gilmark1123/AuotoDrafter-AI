use dotenv::dotenv;
use serde_json::json;
use std::env;
use std::error::Error;

/// Produces chatbot responses
///
pub async fn chat_generate_with_data(
    questions: Vec<String>,
    answers: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|e| Box::<dyn Error>::from(format!("Missing OPENAI_API_KEY: {}", e)))?;
    // Create the content to send to O

    // Combine questions and answers into the conversation history
    let conversation_history = questions
        .iter()
        .zip(answers.iter())
        .map(|(q, a)| format!("Q: {}\nA: {}", q, a))
        .collect::<Vec<String>>()
        .join("\\n");

    // Friendly follow-up prompt
    let follow_up_prompt = "\\nIs there anything else you'd like to add?";

    // Update request body to include full history and let OpenAI analyze what’s missing
    let request_body = json!( {
        "model": "gpt-4o-mini",
        "messages": [{
            "role": "system",
            "content": "You are an assistant designed to help users gather information for writing a letter. Based on the user's entire conversation history, analyze the details and ask about any missing information.You also need to ask basic information about the name of recipient , details of the recipient , who will be sending the letter, does the letter need to be confirmed by someone and the most important to provide the type of the letter and its content. Ensure consistency and do not ask for previously provided details. The response should only be asking 1-3 questions at a time. Prioritize your response in questioning about the main content of the letter. Make sure that your responses will be like conversation just provide 2-5 sentence to you response"
        }, {
            "role": "user",
            "content": format!("Conversation history so far:\n{}{}", conversation_history, follow_up_prompt)
        }],
        "max_tokens": 1024
    });
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    let response_json: serde_json::Value = response.json().await?;

    if let Some(choice_content) = response_json
        .get("choices")
        .and_then(|choices| choices.get(0))
        .and_then(|choice| choice.get("message"))
        .and_then(|message| message.get("content"))
        .and_then(|content| content.as_str())
    {
        Ok(choice_content.to_string())
    } else {
        Err("No valid response generated.".into()) // Return a boxed error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test] // Mark the test as async using tokio's test framework
    async fn testing_chat_manual() {
        let questions: Vec<String> = vec![
            "Hi there! Let's work on creating a letter. Could you provide me details on this letter?".to_string(),
        ];
        let answers: Vec<String> =
            vec!["I want to make a demand letter about Stephen Lewis.".to_string()];

        // Await the asynchronous function call and handle the result
        match chat_generate_with_data(questions, answers).await {
            Ok(response) => {
                println!("Response: {}", response); // Print the successful response
                assert!(!response.is_empty(), "Response should not be empty");
            }
            Err(e) => {
                println!("Test failed with error: {:?}", e); // Handle potential error
            }
        }
    }
}
