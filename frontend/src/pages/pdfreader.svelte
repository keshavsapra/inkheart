<style>
  main {
    width: 100%;
    max-width: 100%;
    min-width: 0;
    height: 100%;

    box-sizing: border-box;
    overflow: hidden;
  }

  .toolbar {
    position: fixed;
    right: 2rem;
    bottom: 1.5rem;

    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0.75rem;
  }

  @media (--small-viewport) {
    main {
      max-width: 100vw;
      width: 100vw;
      height: calc(100vh - var(--app-bar-height));
      height: calc(100dvh - var(--app-bar-height));
    }
  }
</style>

<script lang="ts">
  import { onMount } from 'svelte'
  import IframePdf from '../ui/components/pdf/IframePdf.svelte'
  import NativePdf from '../ui/components/pdf/NativePdf.svelte'
  import EpubReader from '../ui/components/epub/EpubReader.svelte'
  import EmbedModal from '../ui/components/modal/EmbedModal.svelte'
  import Button from '../ui/components/button/Button.svelte'
  import { useLocation } from 'svelte-navigator'
  import queryString from 'query-string'
  import { fileUrl, apiUrl } from '../utils/api'
  import { routeUrl } from '../utils/path'
  import { loadPageProgress, savePageProgress, useLegacyReaderStore } from '../stores/reader'

  export let file = ''

  let modalOpen = false

  let location = useLocation()

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
  let previewTitle = ''

  $: navPreviewTitle = $location.state?.previewTitle || ''
  $: stateFilename = $location.state?.filename || ''

  $: isEpub = stateFilename ? stateFilename.toLowerCase().endsWith('.epub') : false
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

  const handleEmbedClick = () => {
    modalOpen = !modalOpen
  }
</script>

<main>
  {#if isEpub}
    <EpubReader
      file="{fileUrl(file)}"
      fileId="{file}"
      title="{navPreviewTitle || itemTitle}"
      initialPage="{currentPage}"
      initialCfi="{cfi}"
      mode="reader"
    />
  {:else if $useLegacyReaderStore}
    <IframePdf file="{fileUrl(file)}" bind:page="{currentPage}" bind:previewTitle="{previewTitle}" />
    <div class="toolbar">
      <Button variant="secondary" href="{`${routeUrl(`/fullscreen/${file}`)}?page=${currentPage}`}" target="_blank">
        Fullscreen
      </Button>
      <Button variant="secondary" onClick="{handleEmbedClick}">Embed</Button>
    </div>
    <EmbedModal
      bind:open="{modalOpen}"
      baseUrl="{`${window.location.origin}${routeUrl(`/embed/${file}`)}`}"
      pageNumber="{currentPage}"
      previewText="{navPreviewTitle || previewTitle}"
    />
  {:else}
    <NativePdf
      file="{fileUrl(file)}"
      fileId="{file}"
      title="{navPreviewTitle}"
      bind:page="{currentPage}"
      mode="reader"
    />
  {/if}
</main>
