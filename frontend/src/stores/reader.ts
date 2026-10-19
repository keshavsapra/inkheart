import { persist, createLocalStorage } from '@macfja/svelte-persistent-store'
import { writable, get } from 'svelte/store'

type PageSpreadMode = 'none' | 'odd' | 'even'

// PDF.js SpreadMode enum values
const SPREAD_MODE_MAP = {
  none: 0,
  odd: 1,
  even: 2,
} as const

const SPREAD_MODE_REVERSE_MAP = {
  0: 'none',
  1: 'odd',
  2: 'even',
} as const

const ZOOM_LEVELS = [
  'auto',
  'page-actual',
  'page-fit',
  'page-width',
  '0.5',
  '0.75',
  '1',
  '1.25',
  '1.5',
  '2',
  '3',
  '4',
] as const

type ZoomLevel = (typeof ZOOM_LEVELS)[number]

interface PdfJsPreferences {
  spreadModeOnLoad?: number
  defaultZoomValue?: string
  [key: string]: any
}

function getPdfJsPreferences(): PdfJsPreferences {
  try {
    const prefs = localStorage.getItem('pdfjs.preferences')
    return prefs ? JSON.parse(prefs) : {}
  } catch (e) {
    console.error('Failed to read pdfjs.preferences:', e)
    return {}
  }
}

function setPdfJsPreferences(updates: Partial<PdfJsPreferences>): void {
  try {
    const prefs = getPdfJsPreferences()
    const merged = { ...prefs, ...updates }
    localStorage.setItem('pdfjs.preferences', JSON.stringify(merged))
  } catch (e) {
    console.error('Failed to write pdfjs.preferences:', e)
  }
}

function getInitialSpreadMode(): PageSpreadMode {
  const prefs = getPdfJsPreferences()
  const spreadModeValue = prefs.spreadModeOnLoad

  if (typeof spreadModeValue === 'number' && spreadModeValue in SPREAD_MODE_REVERSE_MAP) {
    return SPREAD_MODE_REVERSE_MAP[spreadModeValue as keyof typeof SPREAD_MODE_REVERSE_MAP]
  }

  return 'none'
}

function getInitialZoomLevel(): ZoomLevel {
  const prefs = getPdfJsPreferences()
  const zoomValue = prefs.defaultZoomValue

  if (typeof zoomValue === 'string' && (ZOOM_LEVELS as readonly string[]).includes(zoomValue)) {
    return zoomValue as ZoomLevel
  }

  return 'auto'
}

const progressTrackingStore = persist<boolean>(writable(true), createLocalStorage(), 'readerProgressTracking')

const pageSpreadModeStore = persist<PageSpreadMode>(
  writable(getInitialSpreadMode()),
  createLocalStorage(),
  'readerPageSpreadMode',
)

// Sync changes to PDF.js preferences
pageSpreadModeStore.subscribe((mode) => {
  const numericMode = SPREAD_MODE_MAP[mode]
  setPdfJsPreferences({ spreadModeOnLoad: numericMode })
})

const defaultZoomStore = persist<ZoomLevel>(writable(getInitialZoomLevel()), createLocalStorage(), 'readerDefaultZoom')

// Sync changes to PDF.js preferences
defaultZoomStore.subscribe((zoom) => {
  setPdfJsPreferences({ defaultZoomValue: zoom })
})

type DocumentCardMode = 'poster' | 'minimal'

const documentCardModeStore = persist<DocumentCardMode>(
  writable<DocumentCardMode>('poster'),
  createLocalStorage(),
  'documentCardMode',
)

const showProgressOnCoversStore = persist<boolean>(writable(true), createLocalStorage(), 'showProgressOnCovers')

const useLegacyReaderStore = persist<boolean>(writable(false), createLocalStorage(), 'useLegacyReader')

const collapsibleToolbarStore = persist<boolean>(writable(true), createLocalStorage(), 'readerCollapsibleToolbar')

interface ReadingProgressMap {
  [fileId: string]: {
    page: number
    lastRead: number
    cfi?: string
  }
}

const READING_PROGRESS_KEY = 'readingProgress'

function getReadingProgress(): ReadingProgressMap {
  try {
    const data = localStorage.getItem(READING_PROGRESS_KEY)
    return data ? JSON.parse(data) : {}
  } catch (e) {
    console.error('Failed to read reading progress:', e)
    return {}
  }
}

function saveReadingProgress(progressMap: ReadingProgressMap): void {
  try {
    localStorage.setItem(READING_PROGRESS_KEY, JSON.stringify(progressMap))
  } catch (e) {
    console.error('Failed to save reading progress:', e)
  }
}

export function savePageProgress(fileId: string, page: number, cfi?: string): void {
  if (!get(progressTrackingStore)) return

  const progress = getReadingProgress()
  progress[fileId] = {
    page,
    lastRead: Date.now(),
    ...(cfi ? { cfi } : {}),
  }
  saveReadingProgress(progress)
}

export function loadPageProgress(fileId: string): number | null {
  if (!get(progressTrackingStore)) return null

  const progress = getReadingProgress()
  return progress[fileId]?.page ?? null
}

export function loadEpubProgress(fileId: string): { page: number; cfi?: string } | null {
  if (!get(progressTrackingStore)) return null

  const progress = getReadingProgress()
  return progress[fileId] ? { page: progress[fileId].page, cfi: progress[fileId].cfi } : null
}

export function clearPageProgress(fileId: string): void {
  const progress = getReadingProgress()
  delete progress[fileId]
  saveReadingProgress(progress)
}

export function clearAllProgress(): void {
  try {
    localStorage.removeItem(READING_PROGRESS_KEY)
  } catch (e) {
    console.error('Failed to clear reading progress:', e)
  }
}

export type { PageSpreadMode, ZoomLevel, DocumentCardMode }
export {
  progressTrackingStore,
  pageSpreadModeStore,
  defaultZoomStore,
  documentCardModeStore,
  showProgressOnCoversStore,
  useLegacyReaderStore,
  collapsibleToolbarStore,
}
