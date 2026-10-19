use std::{
    path::{Path, PathBuf},
    sync::{Arc, mpsc},
    thread,
};

use tantivy::{IndexWriter, Term, doc};

use crate::{pdfium::bindings::BindingsWrapper, server::state::AppState};

use super::{ChangeTracker, TextSearchIndex, path_facet};

pub enum TextIndexJob {
    IndexPdf {
        doc_id: String,
        title: String,
        cover_name: String,
        file_path: String,
        last_modified: u64,
    },
    RemoveDoc {
        doc_id: String,
    },
    Flush,
    #[allow(dead_code)]
    Shutdown,
}

pub type TextIndexSender = mpsc::Sender<TextIndexJob>;

fn index_pdf(
    doc_id: &str,
    title: &str,
    cover_name: &str,
    file_path: &str,
    last_modified: u64,
    text_index: &TextSearchIndex,
    writer: &mut IndexWriter,
    tracker: &mut ChangeTracker,
    pdfium: &Arc<BindingsWrapper>,
) {
    let term = Term::from_field_text(text_index.doc_id_field, doc_id);
    writer.delete_term(term);

    let is_epub = file_path.to_ascii_lowercase().ends_with(".epub");
    let extraction_result = if is_epub {
        extract_epub_text(file_path)
    } else {
        extract_pdf_text(pdfium, file_path)
    };

    match extraction_result {
        Ok(pages) => {
            let page_count = pages.len();
            let mut indexed_pages = 0;
            let facet = path_facet(file_path);

            for (page_num, page) in pages.into_iter().enumerate() {
                if !page.content.trim().is_empty() {
                    writer
                        .add_document(doc!(
                            text_index.doc_id_field => doc_id.to_string(),
                            text_index.page_num_field => (page_num + 1) as u64,
                            text_index.title_field => title.to_string(),
                            text_index.content_field => page.content,
                            text_index.headings_field => page.headings,
                            text_index.bookmarks_field => page.bookmarks,
                            text_index.cover_name_field => cover_name.to_string(),
                            text_index.path_field => facet.clone(),
                        ))
                        .ok();

                    indexed_pages += 1;
                }
            }

            tracker.mark_indexed(doc_id.to_string(), last_modified);
            println!(
                "Text indexed: {} ({}/{} pages)",
                title, indexed_pages, page_count
            );
        }
        Err(e) => {
            eprintln!("Text extraction failed for '{}': {}", file_path, e);
        }
    }
}

pub fn start_text_indexer(
    index_dir: PathBuf,
    config_dir: PathBuf,
    pdfium: Arc<BindingsWrapper>,
    state: AppState,
) -> TextIndexSender {
    let (tx, rx) = mpsc::channel::<TextIndexJob>();

    thread::spawn(move || {
        let text_index = match TextSearchIndex::open_or_create(&index_dir) {
            Ok(idx) => idx,
            Err(e) => {
                eprintln!("Text indexer failed to open index: {}", e);
                return;
            }
        };

        let mut writer = match text_index.index.writer(50_000_000) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("Text indexer failed to create writer: {}", e);
                return;
            }
        };

        let mut tracker = ChangeTracker::load(&config_dir);

        for job in rx {
            match job {
                TextIndexJob::IndexPdf {
                    doc_id,
                    title,
                    cover_name,
                    file_path,
                    last_modified,
                } => {
                    index_pdf(
                        &doc_id,
                        &title,
                        &cover_name,
                        &file_path,
                        last_modified,
                        &text_index,
                        &mut writer,
                        &mut tracker,
                        &pdfium,
                    );

                    if let Err(e) = writer.commit() {
                        eprintln!("Text index commit failed: {}", e);
                    }

                    if let Err(e) = tracker.save() {
                        eprintln!("Failed to save change tracker: {}", e);
                    }

                    let _ = state.increment_text_index_processed();
                }

                TextIndexJob::RemoveDoc { doc_id } => {
                    let term = Term::from_field_text(text_index.doc_id_field, &doc_id);
                    writer.delete_term(term);
                    tracker.remove(&doc_id);
                }

                TextIndexJob::Flush => {
                    let _ = state.set_text_indexer_status(false);
                }

                TextIndexJob::Shutdown => {
                    let _ = writer.commit();
                    let _ = tracker.save();
                    break;
                }
            }
        }

        println!("Text indexer thread exiting");
    });

    tx
}

