<style>
  .collection {
    width: 100%;
    height: 100vh;
    height: 100dvh;

    display: grid;
    grid-template-rows: auto 1fr;
    grid-template-areas: 'header' 'content';
  }

  .header {
    grid-area: header;
  }

  .scrollbox {
    overflow-y: scroll;
    grid-area: content;
  }

  .loader {
    width: 100%;
    opacity: 0.6;
    margin: 2rem 0;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .empty-state {
    padding: 3rem;
    text-align: center;
  }

  .empty-title {
    font-size: var(--font-2xl);
    color: var(--text-color-default);
    margin-bottom: 0.5rem;
  }

  .empty-message {
    font-size: var(--font-base);
    color: var(--text-color-light);
    opacity: 0.7;
  }

  @media (--small-viewport) {
    .collection {
      height: calc(100vh - var(--app-bar-height));
      height: calc(100dvh - var(--app-bar-height));
    }
  }

  .header-attached {
    position: absolute;
    top: 5.5rem;
    width: 100%;
    max-height: calc(100vh - 5.5rem);
    max-height: calc(100dvh - 5.5rem);
    overflow-y: auto;
    background: var(--background);
    box-shadow: var(--elevation-2);
    z-index: 50;
  }
</style>

<script lang="ts">
  import { onMount } from 'svelte'
  import { SyncLoader } from 'svelte-loading-spinners'
  import { useNavigate } from 'svelte-navigator'
  import { collections, loadCollections, getActiveCollection, activeCollectionId } from '../../../../stores/collections'
  import type { Collection } from '../../../../types/collections'
  import LibraryHeader from '../library/LibraryHeader.svelte'
  import CollectionGrid from './CollectionGrid.svelte'
  import SearchResults from '../library/search/SearchResults.svelte'
  import { createSearchManager } from '../../../../lib/search'
  import { setFolderContext, clearFolderContext } from '../../../../stores/folder-context'
  import { createScrollCollapse } from '../../../../lib/scroll-collapse'

  const navigate = useNavigate()
  const { searchActive, filteredResults, onSearchChange, onClear } = createSearchManager()

  export let collectionId: string

  let loading = true
  let collection: Collection | null = null

  // Set active collection ID
  $: activeCollectionId.set(collectionId)

  // Get collection data
  $: collection = getActiveCollection(collectionId, $collections)

  // Filter out items where current_item is null (deleted files)
  $: validItems = collection?.items.filter((item) => item.current_item != null).map((item) => item.current_item!) || []

  $: if (collection) {
    setFolderContext({ title: collection.name, ancestors: [], onBack: handleBackButton })
  }

  const sortableProps = [
    { key: 'title', display: 'name' },
    { key: 'last_modified', display: 'modified' },
    { key: 'created', display: 'created' },
  ]

  onMount(async () => {
    try {
      loading = true
      await loadCollections()
    } catch (error) {
      console.error('Failed to load collection:', error)
    } finally {
      loading = false
    }
  })

  // Clean up active collection on unmount
  onMount(() => {
    return () => {
      activeCollectionId.set(null)
      clearFolderContext()
    }
  })

  const { showToolbar, register: scrollboxAction, reset: resetToolbar } = createScrollCollapse()

  const handleBackButton = () => {
    navigate(-1)
  }
</script>

<div class="collection">
  <div class="header">
    {#if collection}
      <LibraryHeader
        title="{collection.name}"
        itemCount="{validItems.length}"
        onSearchChange="{onSearchChange}"
        onClear="{onClear}"
        sortableProps="{sortableProps}"
        onHandle="{resetToolbar}"
        onBack="{handleBackButton}"
        showToolbar="{$showToolbar}"
      >
        {#if $searchActive}
          <div class="header-attached">
            <SearchResults results="{$filteredResults}" />
          </div>
        {/if}
      </LibraryHeader>
    {/if}
  </div>

  <div use:scrollboxAction class="scrollbox">
    {#if loading}
      <div class="loader">
        <SyncLoader size="3" color="var(--button-primary)" unit="rem" duration="1s" />
      </div>
    {:else if !collection}
      <div class="empty-state">
        <div class="empty-title">Collection not found</div>
      </div>
    {:else if collection.items.length === 0}
      <div class="empty-state">
        <div class="empty-title">Empty collection.</div>
        <div class="empty-message">Add items by browsing or searching.</div>
      </div>
    {:else}
      <CollectionGrid collection="{collection}" />
    {/if}
  </div>
</div>
