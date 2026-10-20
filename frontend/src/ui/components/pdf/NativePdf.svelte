<style>
  .native-pdf {
    position: relative;

    display: flex;
    flex-direction: column;

    height: 100%;
    width: 100%;

    overflow: hidden;

    text-align: left;

    background: var(--reader-canvas, #525659);
  }

  .reader-body {
    position: relative;

    display: flex;

    flex: 1;
    min-height: 0;
  }

  .reader-bar-wrap.collapsible {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    z-index: 6;

    transform: translateY(0);

    transition: transform 100ms ease;
  }

  .reader-bar-wrap.collapsible.collapsed {
    transform: translateY(-100%);
  }

  .reader-bar-handle {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    z-index: 7;

    display: flex;
    align-items: center;
    justify-content: center;

    max-height: 1.1rem;
    overflow: hidden;

    color: var(--text-color-light);
    font-size: var(--font-md);

    background: var(--background);
    border-bottom: 1px solid var(--card-color);
    cursor: pointer;

    transition: max-height 200ms ease;
  }

  .reader-bar-handle.hidden {
    max-height: 0;
    border-bottom-color: transparent;
  }

  .reader-bar-handle :global(svg) {
    transition: opacity 120ms ease;
  }

  .reader-bar-handle.hidden :global(svg) {
    opacity: 0;
  }

  .sidebar-host {
    flex: 0 0 auto;
    min-height: 0;
  }

  .sidebar-host.drawer {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    z-index: 20;

    box-shadow: 6px 0 20px rgba(0, 0, 0, 0.4);
  }

  .sidebar-host.sidebar-hidden {
    display: none;
  }

  .scrim {
    position: absolute;
    inset: 0;
    z-index: 15;

    background: rgba(0, 0, 0, 0.5);
  }

  .viewer-wrap {
    position: relative;

    flex: 1;
    min-height: 0;
  }

  .viewer-container {
    position: absolute;
    inset: 0;

    /* constrain annotation z-indexes */
    isolation: isolate;

    overflow: auto;

    scrollbar-width: thin;
    scrollbar-color: var(--card-subtle) transparent;
  }

  .viewer-container::-webkit-scrollbar {
    width: 0.5rem;
    height: 0.5rem;
  }

  .viewer-container::-webkit-scrollbar-track {
    background: transparent;
  }

  .viewer-container::-webkit-scrollbar-thumb {
    background: var(--card-subtle);
    border-radius: var(--radius-sm);
  }

  .viewer-container::-webkit-scrollbar-thumb:hover {
    background: var(--text-color-light);
  }

  /* space for the reader bar */
  .native-pdf.collapsible-bar .viewer-container {
    padding-top: 36px;
  }

  .native-pdf.hand-cursor .viewer-container {
    cursor: grab;
    user-select: none;
  }

  .native-pdf.hand-cursor.panning .viewer-container {
    cursor: grabbing;
  }

  .reader-loading {
    position: absolute;
    inset: 0;
    z-index: 4;

    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 20px;

    pointer-events: none;
  }

  .load-track {
    width: min(70%, 340px);
    height: 3px;

    overflow: hidden;

    background: var(--card-color);
    border-radius: 999px;
  }

  .load-fill {
    height: 100%;

    background: var(--color-primary-500);
    border-radius: 999px;

    transition: width 150ms ease;
  }

  .load-fill.indeterminate {
    width: 40%;

    animation: load-indeterminate 1.1s ease-in-out infinite;
  }

  @keyframes load-indeterminate {
    0% {
      margin-left: -40%;
    }

    100% {
      margin-left: 100%;
    }
  }

  .load-error {
    color: var(--text-color-light);
    font-size: var(--font-sm);
  }
</style>

<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { get } from 'svelte/store'
  import { navigate } from 'svelte-navigator'
  import Icon from '@iconify/svelte'
  import * as pdfjsLib from 'pdfjs-dist/build/pdf.mjs'

  // needs to be set on globalThis so that pdfjs-dist can find it when it loads the worker
  ;(globalThis as unknown as { pdfjsLib: typeof pdfjsLib }).pdfjsLib = pdfjsLib

  import workerUrl from 'pdfjs-dist/build/pdf.worker.min.mjs?url'
  import './reader/pdf-viewer.curated.css'

  import ReaderBar from './reader/ReaderBar.svelte'
  import ReaderPill from './reader/ReaderPill.svelte'
  import FindPanel from './reader/FindPanel.svelte'
  import ReaderSidebar from './reader/ReaderSidebar.svelte'
  import ReaderOptionsMenu from './reader/ReaderOptionsMenu.svelte'
  import DocumentPropertiesModal from './reader/DocumentPropertiesModal.svelte'
  import EmbedModal from '../modal/EmbedModal.svelte'
  import { enrichOutline, type OutlineNode } from '../../../lib/reader/outline'
  import { buildLayers, type LayerNode } from '../../../lib/reader/layers'
  import { renderThumbnail } from '../../../lib/reader/thumbnail'
  import { loadDocumentProps, loadFileSize, emptyDocProps } from '../../../lib/reader/document-props'
  import { loadAttachments, type AttachmentData } from '../../../lib/reader/attachments'
  import { grabPan } from '../../../lib/reader/grab-pan'
  import { createPresentation } from '../../../lib/reader/presentation'
  import { triggerDownload } from '../../../utils/download'

  import { usingAuth } from '../../../stores/auth'
  import { getToken } from '../../../lib/localstorage'
  import { defaultZoomStore, pageSpreadModeStore, collapsibleToolbarStore } from '../../../stores/reader'
  import { selectedTheme } from '../../../stores/options'
  import { getActiveTheme, isDarkTheme, updateTheme } from '../../../lib/theme'
  import { routeUrl } from '../../../utils/path'
  import { getMeta } from '../../../utils/meta'
  import { setReaderContext, clearReaderContext } from '../../../stores/reader-context'
  import { createScrollCollapse } from '../../../lib/scroll-collapse'

  export let file = ''
  export let fileId = ''
  export let page = 1
  export let title = ''
  export let mode: 'reader' | 'fullscreen' | 'embed' = 'reader'

  type ViewerModule = typeof import('pdfjs-dist/web/pdf_viewer.mjs')

  let viewerContainer: HTMLDivElement
  let handleEl: HTMLDivElement | undefined
  let pillRef: ReaderPill

  let pdfDocument: import('pdfjs-dist').PDFDocumentProxy | null = null
  let loadingTask: import('pdfjs-dist').PDFDocumentLoadingTask | null = null
  let pdfViewer: InstanceType<ViewerModule['PDFViewer']>
  let linkService: InstanceType<ViewerModule['PDFLinkService']>
  let findController: InstanceType<ViewerModule['PDFFindController']>
  let eventBus: InstanceType<ViewerModule['EventBus']>

  let numPages = 0
  let docTitle = title

  let loading = true
  let loadProgress = 0
  let loadIndeterminate = false
  let loadError = false
  let sidebarOpen = false
  let sidebarMounted = false
  let zoomValue = get(defaultZoomStore) as string

  let findOpen = false
  let findQuery = ''
  let findHighlightAll = true
  let findCaseSensitive = false
  let findEntireWord = false
  let findMatchDiacritics = false
  let matchCurrent = 0
  let matchTotal = 0

  let outlineNodes: OutlineNode[] = []
  let attachmentData: AttachmentData[] = []
  let attachments: { filename: string; size: number }[] = []
  let layers: LayerNode[] = []
  let ocgConfig: any = null

  let SpreadMode: any = null
  let ScrollMode: any = null

  let spreadMode: 'none' | 'odd' | 'even' = get(pageSpreadModeStore)
  let scrollMode: 'page' | 'vertical' | 'horizontal' | 'wrapped' = 'vertical'
  let cursorMode: 'select' | 'hand' = 'select'
  let rotation = 0
  let panning = false
  let embedOpen = false
  let docPropsOpen = false
  let docProps = emptyDocProps()

  let isMobile = false
  let mediaQuery: MediaQueryList | null = null

  const {
    showToolbar,
    register: scrollboxAction,
    reset: revealToolbar,
    suppress: suppressScrollCollapse,
  } = createScrollCollapse()

  $: collapsibleBar = isMobile && $collapsibleToolbarStore

  // The bound page prop seeds the initial location.
  const initialPage = page
  let initialPositionSettled = false

  const settleInitialPosition = () => {
    if (initialPositionSettled || !initialPage || initialPage <= 1 || !pdfViewer) {
      return
    }

    suppressScrollCollapse(1500)
    pdfViewer.scrollPageIntoView({ pageNumber: initialPage })
    nudgeLanding()
  }

  const abandonInitialPosition = () => {
    initialPositionSettled = true
  }

  $: isDark = isDarkTheme($selectedTheme)

  const authHeaders = (): Record<string, string> => (get(usingAuth) ? { Authorization: `Bearer ${getToken()}` } : {})

  const bumpPill = () => pillRef?.bump()

  const handleBack = () => navigate(-1)

  const onMediaChange = (event: MediaQueryListEvent) => {
    isMobile = event.matches
  }

  $: if (mode === 'reader') {
    setReaderContext({ title: docTitle, onBack: handleBack })
  }

  // mount on first open
  $: if (sidebarOpen) {
    sidebarMounted = true
  }

  const toggleSidebar = () => {
    sidebarOpen = !sidebarOpen
  }

  const openFind = () => {
    findOpen = true
  }

  const closeFind = () => {
    findOpen = false
    eventBus?.dispatch('findbarclose', { source: null })
  }

  const dispatchFind = (type: string, findPrevious = false) => {
    suppressScrollCollapse()
    eventBus?.dispatch('find', {
      source: null,
      type,
      query: findQuery,
      caseSensitive: findCaseSensitive,
      entireWord: findEntireWord,
      matchDiacritics: findMatchDiacritics,
      highlightAll: findHighlightAll,
      findPrevious,
    })
  }

  const restartFind = () => dispatchFind('')

  const onFindQuery = (value: string) => {
    findQuery = value
    restartFind()
  }

  const findNext = () => dispatchFind('again', false)

  const findPrev = () => dispatchFind('again', true)

  const toggleHighlightAll = () => {
    findHighlightAll = !findHighlightAll
    dispatchFind('highlightallchange')
  }

  const toggleCaseSensitive = () => {
    findCaseSensitive = !findCaseSensitive
    restartFind()
  }

  const toggleEntireWord = () => {
    findEntireWord = !findEntireWord
    restartFind()
  }

  const toggleMatchDiacritics = () => {
    findMatchDiacritics = !findMatchDiacritics
    restartFind()
  }

  const toggleTheme = () => {
    updateTheme(getActiveTheme() === 'dark' ? 'light' : 'dark')
  }

  const openFullscreen = () => {
    window.open(`${routeUrl(`/fullscreen/${fileId}`)}?page=${page}`, '_blank')
  }

  const breathingPx = () => parseFloat(getComputedStyle(document.documentElement).fontSize) / 2

  // ensure some minor padding.
  const nudgeLanding = () => {
    if (!viewerContainer) {
      return
    }

    const handleOverlay = collapsibleBar && !$showToolbar && handleEl ? handleEl.offsetHeight : 0

    viewerContainer.scrollTop -= handleOverlay + breathingPx()
  }

  const currentPageEl = (): HTMLElement | null =>
    viewerContainer?.querySelector<HTMLElement>(`.page[data-page-number="${pdfViewer?.currentPageNumber}"]`) ?? null

  const centerPageX = (el: HTMLElement) => {
    const max = viewerContainer.scrollWidth - viewerContainer.clientWidth
    const target = el.offsetLeft + el.offsetWidth / 2 - viewerContainer.clientWidth / 2

    viewerContainer.scrollLeft = Math.min(Math.max(target, 0), max)
  }

  // Center when page fits the viewport.
  const settlePageX = (fallbackScrollLeft: number) => {
    const el = currentPageEl()

    if (el && el.offsetWidth <= viewerContainer.clientWidth) {
      centerPageX(el)
    } else {
      viewerContainer.scrollLeft = fallbackScrollLeft
    }
  }

  // override pdf.js horizontal scroll behavior after zooming.
  const centerAfterScale = () => {
    const apply = () => settlePageX(viewerContainer.scrollLeft)

    requestAnimationFrame(() => requestAnimationFrame(apply))
    setTimeout(apply, 120)
  }

  const gotoPage = (n: number) => {
    const target = Math.min(Math.max(1, n), numPages)

    if (!pdfViewer || numPages === 0 || target === page) {
      return
    }

    abandonInitialPosition()

    suppressScrollCollapse()
    const scrollLeft = viewerContainer.scrollLeft
    pdfViewer.currentPageNumber = target
    settlePageX(scrollLeft)
    nudgeLanding()
  }

  const prevPage = () => gotoPage(page - 1)

  const nextPage = () => gotoPage(page + 1)

  const zoomOut = () => pdfViewer?.decreaseScale()

  const zoomIn = () => pdfViewer?.increaseScale()

  const setZoom = (value: string) => {
    if (pdfViewer) {
      pdfViewer.currentScaleValue = value
    }
  }

  const renderThumb = (pageNumber: number, container: HTMLElement) => {
    if (pdfDocument) {
      renderThumbnail(pdfDocument, pageNumber, container)
    }
  }

  const onNavigateOutline = (node: OutlineNode) => {
    if (linkService && node.dest) {
      linkService.goToDestination(node.dest as Parameters<typeof linkService.goToDestination>[0])
    }
  }

  const onDownloadAttachment = (index: number) => {
    const attachment = attachmentData[index]

    if (!attachment) {
      return
    }

    triggerDownload(new Blob([attachment.content]), attachment.filename)
  }

  const applyLayerConfig = (config: any) => {
    pdfViewer.optionalContentConfigPromise = Promise.resolve(config)
    layers = buildLayers(config)
  }

  const onToggleLayer = (id: string) => {
    if (!ocgConfig || !pdfViewer) {
      return
    }

    const group = ocgConfig.getGroup(id)
    ocgConfig.setVisibility(id, !group?.visible)
    applyLayerConfig(ocgConfig)
  }

  const onResetLayers = async () => {
    if (!pdfDocument || !pdfViewer) {
      return
    }

    ocgConfig = await pdfDocument.getOptionalContentConfig()
    applyLayerConfig(ocgConfig)
  }

  const setSpread = (mode: 'none' | 'odd' | 'even') => {
    spreadMode = mode
    pageSpreadModeStore.set(mode)

    if (pdfViewer && SpreadMode) {
      pdfViewer.spreadMode = SpreadMode[mode.toUpperCase()]
    }
  }

  const setScroll = (mode: 'page' | 'vertical' | 'horizontal' | 'wrapped') => {
    scrollMode = mode

    if (pdfViewer && ScrollMode) {
      pdfViewer.scrollMode = ScrollMode[mode.toUpperCase()]
    }
  }

  const setCursor = (mode: 'select' | 'hand') => {
    cursorMode = mode
  }

  const rotateCw = () => {
    rotation = (rotation + 90) % 360

    if (pdfViewer) {
      pdfViewer.pagesRotation = rotation
    }
  }

  const rotateCcw = () => {
    rotation = (rotation + 270) % 360

    if (pdfViewer) {
      pdfViewer.pagesRotation = rotation
    }
  }

  const onDownload = async () => {
    if (!pdfDocument) {
      return
    }

    const data = await pdfDocument.getData()
    triggerDownload(new Blob([data], { type: 'application/pdf' }), `${docTitle || fileId || 'document'}.pdf`)
  }

  const openEmbed = () => {
    embedOpen = true
  }

  const openDocProps = () => {
    docPropsOpen = true
  }

  const presentation = createPresentation({
    getViewer: () => pdfViewer,
    getContainer: () => viewerContainer,
    getScrollModeEnum: () => ScrollMode,
    getSpreadModeEnum: () => SpreadMode,
    getUserScrollMode: () => scrollMode,
    getUserSpreadMode: () => spreadMode,
    getFallbackScale: () => get(defaultZoomStore) as string,
    getPage: () => page,
    getNumPages: () => numPages,
    gotoPage,
  })

  onMount(async () => {
    mediaQuery = window.matchMedia('(max-width: 45rem)')
    isMobile = mediaQuery.matches
    mediaQuery.addEventListener('change', onMediaChange)

    pdfjsLib.GlobalWorkerOptions.workerSrc = workerUrl

    const viewerModule: ViewerModule = await import('pdfjs-dist/web/pdf_viewer.mjs')
    const { EventBus, PDFLinkService, PDFFindController, PDFViewer } = viewerModule
    ScrollMode = viewerModule.ScrollMode
    SpreadMode = viewerModule.SpreadMode

    eventBus = new EventBus()

    class ReaderLinkService extends PDFLinkService {
      async goToDestination(dest: string | unknown[]) {
        suppressScrollCollapse()

        const scrollLeft = viewerContainer.scrollLeft
        await super.goToDestination(dest)
        settlePageX(scrollLeft)
        nudgeLanding()
      }
    }

    linkService = new ReaderLinkService({ eventBus, ignoreDestinationZoom: true })
    findController = new PDFFindController({ eventBus, linkService })

    pdfViewer = new PDFViewer({
      container: viewerContainer,
      eventBus,
      linkService,
      findController,
    })

    linkService.setViewer(pdfViewer)

    eventBus.on('pagesinit', () => {
      pdfViewer.currentScaleValue = get(defaultZoomStore)
      pdfViewer.spreadMode = SpreadMode[spreadMode.toUpperCase()]

      if (initialPage && initialPage > 0) {
        suppressScrollCollapse(1500)
        pdfViewer.currentPageNumber = initialPage
        nudgeLanding()
      }
    })

    eventBus.on('pagerendered', () => {
      loading = false
      settleInitialPosition()
    })

    eventBus.on('pagesloaded', () => {
      settleInitialPosition()
      initialPositionSettled = true

      if ('requestIdleCallback' in window) {
        requestIdleCallback(() => (sidebarMounted = true))
      } else {
        setTimeout(() => (sidebarMounted = true), 2000)
      }
    })

    eventBus.on('pagechanging', (evt: { pageNumber: number }) => {
      page = evt.pageNumber
    })

    eventBus.on('scalechanging', (evt: { scale: number; presetValue?: string }) => {
      zoomValue = evt.presetValue || String(evt.scale)
      bumpPill()
      centerAfterScale()
    })

    eventBus.on('updatefindmatchescount', (evt: { matchesCount: { current: number; total: number } }) => {
      matchCurrent = evt.matchesCount.current
      matchTotal = evt.matchesCount.total
    })

    eventBus.on('updatefindcontrolstate', (evt: { matchesCount?: { current: number; total: number } }) => {
      matchCurrent = evt.matchesCount?.current ?? 0
      matchTotal = evt.matchesCount?.total ?? 0
    })

    loadingTask = pdfjsLib.getDocument({
      url: file,
      httpHeaders: authHeaders(),
      wasmUrl: routeUrl('static/pdfjs-v6/wasm/'),
      cMapUrl: routeUrl('static/pdfjs/web/cmaps/'),
      cMapPacked: true,
      standardFontDataUrl: routeUrl('static/pdfjs/web/standard_fonts/'),
    })

    loadingTask.onProgress = ({ loaded, total }: { loaded: number; total: number }) => {
      if (total > 0) {
        loadProgress = Math.min(1, loaded / total)
      } else {
        loadIndeterminate = true
      }
    }

    try {
      pdfDocument = await loadingTask.promise
    } catch (e) {
      loadError = true
      loading = false

      return
    }

    numPages = pdfDocument.numPages

    pdfViewer.setDocument(pdfDocument)
    linkService.setDocument(pdfDocument, null)

    docProps = await loadDocumentProps(pdfDocument)

    if (!docTitle) {
      const metas = getMeta()
      const previewTitle = metas['nextchapter:preview-title'] || metas['inkheart:preview-title']

      if (previewTitle && previewTitle !== '$OGTITLE') {
        docTitle = previewTitle
      } else if (docProps.title) {
        docTitle = docProps.title
      }
    }

    loadFileSize(pdfDocument)
      .then((size) => {
        docProps = { ...docProps, fileSize: size }
      })
      .catch(() => {})

    const rawOutline = await pdfDocument.getOutline()

    if (rawOutline) {
      outlineNodes = await enrichOutline(pdfDocument, rawOutline)
    }

    attachmentData = await loadAttachments(pdfDocument)
    attachments = attachmentData.map((a) => ({ filename: a.filename, size: a.content?.length ?? 0 }))

    try {
      ocgConfig = await pdfDocument.getOptionalContentConfig()
      layers = buildLayers(ocgConfig)
    } catch (e) {
      // No optional-content layers, or they could not be read.
    }
  })

  onDestroy(() => {
    presentation.destroy()
    loadingTask?.destroy()
    mediaQuery?.removeEventListener('change', onMediaChange)
    clearReaderContext()
  })
</script>

<div
  class="native-pdf"
  class:hand-cursor="{cursorMode === 'hand'}"
  class:panning="{panning}"
  class:collapsible-bar="{collapsibleBar}"
>
  <div class="reader-bar-wrap" class:collapsible="{collapsibleBar}" class:collapsed="{collapsibleBar && !$showToolbar}">
    <ReaderBar
      title="{docTitle}"
      mode="{mode}"
      isMobile="{isMobile}"
      sidebarOpen="{sidebarOpen}"
      isDark="{isDark}"
      onBack="{handleBack}"
      onToggleSidebar="{toggleSidebar}"
      onOpenFind="{openFind}"
      onToggleTheme="{toggleTheme}"
      onFullscreen="{openFullscreen}"
    >
      <ReaderOptionsMenu
        slot="options"
        spreadMode="{spreadMode}"
        scrollMode="{scrollMode}"
        cursorMode="{cursorMode}"
        showFullscreen="{isMobile && mode === 'reader'}"
        showTheme="{isMobile}"
        isDark="{isDark}"
        onToggleTheme="{toggleTheme}"
        onFullscreen="{openFullscreen}"
        onPresentation="{presentation.enter}"
        onDownload="{onDownload}"
        onEmbed="{openEmbed}"
        onSetSpread="{setSpread}"
        onSetScroll="{setScroll}"
        onSetCursor="{setCursor}"
        onRotateCw="{rotateCw}"
        onRotateCcw="{rotateCcw}"
        onDocumentProperties="{openDocProps}"
      />
    </ReaderBar>
  </div>

  {#if collapsibleBar}
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      class="reader-bar-handle"
      class:hidden="{$showToolbar}"
      bind:this="{handleEl}"
      on:click="{revealToolbar}"
      on:keyup="{revealToolbar}"
      role="button"
      tabindex="-1"
      aria-label="Show toolbar"
    >
      <Icon icon="material-symbols-light:drag-handle" />
    </div>
  {/if}

  {#if findOpen}
    <FindPanel
      isMobile="{isMobile}"
      query="{findQuery}"
      matchCurrent="{matchCurrent}"
      matchTotal="{matchTotal}"
      highlightAll="{findHighlightAll}"
      caseSensitive="{findCaseSensitive}"
      entireWord="{findEntireWord}"
      matchDiacritics="{findMatchDiacritics}"
      onQueryChange="{onFindQuery}"
      onNext="{findNext}"
      onPrev="{findPrev}"
      onToggleHighlight="{toggleHighlightAll}"
      onToggleCaseSensitive="{toggleCaseSensitive}"
      onToggleEntireWord="{toggleEntireWord}"
      onToggleMatchDiacritics="{toggleMatchDiacritics}"
      onClose="{closeFind}"
    />
  {/if}

  <div class="reader-body">
    {#if sidebarOpen && isMobile}
      <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
      <div class="scrim" on:click="{toggleSidebar}"></div>
    {/if}

    {#if sidebarMounted}
      <div class="sidebar-host" class:drawer="{isMobile}" class:sidebar-hidden="{!sidebarOpen}">
        <ReaderSidebar
          outlineNodes="{outlineNodes}"
          currentPage="{page}"
          numPages="{numPages}"
          attachments="{attachments}"
          layers="{layers}"
          onNavigateOutline="{onNavigateOutline}"
          onNavigatePage="{gotoPage}"
          renderThumb="{renderThumb}"
          onDownloadAttachment="{onDownloadAttachment}"
          onToggleLayer="{onToggleLayer}"
          onResetLayers="{onResetLayers}"
          isMobile="{isMobile}"
          onClose="{toggleSidebar}"
        />
      </div>
    {/if}

    <div class="viewer-wrap">
      <div
        class="viewer-container"
        bind:this="{viewerContainer}"
        use:scrollboxAction
        use:grabPan="{{ enabled: cursorMode === 'hand', onPanningChange: (value) => (panning = value) }}"
        on:pointerdown="{abandonInitialPosition}"
        on:wheel="{abandonInitialPosition}"
      >
        <div class="pdfViewer"></div>
      </div>

      {#if loading}
        <div class="reader-loading">
          {#if loadError}
            <span class="load-error">Failed to load document</span>
          {:else}
            <div class="load-track">
              {#if loadIndeterminate}
                <div class="load-fill indeterminate"></div>
              {:else}
                <div class="load-fill" style="width: {Math.round(loadProgress * 100)}%"></div>
              {/if}
            </div>
          {/if}
        </div>
      {/if}

      <ReaderPill
        bind:this="{pillRef}"
        page="{page}"
        numPages="{numPages}"
        zoomValue="{zoomValue}"
        onPrev="{prevPage}"
        onNext="{nextPage}"
        onGotoPage="{gotoPage}"
        onZoomOut="{zoomOut}"
        onZoomIn="{zoomIn}"
        onSetZoom="{setZoom}"
      />
    </div>
  </div>

  <DocumentPropertiesModal bind:open="{docPropsOpen}" properties="{docProps}" />

  <EmbedModal
    bind:open="{embedOpen}"
    baseUrl="{`${window.location.origin}${routeUrl(`/embed/${fileId}`)}`}"
    pageNumber="{page}"
    previewText="{docTitle}"
  />
</div>
