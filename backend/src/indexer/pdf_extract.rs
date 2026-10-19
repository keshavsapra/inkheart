use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use pdfium_render::prelude::*;

use crate::pdfium::bindings::BindingsWrapper;

pub fn extract_pdf_cover(
    cover_path: &String,
    cover_name: &String,
    file_path: &String,
    pdfium: Arc<BindingsWrapper>,
    legacy_cover_name: Option<&String>,
    width: i32,
) -> Result<(), String> {
    let new_cover_file = std::path::Path::new(cover_path).join(cover_name);

    if let Some(legacy_name) = legacy_cover_name {
        let legacy_cover_file = std::path::Path::new(cover_path).join(legacy_name);

        if legacy_cover_file.exists() && legacy_name != cover_name {
            println!("Legacy cover found, renaming to new cover name");
            // Rename (move) legacy cover to new cover name
            std::fs::rename(&legacy_cover_file, &new_cover_file)
                .map_err(|e| format!("Failed to rename legacy cover: {}", e))?;
            return Ok(());
        }
    }

    // If no legacy cover or it does not exist, extract as usual
    extract_pdf_page(file_path, &new_cover_file, 0, width, width * 2, pdfium)?;

    Ok(())
}

pub async fn extract_pdf_thumbnail(
    thumbnail_path: &String,
    id: &String,
    page_index: i32,
    file_path: &String,
    pdfium: Arc<BindingsWrapper>,
    width: i32,
) -> Result<String, String> {
    // Check if the thumbnail image already exists in the root dir for covers
    let filename = format!("{id}-{page_index}.jpg");
    let cover_file = std::path::Path::new(thumbnail_path).join(&filename);
    println!("Extracting thumbnail");

    extract_pdf_page(file_path, &cover_file, page_index, width, width * 2, pdfium)?;

    Ok(filename)
}

fn extract_pdf_page(
    file_path: &String,
    output: &PathBuf,
    page_index: i32,
    width: i32,
    height: i32,
    pdfium: Arc<BindingsWrapper>,
) -> Result<(), String> {
    if output.exists() {
        return Ok(());
    }

    println!("Extracting page {} from {}", page_index, file_path);

    // Check file exists and is readable
    if !Path::new(file_path).exists() {
        return Err(format!("File not found: {}", file_path));
    }

    pdfium.with_pdfium(|p| -> Result<(), String> {
        let document = p
            .load_pdf_from_file(Path::new(file_path), None)
            .map_err(|e| format!("Failed to load PDF '{}': {:?}", file_path, e))?;

        let page_count = document.pages().len();
        if page_index >= page_count {
            return Err(format!(
                "Page index {} out of range (PDF has {} pages)",
                page_index, page_count
            ));
        }

        let render_config = PdfRenderConfig::new()
            .set_target_width(width)
            .set_maximum_height(height)
            .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

        let _page = document
            .pages()
            .get(page_index)
            .map_err(|e| format!("Failed to get page {}: {:?}", page_index, e))?;

        let dynamic_image = _page
            .render_with_config(&render_config)
            .map_err(|e| format!("Failed to render page: {:?}", e))?
            .as_image()
            .map_err(|e| format!("Failed to convert render to image: {:?}", e))?;

        let rgb_image = dynamic_image.to_rgb8();
        rgb_image
            .save_with_format(output, image::ImageFormat::Jpeg)
            .map_err(|e| format!("Failed to save image to {:?}: {}", output, e))?;

        Ok(())
    })?;

    Ok(())
}

