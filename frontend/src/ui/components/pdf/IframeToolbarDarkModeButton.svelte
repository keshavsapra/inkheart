<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import Icon from '@iconify/svelte'
  import { selectedTheme } from '../../../stores/options'
  import { getActiveTheme } from '../../../lib/theme'

  export let iframe: HTMLIFrameElement
  export let onToggle: () => void = () => {}

  let container: HTMLDivElement
  let innerWrapper: HTMLDivElement
  let styleElement: HTMLStyleElement
  let cleanup: (() => void) | null = null

  const updateButtonColor = () => {
    if (!innerWrapper) return
    const icon = innerWrapper.querySelector('svg')
    if (!icon) return
    icon.style.color = 'var(--main-color)'
  }

  onMount(() => {
    if (!iframe?.contentDocument) return

    const doc = iframe.contentDocument
    const toolbar: HTMLDivElement | null = doc.querySelector('.toolbar')
    if (!toolbar) return

    const toolbarViewerRight = toolbar.querySelector('#toolbarViewerRight')
    if (!toolbarViewerRight) return

    const toolbarHeight = toolbar.offsetHeight
    const buttonSize = Math.floor(toolbarHeight - 5) // smaller square inside toolbar

    const styleId = 'injector-darkmode-style'
    if (!doc.getElementById(styleId)) {
      styleElement = doc.createElement('style')
      styleElement.id = styleId
      styleElement.textContent = `
        .injector-darkmode {
          display: flex;
          align-items: center;
          justify-content: center;
          margin: 0 8px;
          height: ${toolbarHeight}px;
          width: ${toolbarHeight}px;
        }
        .injector-darkmode > div {
          display: flex;
          align-items: center;
          justify-content: center;
          height: ${buttonSize}px;
          width: ${buttonSize}px;
          box-sizing: border-box;
          border-radius: var(--radius-xs);
          padding: 3px;
          transition: background-color 0.2s ease;
        }
        .injector-darkmode > div:hover {
          background-color: var(--button-hover-color, #f0f0f0);
        }
        .injector-darkmode svg {
          width: 1.4rem;
          height: 1.4rem;
        }
      `
      doc.head.appendChild(styleElement)
    }

    container = doc.createElement('div')
    innerWrapper = doc.createElement('div')
    container.className = 'injector-darkmode'
    container.setAttribute('role', 'button')
    container.setAttribute('tabindex', '-1')
    container.setAttribute('title', 'Toggle Dark Mode')
    container.addEventListener('click', onToggle)

    container.appendChild(innerWrapper)
    toolbarViewerRight.insertBefore(container, toolbarViewerRight.firstChild)

    new Icon({
      target: innerWrapper,
      props: {
        icon: 'ic:baseline-dark-mode',
      },
    })

    updateButtonColor()

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
    if (styleElement && styleElement.parentNode) {
      styleElement.parentNode.removeChild(styleElement)
    }

    if (container && container.parentNode) {
      container.parentNode.removeChild(container)
    }
  })
</script>
