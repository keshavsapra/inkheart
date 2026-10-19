<style>
  .container {
    height: 100%;
    width: 100%;
  }

  .viewer {
    width: 100%;
    height: 100%;
    border: none;
  }

  @media (--small-viewport) {
    .container {
      width: calc(100% - 0.25rem);
      height: 100%;
    }
  }
</style>

<script lang="ts">
  import { onMount } from 'svelte'
  import { get } from 'svelte/store'
  import { navigate } from 'svelte-navigator'
  import { getMeta } from '../../../utils/meta'
  import { selectedTheme } from '../../../stores/options'
  import { defaultZoomStore } from '../../../stores/reader'
  import { getActiveTheme, updateTheme } from '../../../lib/theme'

  import IframeToolbarBackButton from './IframeToolbarBackButton.svelte'
  import IframeToolbarDarkModeButton from './IframeToolbarDarkModeButton.svelte'

  export let page = 1
  export let file: string
  export let showBackButton = true

  export let previewTitle = 'Inkheart'

  const page_internal = page

  let iframe: HTMLIFrameElement

  let pdfApp: any

  let outline: any = {}

  onMount(() => {
    previewTitle = getPreviewTitle()
  })

  const getPreviewTitle = () => {
    const metaTags = getMeta()

    return metaTags['inkheart:preview-title'] === '$OGTITLE' ? 'Inkheart' : metaTags['inkheart:preview-title']
  }

  interface OutlineItem {
    dest: { num: number; gen: number }[]
    title: string
    items: OutlineItem[]
  }

  const buildOutline = async () => {
    const numPages = pdfApp.pdfViewer.pdfDocument.numPages
    const root: OutlineItem[] = await pdfApp.pdfViewer.pdfDocument.getOutline()
    const map = {}
    for (const item of root) {
      await walkOutline(item, map)
    }

    let currentTitle = previewTitle
    for (let i = 1; i <= numPages; i++) {
      if (map[i] !== undefined) {
        currentTitle = map[i]
      } else {
        map[i] = currentTitle
      }
    }
    outline = map
  }

  const walkOutline = async (item: OutlineItem, acc: any) => {
    try {
      const index = await pdfApp.pdfViewer.pdfDocument.getPageIndex(item.dest[0])
      if (index !== undefined && index !== null && acc[index + 1] === undefined) {
        acc[index + 1] = item.title
      }
    } catch (e) {}

    for (const ch of item.items) {
      await walkOutline(ch, acc)
    }
  }

  const pdfload = () => {
    if (iframe.contentWindow) {
      pdfApp = (iframe.contentWindow as any).PDFViewerApplication
      pdfApp.initializedPromise.then(() => {
        pdfApp.eventBus.on('pagesinit', (e) => {
          //buildOutline()

          // Set initial zoom level from store
          const zoom = get(defaultZoomStore)
          if (zoom) {
            pdfApp.pdfViewer.currentScaleValue = zoom
          }
        })

        pdfApp.eventBus.on('pagechanging', (e) => {
          page = e.pageNumber
          if (outline[page]) {
            previewTitle = outline[page]
          }
        })
      })
    }
  }

  const handleIframeLoad = () => {
    applyTheme()
    injectToolbarControls()
    pdfload()
  }

  // Theme fix

  $: $selectedTheme &&
    (() => {
      applyTheme()
    })

  const injectToolbarControls = () => {
    if (!iframe?.contentDocument) return

    if (showBackButton) {
      new IframeToolbarBackButton({
        target: document.body, // mounting to outside DOM, since it injects internally
        props: {
          iframe,
          onBack: () => {
            navigate(-1)
          },
        },
      })
    }

    new IframeToolbarDarkModeButton({
      target: document.body, // mounting to outside DOM, since it injects internally
      props: {
        iframe,
        onToggle: () => {
          const newTheme = getActiveTheme() === 'dark' ? 'light' : 'dark'
          updateTheme(newTheme)
          applyTheme()
        },
      },
    })
  }

  const applyTheme = () => {
    if (!iframe?.contentDocument) return

    const isDark = getActiveTheme() === 'dark'

    const existingStyle = iframe.contentDocument.getElementById('pdf-theme-override')

    // Only inject new styles if dark mode is requested
    if (isDark) {
      const styleElement = iframe.contentDocument.createElement('style')
      styleElement.id = 'pdf-theme-override'
      styleElement.textContent = `
        :root {
          --main-color: rgba(249, 249, 250, 1) !important;
          --body-bg-color: rgba(42, 42, 46, 1) !important;
          --progressBar-color: rgba(0, 96, 223, 1) !important;
          --progressBar-bg-color: rgba(40, 40, 43, 1) !important;
          --progressBar-blend-color: rgba(20, 68, 133, 1) !important;
          --scrollbar-color: rgba(121, 121, 123, 1) !important;
          --scrollbar-bg-color: rgba(35, 35, 39, 1) !important;
          --toolbar-icon-bg-color: rgba(255, 255, 255, 1) !important;
          --toolbar-icon-hover-bg-color: rgba(255, 255, 255, 1) !important;
          --sidebar-narrow-bg-color: rgba(42, 42, 46, 0.9) !important;
          --sidebar-toolbar-bg-color: rgba(50, 50, 52, 1) !important;
          --toolbar-bg-color: rgba(56, 56, 61, 1) !important;
          --toolbar-border-color: rgba(12, 12, 13, 1) !important;
          --button-hover-color: rgba(102, 102, 103, 1) !important;
          --toggled-btn-color: rgba(255, 255, 255, 1) !important;
          --toggled-btn-bg-color: rgba(0, 0, 0, 0.3) !important;
          --toggled-hover-active-btn-color: rgba(0, 0, 0, 0.4) !important;
          --dropdown-btn-bg-color: rgba(74, 74, 79, 1) !important;
          --separator-color: rgba(0, 0, 0, 0.3) !important;
          --field-color: rgba(250, 250, 250, 1) !important;
          --field-bg-color: rgba(64, 64, 68, 1) !important;
          --field-border-color: rgba(115, 115, 115, 1) !important;
          --treeitem-color: rgba(255, 255, 255, 0.8) !important;
          --treeitem-hover-color: rgba(255, 255, 255, 0.9) !important;
          --treeitem-selected-color: rgba(255, 255, 255, 0.9) !important;
          --treeitem-selected-bg-color: rgba(255, 255, 255, 0.25) !important;
          --sidebaritem-bg-color: rgba(255, 255, 255, 0.15) !important;
          --doorhanger-bg-color: rgba(74, 74, 79, 1) !important;
          --doorhanger-border-color: rgba(39, 39, 43, 1) !important;
          --doorhanger-hover-color: rgba(249, 249, 250, 1) !important;
          --doorhanger-hover-bg-color: rgba(93, 94, 98, 1) !important;
          --doorhanger-separator-color: rgba(92, 92, 97, 1) !important;
          --dialog-button-bg-color: rgba(92, 92, 97, 1) !important;
          --dialog-button-hover-bg-color: rgba(115, 115, 115, 1) !important;
          --loading-icon: url(images/loading-dark.svg) !important;
        }
      `
      iframe.contentDocument.head.appendChild(styleElement)
    } else {
      if (existingStyle) {
        existingStyle.remove()
      }
    }
  }
</script>

<div class="container">
  <iframe
    bind:this="{iframe}"
    on:load="{handleIframeLoad}"
    class="viewer"
    src="static/pdfjs/web/viewer.html?file={file}#page={page_internal}"
    title="viewer"
  >
  </iframe>
</div>