#[derive(Debug)]
pub struct PageContent {
    pub content: String,
    pub headings: String,
    pub bookmarks: String,
}

fn extract_pdf_text(
    pdfium: &Arc<BindingsWrapper>,
    file_path: &str,
) -> Result<Vec<PageContent>, String> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    pdfium.with_pdfium(|p| -> Result<Vec<PageContent>, String> {
        let document = p
            .load_pdf_from_file(path, None)
            .map_err(|e| format!("Failed to load PDF '{}': {:?}", file_path, e))?;

        let bookmarks_by_page = collect_bookmarks_by_page(&document);

        let mut pages = Vec::new();

        for (page_idx, page) in document.pages().iter().enumerate() {
            let text = page
                .text()
                .map_err(|e| format!("Failed to get text from page: {:?}", e))?
                .all();

            let headings = detect_headings(&text);
            let bookmarks = bookmarks_by_page
                .get(&(page_idx as i32))
                .map(|titles| titles.join("\n"))
                .unwrap_or_default();

            pages.push(PageContent {
                content: text,
                headings,
                bookmarks,
            });
        }

        Ok(pages)
    })
}

/// Walks the PDF outline and returns a map of page index to bookmark titles pointing at that page.
fn collect_bookmarks_by_page(
    document: &pdfium_render::prelude::PdfDocument,
) -> std::collections::HashMap<i32, Vec<String>> {
    let mut map: std::collections::HashMap<i32, Vec<String>> = std::collections::HashMap::new();

    for bookmark in document.bookmarks().iter() {
        let title = match bookmark.title() {
            Some(t) if !t.trim().is_empty() => t,
            _ => continue,
        };

        let page_index = match bookmark.destination().and_then(|d| d.page_index().ok()) {
            Some(idx) => idx as i32,
            None => continue,
        };

        map.entry(page_index).or_default().push(title);
    }

    map
}

