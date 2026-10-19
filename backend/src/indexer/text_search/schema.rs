use std::path::Path;

use tantivy::{
    Index, IndexReader, ReloadPolicy,
    directory::MmapDirectory,
    schema::{Facet, FacetOptions, Field, STORED, STRING, Schema, TEXT},
};

/// Encodes a file path into a hierarchical `Facet`
pub fn path_facet(path: &str) -> Facet {
    let segments: Vec<&str> = path
        .split(['/', '\\'])
        .filter(|s| !s.is_empty() && *s != ".")
        .collect();

    Facet::from_path(segments)
}

pub struct TextSearchIndex {
    pub index: Index,
    pub reader: IndexReader,
    pub doc_id_field: Field,
    pub page_num_field: Field,
    pub title_field: Field,
    pub content_field: Field,
    pub headings_field: Field,
    pub bookmarks_field: Field,
    pub cover_name_field: Field,
    pub path_field: Field,
}

impl TextSearchIndex {
    pub fn open_or_create(index_dir: &Path) -> Result<Self, String> {
        std::fs::create_dir_all(index_dir)
            .map_err(|e| format!("Failed to create text index directory: {}", e))?;

        let mut schema_builder = Schema::builder();
        let doc_id_field = schema_builder.add_text_field("doc_id", STRING | STORED);
        let page_num_field =
            schema_builder.add_u64_field("page_num", tantivy::schema::INDEXED | STORED);
        let title_field = schema_builder.add_text_field("title", TEXT | STORED);
        let content_field = schema_builder.add_text_field("content", TEXT);
        let headings_field = schema_builder.add_text_field("headings", TEXT);
        let bookmarks_field = schema_builder.add_text_field("bookmarks", TEXT);
        let cover_name_field = schema_builder.add_text_field("cover_name", STRING | STORED);
        let path_field = schema_builder.add_facet_field("path", FacetOptions::default());
        let schema = schema_builder.build();

        let dir = MmapDirectory::open(index_dir)
            .map_err(|e| format!("Failed to open mmap directory: {}", e))?;

        let index = Index::open_or_create(dir, schema.clone())
            .map_err(|e| format!("Failed to open or create text index: {}", e))?;

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .map_err(|e| format!("Failed to create index reader: {}", e))?;

        Ok(TextSearchIndex {
            index,
            reader,
            doc_id_field,
            page_num_field,
            title_field,
            content_field,
            headings_field,
            bookmarks_field,
            cover_name_field,
            path_field,
        })
    }
}
