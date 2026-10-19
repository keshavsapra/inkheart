<style>
  .library {
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

  .header-attached {
    position: absolute;
    width: 100%;
    background: var(--card-subtle-alpha);

    max-height: 60vh;
    overflow-y: scroll;

    box-shadow: var(--elevation-1);

    scrollbar-width: none;
  }

  .header-attached::-webkit-scrollbar {
    display: none;
  }

  .header-attached::after {
    content: '';
    position: sticky;
    bottom: 0;
    left: 0;
    right: 0;
    display: block;
    height: 2rem;
    margin-top: -2rem;
    background: linear-gradient(to top, rgba(0, 0, 0, 0.5), transparent);
    pointer-events: none;
  }

  .scrollbox {
    overflow-y: scroll;
    grid-area: content;
    position: relative;
  }

  .dim-overlay {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 5;
    cursor: pointer;
  }

  .loader {
    width: 100%;

    opacity: 0.6;

    margin: 2rem 0;

    display: flex;
    justify-content: center;
    align-items: center;
  }

  .no-results {
    padding: 3rem;
    color: var(--text-color-light);
    opacity: 0.6;
    font-size: var(--font-lg);
  }

  @media (--small-viewport) {
    .library {
      height: calc(100vh - var(--app-bar-height));
      height: calc(100dvh - var(--app-bar-height));
    }
  }
</style>

<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { sortModeStore, sortPropStore } from '../../../../stores/options'

  import { SyncLoader } from 'svelte-loading-spinners'
  import BookGrid from './BookGrid.svelte'
  import ErrorMessage from '../../error/ErrorMessage.svelte'
  import { createGetRequest } from '../../../../lib/create-request'
  import LibraryHeader from './LibraryHeader.svelte'
  import type { LibraryItem, AncestorRecord } from '../../../../types/library-item'
  import SearchResults from './search/SearchResults.svelte'
  import { useNavigate } from 'svelte-navigator'
  import { apiUrl } from '../../../../utils/api'
  import { createSearchManager } from '../../../../lib/search'
  import { setFolderScope, setGlobalScope } from '../../../../stores/search-scope'
  import { setFolderContext, clearFolderContext } from '../../../../stores/folder-context'
  import { createScrollCollapse } from '../../../../lib/scroll-collapse'

  export let directory: string = ''

  const navigate = useNavigate()
  const { searchActive, loadingSearch, filteredResults, onSearchChange, onClear } = createSearchManager()

  let unauthorized = false
  $: loadingLibrary = false

  let title = ''
  let items: LibraryItem[] = []
  let ancestors: AncestorRecord[] = []

  const sortableProps = [
    { key: 'title', display: 'name' },
    { key: 'last_modified', display: 'modified' },
    { key: 'created', display: 'created' },
  ]

  const sortItems = (items: LibraryItem[], order: 'asc' | 'dsc', prop: string) => {
    items.sort((a, b) => {
      if (order === 'asc') {
        return a[prop] < b[prop] ? -1 : a[prop] > b[prop] ? 1 : 0
      } else {
        // 'desc' order
        return b[prop] < a[prop] ? -1 : b[prop] > a[prop] ? 1 : 0
      }
    })

    return items
  }

  $: sorted = (() => {
    const dirs = sortItems(
      items.filter((d) => d.is_dir),
      $sortModeStore,
      $sortPropStore,
    )

    const books = sortItems(
      items.filter((d) => !d.is_dir),
      $sortModeStore,
      $sortPropStore,
    )

    return [...dirs, ...books]
  })()

  const loadDirectory = async (dir: string) => {
    const endpoint = directory ? apiUrl(`api/item/${directory}`) : apiUrl('api/mediaroot')

    try {
      loadingLibrary = true
      const response = await fetch(...createGetRequest(endpoint))
      const data = await response.json()

      unauthorized = false
      items = data.children
      ancestors = data.ancestors ?? []
      title = data.title === 'media' ? '' : data.title
      loadingLibrary = false

      if (directory && title) {
        setFolderScope(directory, title)
        setFolderContext({ title, ancestors, onBack: handleBackButton })
      } else {
        setGlobalScope()
        clearFolderContext()
      }
    } catch (e) {
      loadingLibrary = false
      unauthorized = true
      ancestors = []
      clearFolderContext()
    }
  }

  $: loadDirectory(directory)

  onMount(async () => {
    await loadDirectory(directory)
  })

  onDestroy(() => {
    clearFolderContext()
  })

  const { showToolbar, register: scrollboxAction, reset: resetToolbar } = createScrollCollapse()

  const handleBackButton = () => {
    if (ancestors.length === 0) {
      navigate(-1)
      return
    }

    const parent = ancestors[ancestors.length - 1]

    if (ancestors.length === 1) {
      navigate('/')
    } else {
      navigate(`/library/${parent.id}`)
    }
  }
</script>

<div class="library">
  {#if unauthorized}
    <ErrorMessage message="Unauthorized" />
  {:else}
    <!--<div class="toolbar">
        <SortButton></SortButton>
      </div>-->
    <div class="header">
      <LibraryHeader
        title="{title}"
        currentFolderId="{directory}"
        ancestors="{ancestors}"
        itemCount="{sorted.length}"
        sortableProps="{sortableProps}"
        onSearchChange="{onSearchChange}"
        onClear="{onClear}"
        onHandle="{resetToolbar}"
        onBack="{handleBackButton}"
        showToolbar="{$showToolbar}"
      >
        <div class="header-attached">
          {#if $loadingSearch}
            <div class="loader">
              <SyncLoader size="5" color="var(--button-primary)" unit="rem" duration="1s" />
            </div>
          {:else if $searchActive && $filteredResults.length === 0}
            <div class="no-results">
              <span>No results found</span>
            </div>
          {:else}
            <SearchResults results="{$filteredResults}"></SearchResults>
          {/if}
        </div>
      </LibraryHeader>
    </div>

    <div use:scrollboxAction class="scrollbox">
      {#if loadingLibrary}
        <div class="loader">
          <SyncLoader size="5" color="var(--button-primary)" unit="rem" duration="1s" />
        </div>
      {:else}
        <BookGrid items="{sorted}" />
      {/if}

      {#if $searchActive}
        <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
        <div class="dim-overlay" on:click="{onClear}"></div>
      {/if}
    </div>
  {/if}
</div>
