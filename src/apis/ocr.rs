use actix_web::{ HttpResponse, Error};
use actix_multipart::Multipart;
use futures_util::stream::StreamExt as _;
use std::io::Write;
use actix_web::post;
use crate::ocr::ocr_reader::ocr_converter;
use tempfile::NamedTempFile;


#[post("/ocr_picture")]
async fn process_ocr(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // Create a temporary file for storing the uploaded image
    let mut temp_file = NamedTempFile::new()?;
    
    // Iterate over the multipart stream and write the file to the temp file
    while let Some(item) = payload.next().await {
        let mut field = item?;
        // Write chunks of the uploaded image to the temporary file
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            temp_file.write_all(&data)?;
        }
    }

    // Get the path to the temporary file
    let temp_file_path = temp_file.path();

    // Call the OCR reader function
    match ocr_converter(temp_file_path) {
        Ok(text) => Ok(HttpResponse::Ok().body(text)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}