import { get } from 'svelte/store'
import { basepath } from '../stores/basepath'

const apiUrl = (path: string): string => {
  const base = get(basepath)
  // Remove leading slash from path if present
  const cleanPath = path.startsWith('/') ? path.slice(1) : path
  // Remove trailing slash from base if present
  const cleanBase = base.endsWith('/') ? base.slice(0, -1) : base
  return `${cleanBase}/${cleanPath}`
}

// Convenience functions for common endpoints
const fileUrl = (fileId: string): string => apiUrl(`api/file/${fileId}`)

const coverUrl = (fileId: string): string => apiUrl(`cover/${fileId}`)

const thumbnailUrl = (filename: string): string => apiUrl(`thumbnail/${filename}`)

export { apiUrl, fileUrl, coverUrl, thumbnailUrl }
