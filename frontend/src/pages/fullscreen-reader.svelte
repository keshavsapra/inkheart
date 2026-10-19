<style>
  main {
    width: 100vw;
    height: calc(100vh);
    height: calc(100dvh);

    display: flex;
    justify-content: center;
    box-sizing: border-box;
    overflow-y: hidden;
  }
</style>

<script lang="ts">
  import { onMount } from 'svelte'
  import { useLocation } from 'svelte-navigator'
  import IframePdf from '../ui/components/pdf/IframePdf.svelte'
  import NativePdf from '../ui/components/pdf/NativePdf.svelte'
  import EpubReader from '../ui/components/epub/EpubReader.svelte'
  import queryString from 'query-string'
  import { fileUrl, apiUrl } from '../utils/api'
  import { loadPageProgress, savePageProgress, useLegacyReaderStore } from '../stores/reader'

  let location = useLocation()

  export let file = ''
  export let mode: 'fullscreen' | 'embed' = 'fullscreen'

  const getHashValue = (key: string) => {
    const matches = $location.hash.match(new RegExp(`${key}=([^&]*)`))
    return matches ? matches[1] : null
  }

  const parsedQuery = queryString.parse($location.search)
  let page: string = (parsedQuery['page'] as string) || getHashValue('page') || ''
  let cfi: string = (parsedQuery['cfi'] as string) || getHashValue('cfi') || ''

  // Load saved progress if no explicit page in URL
  const savedPage = page ? null : loadPageProgress(file)
  let currentPage = parseInt(page) || savedPage || 1

  $: navPreviewTitle = $location.state?.previewTitle || ''
  $: stateFilename = $location.state?.filename || ''

  let isEpub = stateFilename ? stateFilename.toLowerCase().endsWith('.epub') : false
  let itemTitle = ''

  // Save reading progress when page changes (for PDFs)
  $: if (!isEpub && currentPage && file) {
    savePageProgress(file, currentPage)
  }

  onMount(async () => {
    if (!stateFilename && file) {
      try {
        const res = await fetch(apiUrl(`api/item/${file}`))
        if (res.ok) {
          const item = await res.json()
          if (item.filename) {
            isEpub = item.filename.toLowerCase().endsWith('.epub')
          }
          if (item.title) {
            itemTitle = item.title
          }
        }
      } catch (e) {
        console.error('Failed to load item info:', e)
      }
    }
  })
</script>

<main>
  {#if isEpub}
    <EpubReader
      file="{fileUrl(file)}"
      fileId="{file}"
      title="{navPreviewTitle || itemTitle}"
      initialPage="{currentPage}"
      initialCfi="{cfi}"
      mode="{mode}"
    />
  {:else if $useLegacyReaderStore}
    <IframePdf file="{fileUrl(file)}" bind:page="{currentPage}" showBackButton="{false}" />
  {:else}
    <NativePdf file="{fileUrl(file)}" fileId="{file}" bind:page="{currentPage}" mode="{mode}" />
  {/if}
</main>
