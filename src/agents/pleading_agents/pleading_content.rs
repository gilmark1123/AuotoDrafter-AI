use dotenv::dotenv;
use serde_json::json;
use std::env;
use std::error::Error;
use std::fs;

pub async fn pleading_content_generator (facts:&String, legal_basis:&String)-> Result<String, Box<dyn Error>> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|e| Box::<dyn Error>::from(format!("Missing OPENAI_API_KEY: {}", e)))?;
    let html_content = fs::read_to_string("src/agents/pleading_agents/pleading.html")?;
    let content = format!(
        "Your task is to create a pleading. Generate the sections of the pleading that can support the client. Base all the details on the facts and legal basis provided. You must expound upon the details given. Format it in HTML and replicate the alignment in this template: {}",
        html_content
    );
    
    // Create the content to send to OpenAI for further completion
    let request_body = json!({
        "model": "gpt-4",
        "messages": [{
            "role": "system",
            "content": format!("{}
",content )
        }, {
            "role": "user",
            "content": format!("Here are details presented in the case:\n
            Facts:{}\n
            Legal Basis: {}\n

            ",facts, 
            legal_basis,

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
    async fn testing_pleading_content_generator() {
        let legal_basis:String = format!("LEGAL ARGUMENT:

The legal arguments for this case would be fundamentally based on the Anti-Sexual Harassment Act of 1995 (RA 7877), the Civil Code, and the Labor Code of the Philippines. 

1. Violation of the Anti-Sexual Harassment Act of 1995: Based on the allegations mentioned by Ms. Salilid, it appears that Mr. Lewis has committed acts tantamount to sexual harassment as defined in RA 7877. This involves unwanted sexual advances, or offensive remarks about her appearance which have created a hostile environment for the plaintiff. 

2. Breach of the employer's duty under the Labor Code: Under the Labor Code, employers are obligated to maintain a harassment-free workplace. However, it's clear that Mr. Lewis's alleged continuous harassment led to the creation of an uncomfortable workplace environment, thereby breaching this legal requirement. Additionally, the retaliatory conduct displayed by Mr. Lewis after our client reported these incidents to the management is reflective of violation of employee rights under the Labor Code.

3. Civil liabilities under the Civil Code: The unfounded rumors about Ms. Salilid's work performance and character circulated after reporting the incidents can be construed as defamation, a tortious act under Civil Code, that resulted not only in mental anguish but also in besmirched reputation. 

As per the Civil Code, any person who willfully causes loss or injury to another in a manner that is contrary to morals, good customs or public policy shall compensate the latter for the damage. Given the overlapping violations, we are confident in seeking damages amounting to Php 20,000 for the harm suffered by Ms. Salilid both emotionally and in her professional life.

We, therefore, plead before the court to hold Mr. Lewis accountable for these misdemeanors and oblige him to compensate Ms. Salilid for the damages she has endured.");
        let facts:String = format!("FACTUAL DETAILS OF THE CASE:

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
        match pleading_content_generator(&facts, &legal_basis).await {
            Ok(response) => {
                println!("{}", response)
            }
            Err(e) => {
                panic!("Test failed with error: {:?}", e); // Handle potential error
            }
        }
    }
}


