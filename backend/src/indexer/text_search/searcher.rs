use std::collections::HashMap;

use serde::Serialize;
use tantivy::{
    TantivyDocument, Term,
    collector::TopDocs,
    query::{BooleanQuery, Occur, Query, QueryParser, TermQuery},
    schema::{IndexRecordOption, document::Value},
};

use super::{TextSearchIndex, path_facet};

#[derive(Clone, Serialize, Debug)]
pub struct FullTextResult {
    pub doc_id: String,
    pub page_num: u64,
    pub title: String,
    pub cover_name: String,
    pub match_count: usize,
}

impl TextSearchIndex {
    pub fn search(
        &self,
        query_str: &str,
        limit: usize,
        scope: Option<&str>,
    ) -> Result<Vec<FullTextResult>, String> {
        if query_str.trim().is_empty() {
            return Ok(Vec::new());
        }

        let searcher = self.reader.searcher();

        let mut query_parser = QueryParser::for_index(
            &self.index,
            vec![
                self.content_field,
                self.headings_field,
                self.bookmarks_field,
                self.title_field,
            ],
        );

        // Bookmaks gets the highest boost since they are explicitly created by users to point to important sections.
        // Headings are detected from body text using heuristics.
        query_parser.set_field_boost(self.bookmarks_field, 10.0);
        query_parser.set_field_boost(self.headings_field, 5.0);

        let parsed_query = query_parser
            .parse_query(query_str)
            .map_err(|e| format!("Failed to parse query: {}", e))?;

        let query: Box<dyn Query> = match scope {
            Some(dir) => {
                let facet_term = Term::from_facet(self.path_field, &path_facet(dir));
                let facet_query = TermQuery::new(facet_term, IndexRecordOption::Basic);

                Box::new(BooleanQuery::new(vec![
                    (Occur::Must, parsed_query),
                    (Occur::Must, Box::new(facet_query)),
                ]))
            }
            None => parsed_query,
        };

        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(limit * 5).order_by_score())
            .map_err(|e| format!("Search failed: {}", e))?;

        let mut best_per_doc: HashMap<String, (f32, usize, FullTextResult)> = HashMap::new();

        for (score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher
                .doc(doc_address)
                .map_err(|e| format!("Failed to retrieve document: {}", e))?;

            let doc_id = doc
                .get_first(self.doc_id_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let page_num = doc
                .get_first(self.page_num_field)
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

            let title = doc
                .get_first(self.title_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let cover_name = doc
                .get_first(self.cover_name_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let result = FullTextResult {
                doc_id: doc_id.clone(),
                page_num,
                title,
                cover_name,
                match_count: 1,
            };

            best_per_doc
                .entry(doc_id)
                .and_modify(|(best_score, count, best_result)| {
                    *count += 1;

                    if score > *best_score {
                        *best_score = score;
                        *best_result = result.clone();
                    }
                })
                .or_insert((score, 1, result));
        }

        let mut results: Vec<(f32, FullTextResult)> = best_per_doc
            .into_values()
            .map(|(score, count, mut result)| {
                result.match_count = count;
                (score, result)
            })
            .collect();

        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);

        Ok(results.into_iter().map(|(_, r)| r).collect())
    }
}

#[cfg(test)]
mod tests {
    use tantivy::{IndexWriter, Term, doc};
    use tempfile::TempDir;

    use super::*;
    use crate::indexer::text_search::TextSearchIndex;

    fn create_test_index() -> (TempDir, TextSearchIndex) {
        let tmp = TempDir::new().unwrap();
        let index = TextSearchIndex::open_or_create(tmp.path()).unwrap();
        (tmp, index)
    }

    struct TestPage<'a> {
        doc_id: &'a str,
        page_num: u64,
        title: &'a str,
        content: &'a str,
        headings: &'a str,
        bookmarks: &'a str,
        cover_name: &'a str,
        file_path: Option<&'a str>,
    }

    impl<'a> Default for TestPage<'a> {
        fn default() -> Self {
            TestPage {
                doc_id: "",
                page_num: 1,
                title: "",
                content: "",
                headings: "",
                bookmarks: "",
                cover_name: "",
                file_path: None,
            }
        }
    }

