import { get } from 'svelte/store'
import { basepath } from '../stores/basepath'

const routeUrl = (path: string): string => {
  const base = get(basepath)
  // Remove leading slash from path if present
  const cleanPath = path.startsWith('/') ? path.slice(1) : path
  // Remove trailing slash from base if present
  const cleanBase = base.endsWith('/') ? base.slice(0, -1) : base
  return `${cleanBase}/${cleanPath}`
}

export { routeUrl }
