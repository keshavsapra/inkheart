import type { LibraryItem } from './library-item'

interface PinnedFolder {
  id: string
  pinned_at: number
  display_name?: string
  last_seen_path: string
  current_item?: LibraryItem
}

interface MissingPinnedFolder {
  id: string
  pinned_at: number
  missing_since: number
  display_name?: string
  last_seen_path: string
}

interface PinsResponse {
  version: number
  pinned: PinnedFolder[]
  missing: MissingPinnedFolder[]
}

export type { PinnedFolder, MissingPinnedFolder, PinsResponse }
