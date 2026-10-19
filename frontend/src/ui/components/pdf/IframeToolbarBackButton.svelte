<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import LeftChevron from '../../icons/LeftChevron.svelte'

  export let iframe: HTMLIFrameElement
  export let onBack: () => void = () => {}

  let cleanup: (() => void) | null = null

  onMount(() => {
    if (!iframe?.contentDocument) return

    const doc = iframe.contentDocument

    const toolbar: HTMLDivElement | null = doc.querySelector('.toolbar')
    if (!toolbar) return

    const toolbarViewerLeft = toolbar.querySelector('#toolbarViewerLeft')
    if (!toolbarViewerLeft) return

    const toolbarHeight = toolbar.offsetHeight

    // Inject style for `.injector` if it doesn't already exist
    const styleId = 'injector-hover-style'
    if (!doc.getElementById(styleId)) {
      const style = doc.createElement('style')
      style.id = styleId
      style.textContent = `
        .injector {
          display: flex;
          align-items: center;
          margin: 0 8px;
          height: ${toolbarHeight}px;
          width: ${toolbarHeight}px;
        }
        .injector > div {
          border-radius: var(--radius-xs);
          padding: 3px;
          transition: background-color 0.2s ease;
        }
        .injector > div:hover {
          background-color: var(--button-hover-color, #f0f0f0);
        }
      `
      doc.head.appendChild(style)
    }

    const container = doc.createElement('div')
    const innerWrapper = doc.createElement('div')
    container.className = 'injector'
    container.setAttribute('role', 'button')
    container.setAttribute('tabindex', '-1')
    container.setAttribute('title', 'Back to library')
    container.addEventListener('click', onBack)

    container.appendChild(innerWrapper)

    toolbarViewerLeft.insertBefore(container, toolbarViewerLeft.firstChild)

    new LeftChevron({
      target: innerWrapper,
      props: {
        primaryColor: 'var(--main-color)',
        height: '1.4rem',
        width: '1.4rem',
      },
    })

    const mql = window.matchMedia('(max-width: 45rem)')
    const sync = () => {
      container.style.display = mql.matches ? 'none' : ''
    }

    sync()
    mql.addEventListener('change', sync)
    cleanup = () => mql.removeEventListener('change', sync)
  })

  onDestroy(() => {
    cleanup?.()
  })
</script>
