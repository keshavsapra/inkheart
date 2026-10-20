<style>
  .epub-reader {
    position: relative;
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    max-width: 100%;
    overflow: hidden;
    text-align: left;
    background: var(--background);
  }

  .reader-body {
    position: relative;
    display: flex;
    flex: 1;
    min-height: 0;
    min-width: 0;
    width: 100%;
    max-width: 100%;
    overflow: hidden;
  }

  /* Top Bar */
  .bar {
    box-sizing: border-box;
    display: flex;
    align-items: center;
    gap: 6px;
    height: 40px;
    padding: 0 12px;
    background: var(--background);
    border-bottom: 1px solid var(--card-color);
    z-index: 10;
  }

  .cluster {
    flex: 1 1 0;
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .cluster:last-child {
    justify-content: flex-end;
  }

  .title {
    flex: 0 1 auto;
    min-width: 0;
    margin: 0 8px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    text-align: center;
    color: var(--text-color-default);
    font-size: var(--font-xs);
    font-weight: var(--weight-bold);
  }

  .icon-btn {
    all: unset;
    box-sizing: border-box;
    width: 32px;
    height: 32px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--text-color-light);
    border-radius: var(--radius-sm, 6px);
    cursor: pointer;
    transition: color 120ms ease, background 120ms ease;
  }

  .icon-btn:hover {
    color: var(--text-color-default);
    background: var(--card-subtle);
  }

  .icon-btn.active {
    color: var(--color-primary-500);
    background: var(--card-subtle);
  }

  /* Sidebar TOC Drawer */
  .sidebar-host {
    width: 280px;
    height: 100%;
    background: var(--card-subtle);
    border-right: 1px solid var(--card-color);
    display: flex;
    flex-direction: column;
    z-index: 15;
    transition: transform 200ms ease;
    overflow: hidden;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 14px;
    border-bottom: 1px solid var(--card-color);
    font-size: var(--font-xs);
    font-weight: var(--weight-bold);
    color: var(--text-color-default);
  }

  .toc-list {
    flex: 1;
    overflow-y: auto;
    padding: 8px 0;
    list-style: none;
    margin: 0;
  }

  .toc-item {
    display: block;
    width: 100%;
    box-sizing: border-box;
    padding: 8px 16px;
    font-size: var(--font-xs);
    color: var(--text-color-light);
    text-align: left;
    background: none;
    border: none;
    cursor: pointer;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    transition: background 120ms ease, color 120ms ease;
  }

  .toc-item:hover {
    color: var(--text-color-default);
    background: var(--card-color);
  }

  .toc-item.active {
    color: var(--color-primary-500);
    font-weight: var(--weight-bold);
    background: var(--card-color);
  }

  .toc-subitem {
    padding-left: 28px;
    font-size: var(--font-2xs);
  }

  .scrim {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.4);
    z-index: 12;
  }

  /* Viewer Area */
  .viewer-wrap {
    position: relative;
    flex: 1;
    height: 100%;
    width: 100%;
    min-width: 0;
    max-width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    overflow: hidden;
  }

  .viewer-container {
    width: 100%;
    max-width: 900px;
    height: 100%;
    margin: 0 auto;
    position: relative;
    overflow: hidden;
    display: flex;
    justify-content: center;
  }

  .viewer-container :global(.epub-container) {
    width: 100% !important;
    max-width: 100% !important;
    height: 100% !important;
    overflow: hidden !important;
  }

  .viewer-container :global(.epub-view) {
    width: 100% !important;
    max-width: 100% !important;
    height: 100% !important;
    overflow: hidden !important;
  }

  .viewer-container :global(iframe) {
    width: 100% !important;
    max-width: 100% !important;
    height: 100% !important;
    border: none !important;
  }

  /* Floating Navigation Pill */
  .reader-pill {
    position: absolute;
    bottom: 20px;
    left: 50%;
    transform: translateX(-50%);
    z-index: 25;
    box-sizing: border-box;
    display: inline-flex;
    align-items: center;
    gap: 4px;
    width: max-content;
    height: 36px;
    padding: 0 8px;
    border: 1px solid var(--card-contrast, var(--card-color));
    border-radius: 999px;
    background: var(--card-color);
    box-shadow: var(--elevation-2);
    white-space: nowrap;
    user-select: none;
    pointer-events: auto;
  }

  .pill-btn {
    all: unset;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    color: var(--text-color-light);
    font-size: var(--font-lg);
    cursor: pointer;
    border-radius: 999px;
    transition: color 120ms ease, background 120ms ease;
  }

  .pill-btn:hover {
    color: var(--text-color-default);
    background: var(--card-subtle);
  }

  .pill-label {
    display: inline-flex;
    align-items: baseline;
    gap: 4px;
    padding: 0 6px;
    color: var(--text-color-default);
    font-size: var(--font-xs);
    font-variant-numeric: tabular-nums;
  }

  .pill-sub {
    color: var(--text-color-light);
    font-size: var(--font-2xs);
    margin-left: 2px;
  }

  /* Navigation Margin Zones */
  .nav-zone {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 10%;
    min-width: 40px;
    max-width: 80px;
    z-index: 3;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    cursor: pointer;
    transition: opacity 160ms ease;
  }

  .nav-zone:hover {
    opacity: 0.7;
  }

  .nav-zone.left {
    left: 0;
  }

  .nav-zone.right {
    right: 0;
  }

  .nav-zone-btn {
    all: unset;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: 999px;
    background: var(--card-contrast, rgba(0, 0, 0, 0.4));
    color: var(--text-color-default, #fff);
  }

  /* Loading & Error States */
  .state-message {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    color: var(--text-color-light);
    font-size: var(--font-sm);
  }

  @media (--small-viewport) {
    .sidebar-host {
      position: absolute;
      top: 0;
      bottom: 0;
      left: 0;
      width: 80vw;
      max-width: 300px;
      box-shadow: 6px 0 20px rgba(0, 0, 0, 0.4);
    }

    .nav-zone {
      display: none;
    }
  }
</style>

<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { useNavigate } from 'svelte-navigator'
  import Icon from '@iconify/svelte'
  import ePub, { type Book, type Rendition, type NavItem } from 'epubjs'
  import { selectedTheme } from '../../../stores/options'
  import { isDarkTheme, updateTheme } from '../../../lib/theme'
  import { savePageProgress, loadEpubProgress } from '../../../stores/reader'
  import { usingAuth } from '../../../stores/auth'
  import { getToken } from '../../../lib/localstorage'
  import { get } from 'svelte/store'
  import { routeUrl } from '../../../utils/path'
  import Jumper from 'svelte-loading-spinners/Jumper.svelte'

  export let file = ''
  export let fileId = ''
  export let title = ''
  export let initialPage: number = 1
  export let initialCfi: string = ''
  export let mode: 'reader' | 'fullscreen' | 'embed' = 'reader'

  const navigate = useNavigate()

  let viewerContainer: HTMLDivElement
  let book: Book | null = null
  let rendition: Rendition | null = null

  let loading = true
  let errorMsg = ''
  let sidebarOpen = false
  let isMobile = false

  let toc: NavItem[] = []
  let currentChapterIndex = 0
  let totalChapters = 0
  let currentCfi = ''
  let activeHref = ''

  let locationsReady = false
  let currentLocation = 0
  let totalLocations = 0
  let progressPercentage = 0

  $: darkIcon = isDarkTheme($selectedTheme)

  $: if (rendition && $selectedTheme) {
    applyTheme($selectedTheme)
  }

  const authHeaders = (): Record<string, string> => (get(usingAuth) ? { Authorization: `Bearer ${getToken()}` } : {})

  function applyTheme(themeName: string) {
    if (!rendition) return
    let activeKey = themeName
    if (activeKey === 'system') {
      activeKey = isDarkTheme('system') ? 'dark' : 'light'
    }
    rendition.themes.select(`theme-${activeKey}`)
  }

  function toggleTheme() {
    const isDark = isDarkTheme($selectedTheme)
    updateTheme(isDark ? 'light' : 'dark')
  }

  function handleBack() {
    if (window.history.length > 1) {
      navigate(-1)
    } else {
      navigate('/')
    }
  }

  async function toggleFullscreen() {
    if (mode === 'fullscreen' || mode === 'embed') {
      if (!document.fullscreenElement) {
        document.documentElement.requestFullscreen().catch(() => {})
      } else {
        document.exitFullscreen().catch(() => {})
      }
    } else {
      const targetUrl = `${routeUrl(`/fullscreen/${fileId}`)}?page=${currentChapterIndex + 1}`
      window.open(targetUrl, '_blank')
    }
  }

  function next() {
    if (rendition) {
      rendition.next()
    }
  }

  function prev() {
    if (rendition) {
      rendition.prev()
    }
  }

  function navigateTo(href: string) {
    if (rendition) {
      rendition.display(href)
      activeHref = href
      if (isMobile) {
        sidebarOpen = false
      }
    }
  }

  function updateLocationProgress(cfi: string) {
    if (!book?.locations) return
    try {
      const loc = book.locations.locationFromCfi(cfi)
      if (typeof loc === 'number' && loc >= 0) {
        currentLocation = loc + 1
        totalLocations = book.locations.total || 0
        const pct = book.locations.percentageFromCfi(cfi)
        progressPercentage = Math.round((pct || 0) * 100)
      }
    } catch (_) {}
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'ArrowRight' || e.key === 'PageDown' || e.key === ' ') {
      next()
    } else if (e.key === 'ArrowLeft' || e.key === 'PageUp') {
      prev()
    }
  }

  let _mql: MediaQueryList | null = null
  let _mediaListener: ((e: MediaQueryListEvent) => void) | null = null

  onDestroy(() => {
    if (_mql && _mediaListener) {
      _mql.removeEventListener('change', _mediaListener)
    }
    window.removeEventListener('keydown', handleKeydown)
    if (rendition) {
      try { rendition.destroy() } catch (_) {}
    }
    if (book) {
      try { book.destroy() } catch (_) {}
    }
  })

  onMount(async () => {
    _mql = window.matchMedia('(max-width: 45rem)')
    isMobile = _mql.matches
    _mediaListener = (e: MediaQueryListEvent) => {
      isMobile = e.matches
    }
    _mql.addEventListener('change', _mediaListener)

    try {
      loading = true
      errorMsg = ''

      // Fetch the EPUB file as an ArrayBuffer with auth headers
      const res = await fetch(file, { headers: authHeaders() })
      if (!res.ok) {
        throw new Error(`Failed to download EPUB (status: ${res.status})`)
      }
      const arrayBuffer = await res.arrayBuffer()

      // Initialize epub.js book
      book = ePub(arrayBuffer)
      await book.opened

      // Load table of contents
      const nav = await book.loaded.navigation
      toc = nav?.toc || []

      // Check spine length
      const spine = await book.loaded.spine
      totalChapters = spine?.length || 0

      // Render to container (spread: 'none' ensures single page view without horizontal overflow)
      rendition = book.renderTo(viewerContainer, {
        width: '100%',
        height: '100%',
        flow: 'paginated',
        spread: 'none',
      })

      // Register theme styles
      const baseBodyRules = {
        'font-family': 'system-ui, -apple-system, sans-serif !important',
        'line-height': '1.6 !important',
        'padding': '0 24px !important',
        'box-sizing': 'border-box !important',
        'max-width': '100% !important',
        'overflow-x': 'hidden !important',
      }
      const mediaRules = {
        'max-width': '100% !important',
        'height': 'auto !important',
      }
      const textRules = {
        'word-break': 'break-word !important',
        'overflow-wrap': 'break-word !important',
      }

      rendition.themes.register('theme-dark', {
        body: { ...baseBodyRules, background: '#1a1625 !important', color: '#e4e5f1 !important' },
        'p, div, span, li, blockquote': { ...textRules, color: '#e4e5f1 !important' },
        'h1, h2, h3, h4, h5, h6': { color: '#ffffff !important' },
        'a, a:link, a:visited': { color: '#9171f8 !important' },
        'img, svg, table': mediaRules,
      })

      rendition.themes.register('theme-light', {
        body: { ...baseBodyRules, background: '#ffffff !important', color: '#484b6a !important' },
        'p, div, span, li, blockquote': { ...textRules, color: '#484b6a !important' },
        'h1, h2, h3, h4, h5, h6': { color: '#1a1625 !important' },
        'a, a:link, a:visited': { color: '#5e43f3 !important' },
        'img, svg, table': mediaRules,
      })

      rendition.themes.register('theme-glassmorphism', {
        body: { ...baseBodyRules, background: '#0f172a !important', color: '#f8fafc !important' },
        'p, div, span, li, blockquote': { ...textRules, color: '#f8fafc !important' },
        'h1, h2, h3, h4, h5, h6': { color: '#38bdf8 !important' },
        'a, a:link, a:visited': { color: '#38bdf8 !important' },
        'img, svg, table': mediaRules,
      })

      rendition.themes.register('theme-neomorphism', {
        body: { ...baseBodyRules, background: '#e6ecf5 !important', color: '#2d3748 !important' },
        'p, div, span, li, blockquote': { ...textRules, color: '#2d3748 !important' },
        'h1, h2, h3, h4, h5, h6': { color: '#1a202c !important' },
        'a, a:link, a:visited': { color: '#6366f1 !important' },
        'img, svg, table': mediaRules,
      })

      rendition.themes.register('theme-aurora', {
        body: { ...baseBodyRules, background: '#090d16 !important', color: '#f1f5f9 !important' },
        'p, div, span, li, blockquote': { ...textRules, color: '#f1f5f9 !important' },
        'h1, h2, h3, h4, h5, h6': { color: '#10b981 !important' },
        'a, a:link, a:visited': { color: '#34d399 !important' },
        'img, svg, table': mediaRules,
      })

      rendition.themes.register('theme-neo-brutalism', {
        body: { ...baseBodyRules, background: '#fdfbf7 !important', color: '#000000 !important' },
        'p, div, span, li, blockquote': { ...textRules, color: '#000000 !important' },
        'h1, h2, h3, h4, h5, h6': { color: '#ff5277 !important' },
        'a, a:link, a:visited': { color: '#e02854 !important' },
        'img, svg, table': mediaRules,
      })

      rendition.themes.register('theme-cyberpunk', {
        body: { ...baseBodyRules, background: '#0d0221 !important', color: '#00f0ff !important' },
        'p, div, span, li, blockquote': { ...textRules, color: '#00f0ff !important' },
        'h1, h2, h3, h4, h5, h6': { color: '#ff007f !important' },
        'a, a:link, a:visited': { color: '#ffe600 !important' },
        'img, svg, table': mediaRules,
      })

      rendition.themes.register('theme-monochrome', {
        body: { ...baseBodyRules, background: '#0a0a0a !important', color: '#ededed !important' },
        'p, div, span, li, blockquote': { ...textRules, color: '#ededed !important' },
        'h1, h2, h3, h4, h5, h6': { color: '#ffffff !important' },
        'a, a:link, a:visited': { color: '#ffffff !important' },
        'img, svg, table': mediaRules,
      })

      applyTheme($selectedTheme)

      // Listen to relocation events for progress tracking
      rendition.on('relocated', (location: any) => {
        if (!location?.start) return
        currentCfi = location.start.cfi || ''
        const spineIndex = location.start.index ?? 0
        currentChapterIndex = spineIndex

        if (locationsReady) {
          updateLocationProgress(currentCfi)
        }

        // Save reading progress
        if (fileId) {
          savePageProgress(fileId, spineIndex + 1, currentCfi)
        }

        if (location.start.href) {
          activeHref = location.start.href
        }
      })

      // Generate background locations for page numbering
      book.ready.then(() => {
        return book?.locations.generate(1024)
      }).then(() => {
        locationsReady = true
        if (currentCfi) {
          updateLocationProgress(currentCfi)
        }
      }).catch((e) => {
        console.warn('Epub locations generation skipped:', e)
      })

      // Determine initial position
      const savedProgress = loadEpubProgress(fileId)
      let target: string | number | undefined = undefined

      if (initialCfi) {
        target = initialCfi
      } else if (initialPage > 1 && totalChapters > 0) {
        const spineItem = spine?.get(initialPage - 1)
        target = spineItem?.href || initialPage - 1
      } else if (savedProgress?.cfi) {
        target = savedProgress.cfi
      } else if (savedProgress?.page && savedProgress.page > 1) {
        const spineItem = spine?.get(savedProgress.page - 1)
        target = spineItem?.href || savedProgress.page - 1
      }

      await rendition.display(target)
      loading = false
    } catch (e: any) {
      console.error('Failed to load EPUB:', e)
      errorMsg = e?.message || 'Failed to open EPUB document'
      loading = false
    }

    window.addEventListener('keydown', handleKeydown)
  })
