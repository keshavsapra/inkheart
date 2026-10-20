import { persist, createLocalStorage } from '@macfja/svelte-persistent-store'
import { writable } from 'svelte/store'

type SortMode = 'asc' | 'dsc'

const sortModeStore = persist<SortMode>(writable('asc'), createLocalStorage(), 'sortMode')
const sortPropStore = persist(writable('title'), createLocalStorage(), 'sortProp')

type SearchMode = 'all' | 'file' | 'dir'

const searchModeStore = persist<SearchMode>(writable('all'), createLocalStorage(), 'searchMode')

type Theme =
  | 'light'
  | 'dark'
  | 'system'
  | 'glassmorphism'
  | 'neomorphism'
  | 'aurora'
  | 'neo-brutalism'
  | 'cyberpunk'
  | 'monochrome'

const selectedTheme = persist<Theme>(writable('system'), createLocalStorage(), 'theme')

export type { SortMode, SearchMode, Theme }
export { sortModeStore, sortPropStore, searchModeStore, selectedTheme }
