import { createDeleteRequest, createGetRequest, createPostRequest } from '../lib/create-request'
import type { PinsResponse } from '../types/pins'
import { apiUrl } from '../utils/api'

export const getPins = async (): Promise<PinsResponse> => {
  const response = await fetch(...createGetRequest(apiUrl('api/pins')))

  if (!response.ok) {
    throw new Error(`Failed to fetch pins: ${response.statusText}`)
  }

  return await response.json()
}

export const addPin = async (id: string, displayName?: string): Promise<void> => {
  const body = { id, display_name: displayName }
  const response = await fetch(
    ...createPostRequest(apiUrl('api/pins'), JSON.stringify(body), {
      'Content-Type': 'application/json',
    })
  )

  if (!response.ok) {
    const text = await response.text()
    throw new Error(text || `Failed to add pin: ${response.statusText}`)
  }
}

export const removePin = async (id: string): Promise<void> => {
  const response = await fetch(...createDeleteRequest(apiUrl(`api/pins/${id}`)))

  if (!response.ok) {
    throw new Error(`Failed to remove pin: ${response.statusText}`)
  }
}

export const removeMissingPin = async (id: string): Promise<void> => {
  const response = await fetch(...createDeleteRequest(apiUrl(`api/pins/missing/${id}`)))

  if (!response.ok) {
    throw new Error(`Failed to remove missing pin: ${response.statusText}`)
  }
}
