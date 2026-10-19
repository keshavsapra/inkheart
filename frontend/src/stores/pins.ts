import { writable } from 'svelte/store'
import type { PinnedFolder, MissingPinnedFolder } from '../types/pins'
import { getPins } from '../services/pin-service'

export const pinnedFolders = writable<PinnedFolder[]>([])
export const missingPins = writable<MissingPinnedFolder[]>([])

export const loadPins = async (): Promise<void> => {
  try {
    const data = await getPins()
    pinnedFolders.set(data.pinned)
    missingPins.set(data.missing)
  } catch (error) {
    console.error('Failed to load pins:', error)
    // Keep empty arrays on error
    pinnedFolders.set([])
    missingPins.set([])
  }
}
