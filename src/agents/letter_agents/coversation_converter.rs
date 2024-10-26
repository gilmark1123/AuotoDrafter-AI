use dotenv::dotenv;
use serde_json::json;
use std::env;
use std::error::Error;
// use crate::letter_agents::content_generators::form_generator;
pub async fn values_generator(
    questions: Vec<String>,
    answers: Vec<String>,
) -> Result<Vec<String>, Box<dyn Error>> {
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

    // Update request body to include full history and let OpenAI analyze what’s missing
    let request_body = json!( {
        "model": "gpt-4o-mini",
        "messages": [{
            "role": "system",
            "content": format!("Your task is to generate values for the different parts of a formal letter based on the conversation's history of questions and answers. The response should be in array format, with the following parts in this exact order:

            1. **Date of the Letter** (formatted as 'Month D, YYYY', e.g., January 05, 2002). If the date is missing, insert '[Date of letter]' as a placeholder. If it states that today is the date just still put [Date of letter].
            2. **Recipient Name** (formatted as 'First Name Middle Initial, Last Name', e.g., Juan Joseph D. Cruz if Middle Initial and Lastname is not clear write the fullname base on the conversation). If the recipient's name is missing, insert '[Recipient name]'.
            3. **Recipient Details**, write each line all the personal details such as job position, recipient address or depends on the details given. 
                **Note:** If any of the recipient details are missing, leave the corresponding insert '[Recipient details]'. You must segregate each line the details. 
            4. **Subject of the Letter** (e.g., Demand Letter, Formal Demand Letter, Notice of Breach of Contract, Cease and Desist Order, Notice of Intent to Sue). If the subject is missing, insert '[Type of letter]'.
            5. **Sender's Full Name** (formatted as 'First Name, Middle Initial, Last Name'). If the sender's name is missing put Atty. Jerico D. Salenga.
            6. **Conforme** (If the letter requires a signature of conformity, include the full name of the person who needs to sign; if not, insert 'Conforme not needed').
            7. **Main Content** (All relevant details needed in the body of the letter and explain it briefly).

            Ensure that the array contains exactly these seven elements. For missing details, provide short descriptions inside brackets as placeholders (e.g., '[Date of letter]', '[Recipient name]'). For recipient details, if all lines are missing, return '[Recipient details]'; otherwise, leave specific lines blank if they are missing. 
            Use '<br />' if you need to add a new line Never use '\\n if you will add a new line'.
            You must not include the backticks such as '```json ```' when you return a value." )
        }, {
            "role": "user",
            "content": format!("Conversation history so far:\n{}", conversation_history)
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
        let vector_details: Vec<String> = serde_json::from_str(choice_content)?;
        Ok(vector_details)
    } else {
        Err("No valid response generated.".into()) // Return a boxed error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test] // Mark the test as async using tokio's test framework
    async fn testing_chat_output() {
        let questions: Vec<String> = vec![
            "Hi there! Let's work on creating a letter. Could you provide me details on this letter?".to_string(),
        ];
        let answers: Vec<String> =
            vec!["I want to make a demand letter about Stephen Lewis.".to_string()];

        // Await the asynchronous function call and handle the result
        match values_generator(questions, answers).await {
            Ok(response) => {
                println!("Response: {:?}", response); // Print the successful response
                assert!(!response.is_empty(), "Response should not be empty");
            }
            Err(e) => {
                println!("Test failed with error: {:?}", e); // Handle potential error
            }
        }
    }
}