    fn add_page(index: &TextSearchIndex, writer: &mut IndexWriter, page: TestPage) {
        let default_path = format!("/media/{}.pdf", page.doc_id);
        let file_path = page.file_path.unwrap_or(&default_path);

        writer
            .add_document(doc!(
                index.doc_id_field => page.doc_id.to_string(),
                index.page_num_field => page.page_num,
                index.title_field => page.title.to_string(),
                index.content_field => page.content.to_string(),
                index.headings_field => page.headings.to_string(),
                index.bookmarks_field => page.bookmarks.to_string(),
                index.cover_name_field => page.cover_name.to_string(),
                index.path_field => path_facet(file_path),
            ))
            .unwrap();
    }

    #[test]
    fn search_finds_indexed_document() {
        let (_tmp, index) = create_test_index();
        let mut writer = index.index.writer(15_000_000).unwrap();

        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "doc1",
                page_num: 3,
                title: "My Book",
                content: "dragons and goblins attack the village",
                cover_name: "cover.jpg",
                ..Default::default()
            },
        );
        writer.commit().unwrap();
        index.reader.reload().unwrap();

        let results = index.search("dragons", 10, None).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].doc_id, "doc1");
        assert_eq!(results[0].page_num, 3);
        assert_eq!(results[0].title, "My Book");
        assert_eq!(results[0].cover_name, "cover.jpg");
        assert_eq!(results[0].match_count, 1);
    }

    #[test]
    fn search_nonexistent_term_returns_empty() {
        let (_tmp, index) = create_test_index();
        let mut writer = index.index.writer(15_000_000).unwrap();

        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "doc1",
                content: "some content here",
                ..Default::default()
            },
        );
        writer.commit().unwrap();
        index.reader.reload().unwrap();

        let results = index.search("nonexistent", 10, None).unwrap();

        assert!(results.is_empty());
    }

    #[test]
    fn search_empty_query_returns_empty() {
        let (_tmp, index) = create_test_index();
        let mut writer = index.index.writer(15_000_000).unwrap();

        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "doc1",
                content: "some content",
                ..Default::default()
            },
        );
        writer.commit().unwrap();
        index.reader.reload().unwrap();

        assert!(index.search("", 10, None).unwrap().is_empty());
        assert!(index.search("   ", 10, None).unwrap().is_empty());
    }

    #[test]
    fn search_deduplicates_by_doc_id() {
        let (_tmp, index) = create_test_index();
        let mut writer = index.index.writer(15_000_000).unwrap();

        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "doc1",
                page_num: 1,
                content: "dragons on page one",
                ..Default::default()
            },
        );
        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "doc1",
                page_num: 2,
                content: "more dragons on page two",
                ..Default::default()
            },
        );
        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "doc1",
                page_num: 3,
                content: "even more dragons here",
                ..Default::default()
            },
        );
        writer.commit().unwrap();
        index.reader.reload().unwrap();

        let results = index.search("dragons", 10, None).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].doc_id, "doc1");
        assert_eq!(results[0].match_count, 3);
    }

    #[test]
    fn search_after_delete_returns_empty() {
        let (_tmp, index) = create_test_index();
        let mut writer = index.index.writer(15_000_000).unwrap();

        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "doc1",
                content: "unique findable content",
                ..Default::default()
            },
        );
        writer.commit().unwrap();
        index.reader.reload().unwrap();

        assert_eq!(index.search("findable", 10, None).unwrap().len(), 1);

        let term = Term::from_field_text(index.doc_id_field, "doc1");
        writer.delete_term(term);
        writer.commit().unwrap();
        index.reader.reload().unwrap();

        assert!(index.search("findable", 10, None).unwrap().is_empty());
    }

    #[test]
    fn search_boosts_heading_matches_above_body_matches() {
        let (_tmp, index) = create_test_index();
        let mut writer = index.index.writer(15_000_000).unwrap();

        // Doc 1: term appears in body multiple times but not in headings
        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "doc_body",
                content: "the avowed are mentioned. the avowed appear here too. the avowed once more.",
                headings: "SOMETHING ELSE",
                ..Default::default()
            },
        );

        // Doc 2: term appears in a heading and once in body
        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "doc_heading",
                content: "the section describes the avowed.",
                headings: "THE AVOWED",
                ..Default::default()
            },
        );

        writer.commit().unwrap();
        index.reader.reload().unwrap();

        let results = index.search("the avowed", 10, None).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(
            results[0].doc_id, "doc_heading",
            "Document with term in heading should rank first"
        );
        assert_eq!(results[1].doc_id, "doc_body");
    }

    #[test]
    fn search_boosts_bookmark_matches_above_heading_matches() {
        let (_tmp, index) = create_test_index();
        let mut writer = index.index.writer(15_000_000).unwrap();

        // Doc with term only in body
        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "doc_body",
                content: "candlekeep is mentioned in body text here.",
                ..Default::default()
            },
        );

        // Doc with term in heading
        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "doc_heading",
                content: "the section talks about it.",
                headings: "CANDLEKEEP",
                ..Default::default()
            },
        );

        // Doc with term in bookmark
        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "doc_bookmark",
                content: "the section talks about it.",
                bookmarks: "Candlekeep",
                ..Default::default()
            },
        );

        writer.commit().unwrap();
        index.reader.reload().unwrap();

        let results = index.search("candlekeep", 10, None).unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(
            results[0].doc_id, "doc_bookmark",
            "Bookmark match should rank first"
        );
        assert_eq!(
            results[1].doc_id, "doc_heading",
            "Heading match should rank second"
        );
        assert_eq!(
            results[2].doc_id, "doc_body",
            "Body-only match should rank last"
        );
    }

    #[test]
    fn scoped_search_returns_only_in_scope_docs() {
        let (_tmp, index) = create_test_index();
        let mut writer = index.index.writer(15_000_000).unwrap();

        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "in_scope",
                content: "dragons roam the hills",
                file_path: Some("/media/fantasy/in_scope.pdf"),
                ..Default::default()
            },
        );
        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "out_of_scope",
                content: "dragons roam the hills",
                file_path: Some("/media/history/out_of_scope.pdf"),
                ..Default::default()
            },
        );
        writer.commit().unwrap();
        index.reader.reload().unwrap();

        let results = index
            .search("dragons", 10, Some("/media/fantasy"))
            .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].doc_id, "in_scope");
    }

    #[test]
    fn scoped_search_excludes_sibling_folder() {
        let (_tmp, index) = create_test_index();
        let mut writer = index.index.writer(15_000_000).unwrap();

        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "sibling",
                content: "unique keeper content",
                file_path: Some("/media/other/sibling.pdf"),
                ..Default::default()
            },
        );
        writer.commit().unwrap();
        index.reader.reload().unwrap();

        let results = index.search("keeper", 10, Some("/media/fantasy")).unwrap();

        assert!(results.is_empty());
    }

    #[test]
    fn scoped_search_includes_nested_subfolders() {
        let (_tmp, index) = create_test_index();
        let mut writer = index.index.writer(15_000_000).unwrap();

        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "nested",
                content: "deeply buried avowed lore",
                file_path: Some("/media/fantasy/sub/deeper/nested.pdf"),
                ..Default::default()
            },
        );
        writer.commit().unwrap();
        index.reader.reload().unwrap();

        let results = index.search("avowed", 10, Some("/media/fantasy")).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].doc_id, "nested");
    }

    #[test]
    fn unscoped_search_returns_all_docs() {
        let (_tmp, index) = create_test_index();
        let mut writer = index.index.writer(15_000_000).unwrap();

        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "doc_a",
                content: "shared candlekeep term",
                file_path: Some("/media/fantasy/doc_a.pdf"),
                ..Default::default()
            },
        );
        add_page(
            &index,
            &mut writer,
            TestPage {
                doc_id: "doc_b",
                content: "shared candlekeep term",
                file_path: Some("/media/history/doc_b.pdf"),
                ..Default::default()
            },
        );
        writer.commit().unwrap();
        index.reader.reload().unwrap();

        let results = index.search("candlekeep", 10, None).unwrap();

        assert_eq!(results.len(), 2);
    }
}