</script>

<div class="epub-reader">
  <!-- Top Bar -->
  <div class="bar">
    <div class="cluster">
      {#if mode === 'reader'}
        <button class="icon-btn" on:click="{handleBack}" title="Back">
          <Icon icon="material-symbols:arrow-back-rounded" width="20" height="20" />
        </button>
      {/if}
      <button
        class="icon-btn"
        class:active="{sidebarOpen}"
        on:click="{() => (sidebarOpen = !sidebarOpen)}"
        title="Table of Contents"
      >
        <Icon icon="material-symbols:format-list-bulleted-rounded" width="20" height="20" />
      </button>
    </div>

    <div class="title" title="{title}">{title || 'EPUB Reader'}</div>

    <div class="cluster">
      <button class="icon-btn" on:click="{toggleTheme}" title="Toggle Theme">
        <Icon icon="{darkIcon ? 'material-symbols:light-mode-outline-rounded' : 'material-symbols:dark-mode-outline-rounded'}" width="20" height="20" />
      </button>
      <button class="icon-btn" on:click="{toggleFullscreen}" title="Fullscreen">
        <Icon icon="material-symbols:fullscreen-rounded" width="20" height="20" />
      </button>
    </div>
  </div>

  <div class="reader-body">
    <!-- TOC Sidebar Drawer -->
    {#if sidebarOpen}
      <div class="sidebar-host">
        <div class="sidebar-header">
          <span>Table of Contents</span>
          <button class="icon-btn" on:click="{() => (sidebarOpen = false)}" title="Close">
            <Icon icon="material-symbols:close-rounded" width="18" height="18" />
          </button>
        </div>
        <ul class="toc-list">
          {#if toc.length === 0}
            <li class="toc-item" style="cursor: default; opacity: 0.6;">No chapters found</li>
          {:else}
            {#each toc as item}
              <li>
                <button
                  class="toc-item"
                  class:active="{activeHref.includes(item.href)}"
                  on:click="{() => navigateTo(item.href)}"
                >
                  {item.label?.trim() || 'Untitled Chapter'}
                </button>
                {#if item.subitems && item.subitems.length > 0}
                  {#each item.subitems as sub}
                    <button
                      class="toc-item toc-subitem"
                      class:active="{activeHref.includes(sub.href)}"
                      on:click="{() => navigateTo(sub.href)}"
                    >
                      {sub.label?.trim() || 'Untitled'}
                    </button>
                  {/each}
                {/if}
              </li>
            {/each}
          {/if}
        </ul>
      </div>
      {#if isMobile}
        <!-- Backdrop scrim on mobile -->
        <div class="scrim" role="button" tabindex="0" on:click="{() => (sidebarOpen = false)}" on:keydown="{(e) => (e.key === 'Escape' || e.key === 'Enter') && (sidebarOpen = false)}"></div>
      {/if}
    {/if}

    <!-- Main Viewer Area -->
    <div class="viewer-wrap">
      {#if loading}
        <div class="state-message">
          <Jumper size="60" color="var(--button-primary, #6366f1)" unit="px" />
          <span>Opening EPUB...</span>
        </div>
      {:else if errorMsg}
        <div class="state-message">
          <Icon icon="material-symbols:error-outline-rounded" width="48" height="48" style="color: var(--color-danger, #ef4444);" />
          <span>{errorMsg}</span>
        </div>
      {/if}

      <!-- Page turn click zones -->
      <div class="nav-zone left" role="button" tabindex="0" on:click="{prev}" on:keydown="{(e) => (e.key === 'Enter' || e.key === ' ') && prev()}" title="Previous Page">
        <div class="nav-zone-btn">
          <Icon icon="material-symbols:chevron-left-rounded" width="24" height="24" />
        </div>
      </div>
      <div class="nav-zone right" role="button" tabindex="0" on:click="{next}" on:keydown="{(e) => (e.key === 'Enter' || e.key === ' ') && next()}" title="Next Page">
        <div class="nav-zone-btn">
          <Icon icon="material-symbols:chevron-right-rounded" width="24" height="24" />
        </div>
      </div>

      <div bind:this="{viewerContainer}" class="viewer-container" style="display: {loading || errorMsg ? 'none' : 'block'};"></div>

      <!-- Floating Bottom Navigation Pill -->
      {#if !loading && !errorMsg}
        <div class="reader-pill">
          <button class="pill-btn" on:click="{prev}" title="Previous Page">
            <Icon icon="material-symbols:chevron-left-rounded" width="20" height="20" />
          </button>
          <div class="pill-label">
            {#if locationsReady && totalLocations > 0}
              <span>Page {currentLocation} / {totalLocations}</span>
              <span class="pill-sub">({progressPercentage}%)</span>
            {:else if totalChapters > 0}
              <span>Chapter {currentChapterIndex + 1} / {totalChapters}</span>
            {:else}
              <span>Reading</span>
            {/if}
          </div>
          <button class="pill-btn" on:click="{next}" title="Next Page">
            <Icon icon="material-symbols:chevron-right-rounded" width="20" height="20" />
          </button>
        </div>
      {/if}
    </div>
  </div>
</div>
