use crate::models::auto_drafter_object::FormDetails;
use dotenv::dotenv;
use serde_json::json;
use std::env;
use std::error::Error;

pub async  fn facts_generator (form:&FormDetails)-> Result<String, Box<dyn Error>> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|e| Box::<dyn Error>::from(format!("Missing OPENAI_API_KEY: {}", e)))?;
    // Create the content to send to OpenAI for further completion
    let request_body = json!({
        "model": "gpt-4",
        "messages": [{
            "role": "system",
            "content": "Generate a detailed facts section for the following case, ensuring that the information strongly supports the client's claims. Also, specify the type of pleading to be generated."
        }, {
            "role": "user",
            "content": format!(
                "Generate a detailed facts section for the following case:\n\
                1. Date of Incident: {}\n\
                2. Opponent: {}\n\
                3. Opponent Details: {}\n\
                4. Legal Concern: {}\n\
                5. Lawyer: {}\n\
                6. Client: {}\n\
                7. Client Details: {}\n\
                8. Amount Involved: {}\n\
                9. Client's Claims: {}\n",
                form.date,
                form.opponent,
                form.opponent_details,
                form.legal_concern,
                form.sender,
                form.client,
                form.client_details,
                form.amount_involved,
                form.concern
            )
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
        Err("No valid response generated.".into())
    }
}
/// {
/// "date": "October 22, 2024",
/// "opponent": "Stephen Lewis",
/// "opponent_details": "San Nicolas, Pasig",
///"legal_concern": "Workplace Harassment",
///"sender": "Atty. Jerico D. Salenga",
///"client": "Nica Rebadmoia",
///"client_details": "Mandaluyong",
///"amount_involve":"20000",
///"concern": "Stephen Lewis has continuously engaged in abusive behavior towards me at the workplace over the past few months. His actions include making demeaning and offensive remarks about my appearance, such as commenting on my body inappropriately in front of other employees. He has also sent me unsolicited messages outside of work hours that are unprofessional and harassing in nature. Furthermore, Stephen has made several unwanted advances, which I have explicitly rejected, but he persists in making me uncomfortable. His behavior escalated when I reported the incidents to management, as he retaliated by spreading false rumors about my work performance and character. These actions have created a hostile work environment, making it difficult for me to perform my duties and causing me emotional distress."
///}


#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn testing_facts_generator() {
        let responses: FormDetails = FormDetails {
            date: "October 22, 2024".to_string(),
            opponent: "Stephen Lewis".to_string(),
            opponent_details: "San Nicolas, Pasig".to_string(),
            legal_concern: "Workplace Harassment".to_string(),
            sender: "Atty. Jerico D. Salenga".to_string(),
            client: "Jacquie Salilid".to_string(),
            client_details: "Mandaluyong".to_string(),
            amount_involved: "20000".to_string(),
            concern: "Stephen Lewis has continuously engaged in abusive behavior towards me at the workplace over the past few months. His actions include making demeaning and offensive remarks about my appearance, such as commenting on my body inappropriately in front of other employees. He has also sent me unsolicited messages outside of work hours that are unprofessional and harassing in nature. Furthermore, Stephen has made several unwanted advances, which I have explicitly rejected, but he persists in making me uncomfortable. His behavior escalated when I reported the incidents to management, as he retaliated by spreading false rumors about my work performance and character. These actions have created a hostile work environment, making it difficult for me to perform my duties and causing me emotional distress.".to_string(),
        };
        // Await the asynchronous function call and handle the result
        match facts_generator(&responses).await {
            Ok(response) => {
                println!("{}", response)
            }
            Err(e) => {
                panic!("Test failed with error: {:?}", e); // Handle potential error
            }
        }
    }
}