pub fn count_pdf_pages(file_path: &String, pdfium: Arc<BindingsWrapper>) -> Result<u32, String> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    pdfium.with_pdfium(|p| -> Result<u32, String> {
        let document = p
            .load_pdf_from_file(path, None)
            .map_err(|e| format!("Failed to load PDF '{}': {:?}", file_path, e))?;

        Ok(document.pages().len() as u32)
    })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::pdfium::bindings::init_pdfium;

    use super::*;

    /// Helper to ensure test directory is clean before and after tests
    fn cleanup_test_file(path: &str) {
        let _ = fs::remove_file(Path::new(path));
    }

    #[test]
    fn pdfium_extract() {
        let pdfium = init_pdfium();
        let output_path = "./test/sample.jpg";

        // Cleanup before test
        cleanup_test_file(output_path);

        let result = extract_pdf_cover(
            &"./test".to_string(),
            &"sample.jpg".to_string(),
            &"./test/sample.pdf".to_string(),
            pdfium,
            None,
            900,
        );

        assert!(result.is_ok(), "Failed to extract pdf cover! {:?}", result);
        assert!(
            Path::new(output_path).exists(),
            "Cover file was not created"
        );

        // Cleanup after test
        cleanup_test_file(output_path);
    }

    #[test]
    fn pdfium_extract_broken_pdf() {
        let pdfium = init_pdfium();
        let output_path = "./test/sample-broken.jpg";

        // Cleanup before test
        cleanup_test_file(output_path);

        let result = extract_pdf_cover(
            &"./test".to_string(),
            &"sample-broken.jpg".to_string(),
            &"./test/sample-broken.pdf".to_string(),
            pdfium,
            None,
            900,
        );

        // Should fail gracefully, not panic
        assert!(
            result.is_err(),
            "Expected error when processing broken PDF, got Ok"
        );

        // Verify error message is informative
        let error_msg = result.unwrap_err();
        assert!(!error_msg.is_empty(), "Error message should not be empty");
        println!("Broken PDF error (expected): {}", error_msg);

        // Cover should NOT be created
        assert!(
            !Path::new(output_path).exists(),
            "Cover file should not exist for broken PDF"
        );

        // Cleanup (shouldn't be necessary, but just in case)
        cleanup_test_file(output_path);
    }

    #[test]
    fn legacy_cover_migration() {
        let pdfium = init_pdfium();

        let cover_path = "./test";
        let legacy_cover = "legacy-migration-old.jpg";
        let new_cover = "legacy-migration-new.jpg";

        let legacy_cover_path = Path::new(cover_path).join(legacy_cover);
        let new_cover_path = Path::new(cover_path).join(new_cover);

        // Ensure clean state before test
        cleanup_test_file(legacy_cover_path.to_str().unwrap());
        cleanup_test_file(new_cover_path.to_str().unwrap());

        // Create dummy legacy cover file
        fs::write(&legacy_cover_path, b"dummy legacy content").unwrap();

        // Call extract_pdf_cover with legacy_cover_name set
        let result = extract_pdf_cover(
            &cover_path.to_string(),
            &new_cover.to_string(),
            &format!("{}/sample.pdf", cover_path),
            pdfium,
            Some(&legacy_cover.to_string()),
            900,
        );
        assert!(result.is_ok(), "Legacy migration failed: {:?}", result);

        // Legacy file should be gone (renamed)
        assert!(!legacy_cover_path.exists(), "Legacy cover was not renamed");

        // New cover file should exist
        assert!(
            new_cover_path.exists(),
            "New cover file missing after migration"
        );

        // Cleanup
        cleanup_test_file(new_cover_path.to_str().unwrap());
    }

    #[test]
    fn scan_continues_after_broken_pdf() {
        // Simulate the scan loop behavior with mixed valid/broken PDFs
        let pdfium = init_pdfium();

        let test_files = vec![
            ("./test/sample.pdf", "./test/test-valid-1.jpg", true),
            ("./test/sample-broken.pdf", "./test/test-broken.jpg", false),
            ("./test/sample.pdf", "./test/test-valid-2.jpg", true),
        ];

        // Cleanup
        for (_, output, _) in &test_files {
            cleanup_test_file(output);
        }

        let mut successful = 0;
        let mut failed = 0;

        // Simulate scan loop
        for (input, output, should_succeed) in &test_files {
            let filename = Path::new(output).file_name().unwrap().to_str().unwrap();

            let result = extract_pdf_cover(
                &"./test".to_string(),
                &filename.to_string(),
                &input.to_string(),
                pdfium.clone(),
                None,
                900,
            );

            match result {
                Ok(_) => {
                    successful += 1;
                    assert!(should_succeed, "Unexpected success for {}", input);
                }
                Err(e) => {
                    failed += 1;
                    assert!(!should_succeed, "Unexpected failure for {}: {}", input, e);
                    eprintln!("Expected failure for {}: {}", input, e);
                }
            }
        }

        assert_eq!(successful, 2, "Should have 2 successful extractions");
        assert_eq!(failed, 1, "Should have 1 failed extraction");

        // Verify successful files were created
        assert!(Path::new("./test/test-valid-1.jpg").exists());
        assert!(Path::new("./test/test-valid-2.jpg").exists());
        assert!(!Path::new("./test/test-broken.jpg").exists());

        // Cleanup
        for (_, output, _) in &test_files {
            cleanup_test_file(output);
        }
    }

    #[test]
    fn extract_handles_nonexistent_file() {
        let pdfium = init_pdfium();

        let result = extract_pdf_cover(
            &"./test".to_string(),
            &"nonexistent.jpg".to_string(),
            &"./test/this-file-does-not-exist.pdf".to_string(),
            pdfium,
            None,
            900,
        );

        assert!(result.is_err(), "Should fail for nonexistent file");

        let error_msg = result.unwrap_err();
        // Error should mention the file issue (depends on your error handling)
        println!("Nonexistent file error: {}", error_msg);
    }
}
