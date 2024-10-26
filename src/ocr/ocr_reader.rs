use std::path::Path;
use tesseract::Tesseract;

//sudo apt-get install tesseract-ocr install first beforte running

pub fn ocr_converter(image_path: &Path) -> Result<String, String> {
    // Create a new Tesseract instance
    let mut tess = Tesseract::new(None, Some("eng")).map_err(|_| "Failed to create Tesseract instance".to_string())?;

    // Convert Path to &str and set the image for Tesseract to process
    if let Some(image_str) = image_path.to_str() {
        tess = tess.set_image(image_str).map_err(|_| "Failed to set image".to_string())?;

        // Perform OCR and get the recognized text
        let text = tess.get_text().map_err(|_| "Failed to get text".to_string())?;
        
        Ok(text)
    } else {
        Err("Failed to convert image path to string.".to_string())
    }
}
// #[cfg(test)]
// mod tests{
//     use super::*;
//     #[test]
//     fn test_ocr() {
//         let path_image:&Path=Path::new("src/ocr/sample_file.pdf");
//         ocr_converter(path_image);
//     }
// }