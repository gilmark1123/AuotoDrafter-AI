

use crate::agents::pleading_agents::facts_generator::facts_generator;
use crate::agents::pleading_agents::legal_defense_generator::legality_basis;
use crate::agents::pleading_agents::pleading_content::pleading_content_generator;
use crate::models::auto_drafter_object::FormDetails;
use actix_web::{post, web, HttpResponse, Responder};
use std::collections::HashMap;
// Handler function to process form in Pleadings

///
/// 
/// Sample post usage:
/// {
/// "date": "October 24, 2024",
/// "opponent": "Stephen Lewis",
/// "opponent_details": "San Nicolas, Pasig",
///"legal_concern": "Workplace Harassment",
///"sender": "Atty. Jerico D. Salenga",
///"client": "Nica Rebadmoia",
///"client_details": "Mandaluyong",
///"amount_involve":"20000",
///"concern": "Stephen Lewis has continuously engaged in abusive behavior towards me at the workplace over the past few months. His actions include making demeaning and offensive remarks about my appearance, such as commenting on my body inappropriately in front of other employees. He has also sent me unsolicited messages outside of work hours that are unprofessional and harassing in nature. Furthermore, Stephen has made several unwanted advances, which I have explicitly rejected, but he persists in making me uncomfortable. His behavior escalated when I reported the incidents to management, as he retaliated by spreading false rumors about my work performance and character. These actions have created a hostile work environment, making it difficult for me to perform my duties and causing me emotional distress."
///}
///
/// 
#[post("/form_response")]

async fn pleadings_form(data: web::Json<FormDetails>) -> impl Responder {
     // Call the first function, check for errors
     let facts = match facts_generator(&data).await {
        Ok(first_str) => first_str,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    };

    let legal_basis = match legality_basis(&facts).await {
        Ok(legality) => legality,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    };
    
    let pleadings = match pleading_content_generator(&facts, &legal_basis).await {
        Ok(final_str) => final_str,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error:: {}", e)),
    };
    let mut response: HashMap<String, String> = HashMap::new();
            response.insert("response".to_string(), pleadings);

    HttpResponse::Ok().json(response)

}
