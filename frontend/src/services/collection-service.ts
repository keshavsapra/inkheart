import { createGetRequest, createPostRequest, createPatchRequest, createDeleteRequest } from '../lib/create-request'
import { apiUrl } from '../utils/api'
import type { CollectionsResponse } from '../types/collections'

export const getCollections = async (): Promise<CollectionsResponse> => {
  const response = await fetch(...createGetRequest(apiUrl('api/collections')))
  if (!response.ok) {
    const text = await response.text()
    throw new Error(text || `Failed to fetch collections: ${response.statusText}`)
  }
  return response.json()
}

export const createCollection = async (name: string): Promise<{ id: string }> => {
  const response = await fetch(
    ...createPostRequest(apiUrl('api/collections'), JSON.stringify({ name }), {
      'Content-Type': 'application/json',
    }),
  )
  if (!response.ok) {
    const text = await response.text()
    throw new Error(text || `Failed to create collection: ${response.statusText}`)
  }
  return response.json()
}

export const renameCollection = async (id: string, name: string): Promise<void> => {
  const response = await fetch(
    ...createPatchRequest(apiUrl(`api/collections/${id}`), JSON.stringify({ name }), {
      'Content-Type': 'application/json',
    }),
  )
  if (!response.ok) {
    const text = await response.text()
    throw new Error(text || `Failed to rename collection: ${response.statusText}`)
  }
}

export const deleteCollection = async (id: string): Promise<void> => {
  const response = await fetch(...createDeleteRequest(apiUrl(`api/collections/${id}`)))
  if (!response.ok) {
    const text = await response.text()
    throw new Error(text || `Failed to delete collection: ${response.statusText}`)
  }
}

export const addItemToCollection = async (collectionId: string, itemId: string): Promise<void> => {
  const response = await fetch(
    ...createPostRequest(
      apiUrl(`api/collections/${collectionId}/items`),
      JSON.stringify({ item_id: itemId }),
      {
        'Content-Type': 'application/json',
      },
    ),
  )
  if (!response.ok) {
    const text = await response.text()
    throw new Error(text || `Failed to add item to collection: ${response.statusText}`)
  }
}

export const removeItemFromCollection = async (collectionId: string, itemId: string): Promise<void> => {
  const response = await fetch(...createDeleteRequest(apiUrl(`api/collections/${collectionId}/items/${itemId}`)))
  if (!response.ok) {
    const text = await response.text()
    throw new Error(text || `Failed to remove item from collection: ${response.statusText}`)
  }
}
