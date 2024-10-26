// use reqwest::multipart;
// use serde_json::Value;
// use tokio::fs::File as TokioFile;
// use tokio_util::io::{ReaderStream};

// pub async fn transcribe_audio(api_key: &str, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let file = TokioFile::open(file_path).await?;
//     let file_stream = ReaderStream::new(file);

//     let form = multipart::Form::new()
//         .text("model", "whisper-1")
//         .part(
//             "file",
//             multipart::Part::stream(reqwest::Body::wrap_stream(file_stream))
//                 .file_name("audio.mp3")
//                 .mime_str("audio/mpeg")?,
//         );

//     let client = reqwest::Client::new();
//     let response = client
//         .post("https://api.openai.com/v1/audio/transcriptions")
//         .bearer_auth(api_key)
//         .multipart(form)
//         .send()
//         .await?;

//     let response_text = response.text().await?;
//     let json: Value = serde_json::from_str(&response_text)?;

//     if let Some(text) = json["text"].as_str() {
//         Ok(text.to_string())
//     } else {
//         Err("Failed to transcribe audio".into())
//     }
// }

use reqwest::multipart;
use serde_json::Value;
use tokio::fs::File as TokioFile;
use tokio_util::io::ReaderStream;

pub async fn transcribe_audio(
    api_key: &str,
    file_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let file = TokioFile::open(file_path).await?;
    let file_stream = ReaderStream::new(file);
    let prompt = "Transcribe and format it into a pleading"; // with custom Whisper prompt
    let form = multipart::Form::new()
        .text("model", "whisper-1")
        .text("prompt", prompt) // with custom Whisper prompt
        .part(
            "file",
            multipart::Part::stream(reqwest::Body::wrap_stream(file_stream))
                .file_name("audio.mp3")
                .mime_str("audio/mpeg")?,
        );

    // Send the transcription request
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .bearer_auth(api_key)
        .multipart(form)
        .send()
        .await?;

    let response_text = response.text().await?;
    let json: Value = serde_json::from_str(&response_text)?;

    // Extract the transcription text
    if let Some(text) = json["text"].as_str() {
        let corrected_text = correct_transcription(api_key, text).await?;
        Ok(corrected_text)
    } else {
        Err("Failed to transcribe audio".into())
    }
}

async fn correct_transcription(
    api_key: &str,
    transcription: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let system_prompt = "Turn it into a pleading format"; // for adding prompts using gpt 4o

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&serde_json::json!({
            "model": "gpt-4o",
            "messages": [
                {"role": "system", "content": system_prompt}, //for adding prompts using gpt 4o
                {"role": "user", "content": transcription}
            ],
            "temperature": 0.0
        }))
        .send()
        .await?;

    let response_text = response.text().await?;
    let json: Value = serde_json::from_str(&response_text)?;

    // Extract and return the corrected text
    if let Some(corrected) = json["choices"][0]["message"]["content"].as_str() {
        Ok(corrected.to_string())
    } else {
        Err("Failed to correct transcription".into())
    }
}
