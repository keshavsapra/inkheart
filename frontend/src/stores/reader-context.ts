import { writable } from 'svelte/store'

export type ReaderContext = {
  title: string
  onBack: () => void
}

export const readerContext = writable<ReaderContext | null>(null)

export const setReaderContext = (context: ReaderContext) => {
  readerContext.set(context)
}

export const clearReaderContext = () => {
  readerContext.set(null)
}
