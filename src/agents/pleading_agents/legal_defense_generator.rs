
use dotenv::dotenv;
use serde_json::json;
use std::env;
use std::error::Error;

pub async fn legality_basis (facts:&String, )-> Result<String, Box<dyn Error>> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|e| Box::<dyn Error>::from(format!("Missing OPENAI_API_KEY: {}", e)))?;
    // Create the content to send to OpenAI for further completion
    let request_body = json!({
        "model": "gpt-4",
        "messages": [{
            "role": "system",
            "content": "Generate the legal arguments and basis for the pleading, including any relevant laws that support the client. Base your explanation on Philippine law and keep it concise."
        }, {
            "role": "user",
            "content": format!(
                "Generate the legal arguments according to the following facts: \n\
                {}",
                facts
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



#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn testing_facts_generator() {
        let responses: String = format!("FACTUAL DETAILS OF THE CASE:

1. Time Frame: The claims pertain to incidents reported over several months leading up to the date of this document, October 22, 2024.

2. Parties Involved: The plaintiff involved in this case is Ms. Jacquie Salilid, a resident of Mandaluyong. The defendant in this case is Mr. Stephen Lewis, who resides at San Nicolas, Pasig. 

3. Relationship between parties: Both Ms. Salilid and Mr. Lewis are colleagues, sharing the same workplace.

4. Description of Offences: The plaintiff allegations against Mr. Lewis involve continuous Workplace Harassment. The details of the harassment include:
   - Demeaning and offensive remarks about her appearance in front of other employees.
   - Unsolicited, unprofessional, and harassing messages outside of work hours.
   - Unwanted personal advances which Ms. Salilid claims she openly rejected.
   - Retaliatory conduct from Mr. Lewis after Ms. Salilid reported these incidents to management, denoted by false rumors about her work performance and character.

5. Impact on the Plaintiff: These repeated harassments and hostile behaviour created an uncomfortable workplace environment for Ms. Salilid, leading to subpar work performance and have caused emotional distress.

6. Monetary Damages: As a result of the aforementioned behaviours and subsequent impacts on her, Ms. Salilid is seeking damages amounting to 20,000.

The Type of Pleading for this case is Complaint for Damages due to Workplace Harassment. The lawyer for the plaintiff, Atty. Jerico D. Salenga, would be preparing this pleading to make a formal charge or claim against Mr. Stephen Lewis on behalf of Ms. Jacquie Salilid.

");
        // Await the asynchronous function call and handle the result
        match legality_basis(&responses).await {
            Ok(response) => {
                println!("{}", response)
            }
            Err(e) => {
                panic!("Test failed with error: {:?}", e); // Handle potential error
            }
        }
    }
} 