/// Detects heading-like lines in extracted PDF text and returns them joined by newlines.
/// Heuristic: short, mostly-uppercase lines.
fn detect_headings(text: &str) -> String {
    text.lines()
        .filter(|line| is_heading_like(line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn is_heading_like(line: &str) -> bool {
    let trimmed = line.trim();

    if trimmed.len() < 5 || trimmed.len() > 60 {
        return false;
    }

    let alphabetic: Vec<char> = trimmed.chars().filter(|c| c.is_alphabetic()).collect();

    if alphabetic.len() < 3 {
        return false;
    }

    let uppercase_count = alphabetic.iter().filter(|c| c.is_uppercase()).count();
    let uppercase_ratio = uppercase_count as f32 / alphabetic.len() as f32;

    uppercase_ratio >= 0.7
}

pub fn extract_epub_text(file_path: &str) -> Result<Vec<PageContent>, String> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let mut doc = epub::doc::EpubDoc::new(path)
        .map_err(|e| format!("Failed to load EPUB '{}': {:?}", file_path, e))?;

    let num_chapters = doc.get_num_chapters();
    let bookmarks_by_chapter = collect_epub_bookmarks_by_chapter(&doc);

    let mut pages = Vec::with_capacity(num_chapters);

    for chapter_idx in 0..num_chapters {
        doc.set_current_page(chapter_idx);
        let raw_html = if let Some((content, _)) = doc.get_current_str() {
            content
        } else if let Some(spine_item) = doc.spine.get(chapter_idx) {
            match doc.get_resource_str(&spine_item.idref) {
                Some((content, _)) => content,
                None => String::new(),
            }
        } else {
            String::new()
        };

        let (text, extracted_headings) = html_to_text_and_headings(&raw_html);
        let detected = detect_headings(&text);
        let headings = match (extracted_headings.is_empty(), detected.is_empty()) {
            (false, false) => format!("{}\n{}", extracted_headings, detected),
            (false, true) => extracted_headings,
            (true, false) => detected,
            (true, true) => String::new(),
        };

        let bookmarks = bookmarks_by_chapter
            .get(&(chapter_idx as i32))
            .map(|titles| titles.join("\n"))
            .unwrap_or_default();

        pages.push(PageContent {
            content: text,
            headings,
            bookmarks,
        });
    }

    Ok(pages)
}

fn collect_epub_bookmarks_by_chapter<R: std::io::Read + std::io::Seek>(
    doc: &epub::doc::EpubDoc<R>,
) -> std::collections::HashMap<i32, Vec<String>> {
    let mut map: std::collections::HashMap<i32, Vec<String>> = std::collections::HashMap::new();

    fn process_nav_points<R: std::io::Read + std::io::Seek>(
        nav_points: &[epub::doc::NavPoint],
        doc: &epub::doc::EpubDoc<R>,
        map: &mut std::collections::HashMap<i32, Vec<String>>,
    ) {
        for nav in nav_points {
            let label = nav.label.trim();
            if !label.is_empty() {
                let clean_content_str = nav.content.to_string_lossy();
                let clean_path = if let Some(idx) = clean_content_str.find('#') {
                    PathBuf::from(&clean_content_str[..idx])
                } else {
                    nav.content.clone()
                };

                let chapter_opt = doc.resource_uri_to_chapter(&clean_path).or_else(|| {
                    let file_name = clean_path.file_name()?;
                    for (id, res) in &doc.resources {
                        if res.path.file_name() == Some(file_name) {
                            if let Some(ch) = doc.resource_id_to_chapter(id) {
                                return Some(ch);
                            }
                        }
                    }
                    None
                });

                if let Some(ch) = chapter_opt {
                    map.entry(ch as i32).or_default().push(label.to_string());
                }
            }
            if !nav.children.is_empty() {
                process_nav_points(&nav.children, doc, map);
            }
        }
    }

    process_nav_points(&doc.toc, doc, &mut map);
    map
}

pub fn html_to_text_and_headings(html: &str) -> (String, String) {
    let mut text = String::with_capacity(html.len());
    let mut headings = Vec::new();

    let mut in_tag = false;
    let mut current_tag = String::new();
    let mut in_skip_element = false;
    let mut skip_tag_name = String::new();
    let mut in_heading = false;
    let mut current_heading = String::new();

    for c in html.chars() {
        if c == '<' {
            in_tag = true;
            current_tag.clear();
            continue;
        }

        if in_tag {
            if c == '>' {
                in_tag = false;
                let tag_trimmed = current_tag.trim().to_lowercase();
                let tag_name = tag_trimmed
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim_start_matches('/');

                if in_skip_element {
                    if current_tag.trim().starts_with('/') && tag_name == skip_tag_name {
                        in_skip_element = false;
                        skip_tag_name.clear();
                    }
                } else if !current_tag.trim().starts_with('/') {
                    if tag_name == "script" || tag_name == "style" || tag_name == "head" {
                        in_skip_element = true;
                        skip_tag_name = tag_name.to_string();
                    } else if tag_name == "h1"
                        || tag_name == "h2"
                        || tag_name == "h3"
                        || tag_name == "h4"
                    {
                        in_heading = true;
                        current_heading.clear();
                        text.push('\n');
                    } else if tag_name == "p"
                        || tag_name == "div"
                        || tag_name == "br"
                        || tag_name == "li"
                        || tag_name == "tr"
                    {
                        text.push('\n');
                    }
                } else {
                    if in_heading
                        && (tag_name == "h1"
                            || tag_name == "h2"
                            || tag_name == "h3"
                            || tag_name == "h4")
                    {
                        in_heading = false;
                        let trimmed_h = current_heading.trim();
                        if !trimmed_h.is_empty() {
                            headings.push(trimmed_h.to_string());
                        }
                        current_heading.clear();
                        text.push('\n');
                    } else if tag_name == "p"
                        || tag_name == "div"
                        || tag_name == "li"
                        || tag_name == "tr"
                    {
                        text.push('\n');
                    }
                }
                continue;
            } else {
                current_tag.push(c);
                continue;
            }
        }

        if in_skip_element {
            continue;
        }

        if in_heading {
            current_heading.push(c);
        }
        text.push(c);
    }

    let decoded_text = decode_html_entities(&text);
    let decoded_headings = headings
        .into_iter()
        .map(|h| decode_html_entities(&h))
        .filter(|h| !h.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    let clean_text = decoded_text
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    (clean_text, decoded_headings)
}

fn decode_html_entities(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '&' {
            let mut entity = String::new();
            let mut closed = false;
            while let Some(&next_c) = chars.peek() {
                if next_c == ';' {
                    chars.next();
                    closed = true;
                    break;
                } else if next_c.is_alphanumeric() || next_c == '#' {
                    entity.push(chars.next().unwrap());
                    if entity.len() > 10 {
                        break;
                    }
                } else {
                    break;
                }
            }

            if closed {
                match entity.as_str() {
                    "amp" => result.push('&'),
                    "lt" => result.push('<'),
                    "gt" => result.push('>'),
                    "quot" => result.push('"'),
                    "apos" => result.push('\''),
                    "nbsp" => result.push(' '),
                    _ if entity.starts_with("#x") || entity.starts_with("#X") => {
                        if let Ok(val) = u32::from_str_radix(&entity[2..], 16) {
                            if let Some(decoded_char) = char::from_u32(val) {
                                result.push(decoded_char);
                            } else {
                                result.push('&');
                                result.push_str(&entity);
                                result.push(';');
                            }
                        } else {
                            result.push('&');
                            result.push_str(&entity);
                            result.push(';');
                        }
                    }
                    _ if entity.starts_with('#') => {
                        if let Ok(val) = entity[1..].parse::<u32>() {
                            if let Some(decoded_char) = char::from_u32(val) {
                                result.push(decoded_char);
                            } else {
                                result.push('&');
                                result.push_str(&entity);
                                result.push(';');
                            }
                        } else {
                            result.push('&');
                            result.push_str(&entity);
                            result.push(';');
                        }
                    }
                    _ => {
                        result.push('&');
                        result.push_str(&entity);
                        result.push(';');
                    }
                }
            } else {
                result.push('&');
                result.push_str(&entity);
            }
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdfium::bindings::init_pdfium;

    #[test]
    fn extract_text_from_valid_pdf() {
        let pdfium = init_pdfium();
        let result = extract_pdf_text(&pdfium, "./test/sample.pdf");

        assert!(result.is_ok(), "Failed to extract text: {:?}", result);

        let pages = result.unwrap();
        assert!(!pages.is_empty(), "Expected at least one page");
    }

    #[test]
    fn extract_text_nonexistent_file() {
        let pdfium = init_pdfium();
        let result = extract_pdf_text(&pdfium, "./test/does-not-exist.pdf");

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("File not found"));
    }

    #[test]
    fn extract_text_broken_pdf() {
        let pdfium = init_pdfium();
        let result = extract_pdf_text(&pdfium, "./test/sample-broken.pdf");

        assert!(result.is_err());
    }

    #[test]
    fn heading_detection_all_caps_short_line() {
        assert!(is_heading_like("THE AVOWED"));
        assert!(is_heading_like("KEEPER OF TOMES"));
        assert!(is_heading_like("SAGES AND MASTER SAGES"));
        assert!(is_heading_like("READERS"));
    }

    #[test]
    fn heading_detection_rejects_too_short() {
        assert!(!is_heading_like("DM"));
        assert!(!is_heading_like("PCs"));
        assert!(!is_heading_like(""));
    }

    #[test]
    fn heading_detection_rejects_too_long() {
        let long = "THIS IS A VERY LONG LINE THAT SHOULDN'T REALLY COUNT AS A HEADING BECAUSE IT GOES ON AND ON";
        assert!(!is_heading_like(long));
    }

    #[test]
    fn heading_detection_rejects_lowercase_body_text() {
        assert!(!is_heading_like("The Keeper of Tomes appoints individuals"));
        assert!(!is_heading_like(
            "Roughly three hundred Avowed live in Candle-"
        ));
    }

    #[test]
    fn heading_detection_rejects_mixed_case() {
        assert!(!is_heading_like("Title Case Heading Here"));
    }

    #[test]
    fn heading_detection_extracts_multiple_from_text() {
        let sample = "
No fighting. All arguments must follow the rules.
SAGES AND MASTER SAGES
Candlekeep's resident lore experts are master sages.
THE AVOWED
Roughly three hundred Avowed live in Candlekeep.
KEEPER OF TOMES
The Keeper is the highest-ranking member.
";
        let headings = detect_headings(sample);
        assert!(headings.contains("SAGES AND MASTER SAGES"));
        assert!(headings.contains("THE AVOWED"));
        assert!(headings.contains("KEEPER OF TOMES"));
        assert!(!headings.contains("Roughly three hundred"));
    }

    #[test]
    fn heading_detection_empty_for_no_headings() {
        let sample =
            "Just some normal body text here.\nMore body text on this line.\nNothing all caps.";
        assert_eq!(detect_headings(sample), "");
    }

    #[test]
    fn extract_text_from_valid_epub() {
        let result = extract_epub_text("./test/sample.epub");
        assert!(result.is_ok(), "Failed to extract epub text: {:?}", result);

        let pages = result.unwrap();
        assert_eq!(pages.len(), 2, "Expected 2 chapters");

        // Chapter 1 check
        assert!(
            pages[0].content.contains("Once upon a time"),
            "Chapter 1 content missing expected text"
        );
        assert!(
            pages[0].headings.contains("THE BEGINNING"),
            "Chapter 1 headings missing THE BEGINNING"
        );
        assert!(
            pages[0].bookmarks.contains("Chapter 1: The Beginning"),
            "Chapter 1 bookmarks missing TOC label"
        );

        // Chapter 2 check
        assert!(
            pages[1].content.contains("The journey was long"),
            "Chapter 2 content missing expected text"
        );
        assert!(
            pages[1].headings.contains("THE JOURNEY"),
            "Chapter 2 headings missing THE JOURNEY"
        );
    }

    #[test]
    fn extract_text_nonexistent_epub() {
        let result = extract_epub_text("./test/does-not-exist.epub");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("File not found"));
    }

    #[test]
    fn extract_text_broken_epub() {
        let result = extract_epub_text("./test/sample-broken.epub");
        assert!(result.is_err());
    }

    #[test]
    fn html_to_text_and_headings_parsing() {
        let sample = r#"
            <!DOCTYPE html>
            <html>
            <head><title>Test Title</title><style>body { color: red; }</style></head>
            <body>
                <h1>CHAPTER ONE</h1>
                <p>Hello &amp; welcome to the &quot;library&quot;.</p>
                <h2>SUBSECTION</h2>
                <div>Here is some text with &lt;special&gt; characters &amp; entities.</div>
            </body>
            </html>
        "#;

        let (text, headings) = html_to_text_and_headings(sample);
        assert!(text.contains("CHAPTER ONE"));
        assert!(text.contains("Hello & welcome to the \"library\"."));
        assert!(text.contains("Here is some text with <special> characters & entities."));
        assert!(!text.contains("body { color: red; }"));
        assert!(!text.contains("Test Title"));
        assert!(headings.contains("CHAPTER ONE"));
        assert!(headings.contains("SUBSECTION"));
    }
}
