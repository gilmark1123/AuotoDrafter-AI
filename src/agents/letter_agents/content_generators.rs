use crate::models::auto_drafter_object::FormDetails;
use dotenv::dotenv;
use serde_json::json;
use std::env;
use std::error::Error;
use std::fs;
pub async fn form_generator(
    responses: &FormDetails,
) -> Result<String, Box<dyn Error>> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|e| Box::<dyn Error>::from(format!("Missing OPENAI_API_KEY: {}", e)))?;
    let html_content = fs::read_to_string("src/agents/letter_agents/letter.html")?;
    // Create the content to send to OpenAI for further completion
    let request_body = json!({
        "model": "gpt-4",
        "messages": [{
            "role": "system",
            "content": format!("Your task is to generate a formal letter. Every letter you generate should be based on the sender's point of view. The response should follow the same template as this: 
                {}
            The return tags for the template values are as follows: 
                1. **date**: Format as 'Month Day, Year'. If there is no date, use '[Date of letter]'.
                2. **opponent**: (formatted as 'First Name Middle Initial, Last Name', e.g., Juan Joseph D. Cruz). If the opponent's name is unclear, use the full name based on the conversation. If the opponent's name is missing, insert '[Recipient name]'.
                3. **opponent_details**: Provide the recipient’s address or additional information, phrased professionally as if it is intended for the recipient's details. Format each line based on professional letter format; do not add annotations to the line details. Use '<br />' for new lines. Never use '\\n'.
                4. **legal_concern**: (e.g., Demand Letter, Formal Demand Letter, Notice of Breach of Contract, Cease and Desist Order, Notice of Intent to Sue). Rephrase it professionally if it does not sound formal. If the subject is missing, insert '[Legal Concern]'.
                5. **sender**: (formatted as 'First Name Middle Initial, Last Name', e.g., Juan Joseph D. Cruz). If the sender's name is unclear, use the full name based on the conversation. If the sender's name is missing, insert '[Sender name]'.
                6. **client**: If applicable, include the name and agreement, paraphrasing the context into the format 'Juan D. Cruz'. If it is empty, do not generate the tags with IDs for client and client_details.
                7. **client_details**: Provide the client's address or additional information, phrased professionally for the recipient's details. Format each line based on professional letter format; do not include the client's name in this field. Use '<br />' for new lines. If it is empty, do not generate the tags with IDs for client and client_details.
                8. **concern**: Structure the content professionally and expand upon the given content. Support the letter with a clear introduction, body, and conclusion. Always start the letter like: (Example one: 'Dear Mr. Cruz', Example two: 'Dear Ms. Estabas', Example three: 'Dear Dr. Cruz'). Make the content at least 200 words or more. Do not assume any details such as gender and age unless implied. Add two new lines after each paragraph for improved readability. End the content with the line: 'Please be guided accordingly.' Ensure logical flow and clarity throughout. Do not include any closing such as 'Sincerely, Juan Dela Cruz'. Use '<br />' for new lines. Never use '\\n'.", html_content)
        }, {
            "role": "user",
            "content": format!(
                "Create a letter using the following details:\n\
                1. date: {}\n\
                2. opponent: {}\n\
                3. opponent_details: {}\n\
                4. legal_concern: {}\n\
                5. sender: {}\n\
                6. client: {}\n\
                7. client_details: {}\n\
                8. amount_involved: {}\n\
                9. concern: {}\n",
                responses.date,
                responses.opponent,
                responses.opponent_details,
                responses.legal_concern,
                responses.sender,
                responses.client,
                responses.client_details,
                responses.amount_involved,
                responses.concern
            )
        }],
        "max_tokens": 2048
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
        Err("No valid response generated.".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn testing_content_generate() {
        let responses: FormDetails = FormDetails {
            date: "October 10, 2024".to_string(),
            opponent: "Spehen Lewis".to_string(),
            opponent_details: "Lives in Pasig".to_string(),
            legal_concern: "Payment Needed".to_string(),
            sender: "Atty. Jerico D. Salenga".to_string(),
            client: "".to_string(),
            client_details: "".to_string(),
            amount_involved: "100,00".to_string(),
            concern: "he  needs to pay to his bill".to_string(),
        };
        // Await the asynchronous function call and handle the result
        match form_generator(&responses).await {
            Ok(response) => {
                println!("{}", response)
            }
            Err(e) => {
                panic!("Test failed with error: {:?}", e); // Handle potential error
            }
        }
    }
}
