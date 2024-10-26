use crate::agents::letter_agents::chatbot_letter::chat_generate_with_data;
use crate::agents::letter_agents::content_generators::form_generator;
use crate::agents::letter_agents::coversation_converter::values_generator;
use crate::audio_processing::process_audio::transcribe_audio;
use crate::models::auto_drafter_object::{FormDetails, QuestionAnswer};
use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use dotenv::dotenv;
use futures::stream::StreamExt;
use serde_json::json;
use std::collections::HashMap;
use std::env;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

// Handler function to process the incoming vectors
#[post("/chatbot_letter")]
async fn process_vectors(data: web::Json<QuestionAnswer>) -> impl Responder {
    // Extract the questions and answers from the incoming request
    let questions = data.questions.clone();
    let answers = data.answers.clone();

    // Call the chat_generate function with the provided questions and answers
    match chat_generate_with_data(questions, answers).await {
        Ok(response_content) => {
            // Return the generated content as a response
            HttpResponse::Ok().body(response_content)
        }
        Err(e) => {
            // If there was an error, return a 500 Internal Server Error
            let error_response = json!({
                "Error": e.to_string()
            });

            HttpResponse::InternalServerError().json(error_response)
        }
    }
}

// Handler function to process form
#[post("/form_response")]

async fn process_form(data: web::Json<FormDetails>) -> impl Responder {
    // Call the chat_generate function with the provided questions and answers
    match form_generator(&data).await {
        Ok(response_content) => {
            // Return the generated content as a response
            let mut response: HashMap<String, String> = HashMap::new();
            response.insert("response".to_string(), response_content);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            // If there was an error, return a 500 Internal Server Error
            let error_response = json!({
                "Error": e.to_string()
            });

            HttpResponse::InternalServerError().json(error_response)
        }
    }
}

// Handler function to process form
#[post("/chatbot_letter_generate")]
async fn question_answer_output(data: web::Json<QuestionAnswer>) -> impl Responder {
    // Extract the questions and answers from the incoming request
    let questions = data.questions.clone();
    let answers = data.answers.clone();

    match values_generator(questions, answers).await {
        Ok(response_content) => {
            // Return the generated content as a response

            HttpResponse::Ok().json(response_content)
        }
        Err(e) => {
            // If there was an error, return a 500 Internal Server Error
            let error_response = json!({
                "Error": e.to_string()
            });

            HttpResponse::InternalServerError().json(error_response)
        }
    }
}

#[post("/transcribe_audio")]
async fn transcribe_audio_handler(mut payload: Multipart) -> impl Responder {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    // Process the uploaded files
    while let Some(field) = payload.next().await {
        match field {
            Ok(mut field) => {
                // Get the filename from content_disposition
                let content_disposition = field.content_disposition();
                let file_name = content_disposition.get_filename().unwrap_or("audio.mp3");
                let file_path = format!("/tmp/{}", file_name);

                // Create a temporary file to write the uploaded content
                let mut f = match File::create(&file_path).await {
                    Ok(file) => file,
                    Err(e) => {
                        let error_response = json!({ "Error": e.to_string() });
                        return HttpResponse::InternalServerError().json(error_response);
                    }
                };

                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(data) => {
                            if let Err(e) = f.write_all(&data).await {
                                let error_response = json!({ "Error": e.to_string() });
                                return HttpResponse::InternalServerError().json(error_response);
                            }
                        }
                        Err(e) => {
                            let error_response = json!({ "Error": e.to_string() });
                            return HttpResponse::InternalServerError().json(error_response);
                        }
                    }
                }

                match transcribe_audio(&api_key, &file_path).await {
                    Ok(transcription) => {
                        return HttpResponse::Ok().json(json!({ "transcription": transcription }));
                    }
                    Err(e) => {
                        let error_response = json!({ "Error": e.to_string() });
                        return HttpResponse::InternalServerError().json(error_response);
                    }
                }
            }
            Err(e) => {
                let error_response = json!({ "Error": e.to_string() });
                return HttpResponse::InternalServerError().json(error_response);
            }
        }
    }

    HttpResponse::BadRequest().body("No file uploaded")
}
