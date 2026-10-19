import { writable } from 'svelte/store'
import type { Collection } from '../types/collections'
import { getCollections } from '../services/collection-service'

export const collections = writable<Collection[]>([])
export const activeCollectionId = writable<string | null>(null)

export const loadCollections = async (): Promise<void> => {
  try {
    const data = await getCollections()
    collections.set(data.collections)
  } catch (error) {
    console.error('Failed to load collections:', error)
    collections.set([])
  }
}

export const getActiveCollection = (collectionId: string | null, allCollections: Collection[]): Collection | null => {
  if (!collectionId) return null
  return allCollections.find((c) => c.id === collectionId) || null
}
