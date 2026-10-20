use std::{
    fs,
    path::Path,
};

use epub::doc::EpubDoc;

/// Extracts the cover image from an EPUB document and saves it as a JPEG.
/// Follows the same naming and legacy-rename scheme as PDF cover extraction.
pub fn extract_epub_cover(
    cover_path: &String,
    cover_name: &String,
    file_path: &String,
    legacy_cover_name: Option<&String>,
    width: i32,
) -> Result<(), String> {
    let new_cover_file = Path::new(cover_path).join(cover_name);

    if let Some(legacy_name) = legacy_cover_name {
        let legacy_cover_file = Path::new(cover_path).join(legacy_name);

        if legacy_cover_file.exists() && legacy_name != cover_name {
            println!("Legacy cover found, renaming to new cover name");
            fs::rename(&legacy_cover_file, &new_cover_file)
                .map_err(|e| format!("Failed to rename legacy cover: {}", e))?;
            return Ok(());
        }
    }

    if new_cover_file.exists() {
        return Ok(());
    }

    if !Path::new(file_path).exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let mut doc = EpubDoc::new(file_path)
        .map_err(|e| format!("Failed to load EPUB '{}': {:?}", file_path, e))?;

    // 1. Try to get cover from EPUB metadata
    let mut cover_bytes: Option<Vec<u8>> = doc.get_cover().map(|(bytes, _mime)| bytes);

    // 2. If no cover in metadata, search resources for the first image
    if cover_bytes.is_none() {
        let image_ids: Vec<String> = doc
            .resources
            .iter()
            .filter(|(_, res)| res.mime.starts_with("image/"))
            .map(|(id, _)| id.clone())
            .collect();

        for id in image_ids {
            if let Some((bytes, _)) = doc.get_resource(&id) {
                if image::load_from_memory(&bytes).is_ok() {
                    cover_bytes = Some(bytes);
                    break;
                }
            }
        }
    }

    let target_width = width.max(100) as u32;
    let target_height = target_width * 3 / 2; // standard 2:3 book cover aspect ratio

    if let Some(bytes) = cover_bytes {
        let dynamic_image = image::load_from_memory(&bytes)
            .map_err(|e| format!("Failed to decode EPUB cover image: {}", e))?;

        let resized = dynamic_image.resize(
            target_width,
            target_height,
            image::imageops::FilterType::Lanczos3,
        );
        let rgb_image = resized.to_rgb8();
        rgb_image
            .save_with_format(&new_cover_file, image::ImageFormat::Jpeg)
            .map_err(|e| format!("Failed to save cover image to {:?}: {}", new_cover_file, e))?;
    } else {
        // Fallback: generate a clean placeholder card image (3:4 aspect ratio)
        let rgb_image = image::RgbImage::from_pixel(
            target_width,
            target_width * 3 / 2, // standard 2:3 book cover aspect ratio
            image::Rgb([40, 44, 52]),
        );
        rgb_image
            .save_with_format(&new_cover_file, image::ImageFormat::Jpeg)
            .map_err(|e| format!("Failed to save fallback cover image: {}", e))?;
    }

    Ok(())
}

/// Returns the number of chapters (spine items) in the EPUB.
pub fn count_epub_chapters(file_path: &str) -> Result<u32, String> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let doc = EpubDoc::new(path)
        .map_err(|e| format!("Failed to load EPUB '{}': {:?}", file_path, e))?;

    Ok(doc.get_num_chapters() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cleanup_test_file(path: &str) {
        let _ = fs::remove_file(Path::new(path));
    }

    #[test]
    fn epub_extract_cover_success() {
        let output_path = "./test/sample-epub-cover.jpg";
        cleanup_test_file(output_path);

        let result = extract_epub_cover(
            &"./test".to_string(),
            &"sample-epub-cover.jpg".to_string(),
            &"./test/sample.epub".to_string(),
            None,
            900,
        );

        assert!(result.is_ok(), "Failed to extract epub cover: {:?}", result);
        assert!(Path::new(output_path).exists(), "Cover file was not created");
        cleanup_test_file(output_path);
    }

    #[test]
    fn epub_extract_broken_fails_gracefully() {
        let output_path = "./test/sample-broken-cover.jpg";
        cleanup_test_file(output_path);

        let result = extract_epub_cover(
            &"./test".to_string(),
            &"sample-broken-cover.jpg".to_string(),
            &"./test/sample-broken.epub".to_string(),
            None,
            900,
        );

        assert!(result.is_err(), "Expected error for broken epub");
        assert!(!Path::new(output_path).exists(), "Cover file should not exist");
        cleanup_test_file(output_path);
    }

    #[test]
    fn epub_count_chapters_test() {
        let result = count_epub_chapters("./test/sample.epub");
        assert!(result.is_ok(), "Failed to count chapters: {:?}", result);
        assert_eq!(result.unwrap(), 2, "Expected 2 chapters in sample.epub");
    }

    #[test]
    fn epub_count_chapters_nonexistent() {
        let result = count_epub_chapters("./test/does-not-exist.epub");
        assert!(result.is_err());
    }
}
