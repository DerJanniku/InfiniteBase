use std::process::Command;
use std::path::Path;
use anyhow::{anyhow, Result};
use image::imageops::FilterType;

pub struct PreviewService;

impl PreviewService {
    pub async fn generate_preview(file_path: &str, output_path: &str) -> Result<()> {
        let path = Path::new(file_path);
        let ext = path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        match ext.as_str() {
            "jpg" | "jpeg" | "png" | "gif" | "webp" => {
                Self::generate_image_preview(file_path, output_path).await
            }
            "pdf" => {
                Self::generate_pdf_preview(file_path, output_path).await
            }
            "mp4" | "mov" | "avi" | "mkv" => {
                Self::generate_video_preview(file_path, output_path).await
            }
            _ => Err(anyhow!("Unsupported file type for preview: {}", ext)),
        }
    }

    async fn generate_image_preview(file_path: &str, output_path: &str) -> Result<()> {
        let img = image::open(file_path)?;
        let thumbnail = img.resize(400, 400, FilterType::Lanczos3);
        thumbnail.save(output_path)?;
        Ok(())
    }

    async fn generate_pdf_preview(file_path: &str, output_path: &str) -> Result<()> {
        // Use pdftoppm to generate the first page as an image
        // pdftoppm -f 1 -l 1 -png -singlefile <file> <output_prefix>
        let output_prefix = output_path.replace(".png", "");
        let status = Command::new("pdftoppm")
            .args(["-f", "1", "-l", "1", "-png", "-singlefile", file_path, &output_prefix])
            .status()?;

        if !status.success() {
            return Err(anyhow!("pdftoppm failed with status: {}", status));
        }

        // pdftoppm appends .png if not explicitly told otherwise, but we used -singlefile
        // We might need to rename or ensure the path is exact
        Ok(())
    }

    async fn generate_video_preview(file_path: &str, output_path: &str) -> Result<()> {
        // Use ffmpeg to grab a frame from the middle
        // ffmpeg -i <input> -ss 00:00:01 -vframes 1 <output>
        let status = Command::new("ffmpeg")
            .args(["-i", file_path, "-ss", "00:00:01", "-vframes", "1", "-y", output_path])
            .status()?;

        if !status.success() {
            return Err(anyhow!("ffmpeg failed with status: {}", status));
        }

        Ok(())
    }
}
