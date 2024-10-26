use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use std::error::Error;

fn convert_pdf_to_images(pdf_path: &Path, output_dir: &str) -> Result<(), Box<dyn Error>> {
    
    // Check if the PDF file exists
    if !pdf_path.exists() {
        return Err(format!("Error: PDF file not found at path '{}'", pdf_path.display()).into());
    }

    // Check if the output directory exists, and create it if it doesn't
    let output_path = PathBuf::from(output_dir);
    if !output_path.exists() {
        fs::create_dir_all(&output_path)
            .map_err(|err| format!("Error: Failed to create output directory '{}': {}", output_dir, err))?;
    }

    // Construct the output file prefix (e.g., 'output_images/output')
    let output_prefix = output_path.join("output");
    let output_str = output_prefix
        .to_str()
        .ok_or_else(|| format!("Error: Failed to convert output path to string '{}'", output_prefix.display()))?;

    // Execute the `pdftoppm` command to convert PDF to PNG images
    let output = Command::new("pdftoppm")
        .arg(pdf_path)                // Input PDF file
        .arg("-png")                  // Output format as PNG images
        .arg(output_str)              // Output prefix
        .output()?;

    // Check if the command execution was successful
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Error during PDF to image conversion: {}", stderr).into());
    }

    println!("Success: PDF converted to images successfully!");

    Ok(())
}
#[cfg(test)]
mod tests{

    use super::*;
    #[test]
    fn test_converter(){
        let path = Path::new("src/ocr/20240115-COMPLAINT.pdf");
        let value = "src/ocr";
        convert_pdf_to_images(path, value);
    }
}