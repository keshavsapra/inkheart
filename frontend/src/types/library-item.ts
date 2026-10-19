interface AncestorRecord {
  id: string
  title: string
}

interface LibraryItem {
  id: string
  filename: string
  title: string
  cover_name: string
  is_dir: boolean
  dir_previews: string[]
  children: LibraryItem[]
  last_modified: number
  created: number
  pages?: number
  ancestors?: AncestorRecord[]
}

interface SearchResultItem extends LibraryItem {
  search_source?: 'filename' | 'content'
  page_num?: number
  match_count?: number
}

interface FullTextResult {
  doc_id: string
  page_num: number
  title: string
  cover_name: string
  is_dir: boolean
  match_count: number
  pages?: number
}

export type { LibraryItem, SearchResultItem, FullTextResult, AncestorRecord }
