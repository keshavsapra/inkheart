import type { LibraryItem } from './library-item'

export interface Collection {
  id: string
  name: string
  created_at: number
  updated_at: number
  items: CollectionItem[]
}

export interface CollectionItem {
  id: string
  added_at: number
  last_seen_path: string
  current_item?: LibraryItem
}

export interface CollectionsResponse {
  version: number
  collections: Collection[]
}
