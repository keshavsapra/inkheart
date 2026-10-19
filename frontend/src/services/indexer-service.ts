import { toast } from '@zerodevx/svelte-toast'
import { createPostRequest } from '../lib/create-request'
import IndexerFinishedToast from '../ui/toasts/IndexerFinishedToast.svelte'
import { apiUrl } from '../utils/api'

const startIndexer = async (): Promise<void> => {
  const response = await fetch(...createPostRequest(apiUrl('api/indexer/start')))

  if (!response.ok) {
    throw new Error(`Failed to start indexer: ${response.statusText}`)
  }
}

const isLibraryPage = () => {
  const path = window.location.pathname
  return path === '/' || path.startsWith('/library')
}

const showIndexerFinishedToast = () => {
  if (isLibraryPage()) {
    toast.push({
      component: {
        src: IndexerFinishedToast,
        props: {},
      },
      dismissable: true,
      duration: 5000,
    })
  } else {
    // fallback: simple toast without reload button
    toast.push('Indexing finished.', {
      dismissable: true,
      duration: 3000,
    })
  }
}

export { startIndexer, showIndexerFinishedToast }
