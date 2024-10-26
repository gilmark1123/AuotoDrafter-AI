mod agents;
mod apis;
mod audio_processing;
mod models;
mod ocr;
use actix_cors::Cors;
use actix_web;
use actix_web::{web, App, HttpServer};
use apis::pleadings_request::pleadings_form;
use apis::letter_request::{
    process_form, process_vectors, question_answer_output, transcribe_audio_handler,
};
use apis::ocr::process_ocr;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(
                web::scope("/letter") //scope for the letter
                    .service(process_vectors)
                    .service(process_form)
                    .service(question_answer_output)
                    .service(transcribe_audio_handler),
            )
            .service(
                web::scope("/pleadings") //scope for the letter
                    .service(pleadings_form)
            )
            .service(process_ocr)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
