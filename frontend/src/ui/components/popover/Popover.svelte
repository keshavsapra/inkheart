<style>
  .popover-anchor {
    position: relative;
    display: inline-flex;
  }

  .panel {
    position: absolute;
    z-index: 100;
    min-width: 9rem;
    background: var(--card-subtle);
    border: 1px solid var(--card-color);
    border-radius: var(--radius-md);
    box-shadow: var(--elevation-2);
    overflow-y: auto;
    overflow-x: hidden;
  }

  .panel.fixed {
    position: fixed;
  }

  .panel.bottom-start {
    top: calc(100% + 0.35rem);
    left: 0;
  }

  .panel.bottom-end {
    top: calc(100% + 0.35rem);
    right: 0;
  }

  .panel.right-start {
    top: 0;
    left: calc(100% + 0.35rem);
  }

  .header {
    padding: 0.5rem 0.8rem 0.25rem;
    color: var(--text-color-light);
    font-size: var(--font-2xs);
    font-weight: var(--weight-bold);
    letter-spacing: var(--tracking-label);
    text-transform: uppercase;
  }
</style>

<script lang="ts">
  import { tick, onDestroy } from 'svelte'

  export let open = false
  export let placement: 'bottom-start' | 'bottom-end' | 'right-start' = 'bottom-start'
  export let strategy: 'absolute' | 'fixed' = 'absolute'
  export let width = ''

  let anchorElement: HTMLDivElement
  let panelElement: HTMLDivElement
  let triggerElement: HTMLElement | null = null

  const getItems = (): HTMLElement[] => {
    if (!panelElement) return []

    return Array.from(panelElement.querySelectorAll<HTMLElement>('[data-popover-item]'))
  }

  const focusInitial = () => {
    const list = getItems()

    if (list.length === 0) return

    const selected = list.find((el) => el.getAttribute('data-selected') === 'true')
    const target = selected ?? list[0]

    target.focus()
  }

  const positionPanel = () => {
    if (strategy !== 'fixed' || !open || !anchorElement || !panelElement) {
      return
    }

    const anchor = anchorElement.getBoundingClientRect()
    const rootFontSize = parseFloat(getComputedStyle(document.documentElement).fontSize)
    const gap = 0.35 * rootFontSize

    let top = anchor.bottom + gap
    let left = anchor.left

    if (placement === 'bottom-end') {
      left = anchor.right - panelElement.offsetWidth
    } else if (placement === 'right-start') {
      top = anchor.top
      left = anchor.right + gap
    }

    const pw = panelElement.offsetWidth
    const ph = panelElement.offsetHeight

    left = Math.max(gap, Math.min(left, window.innerWidth - pw - gap))
    top = Math.max(gap, Math.min(top, window.innerHeight - ph - gap))

    panelElement.style.top = `${top}px`
    panelElement.style.left = `${left}px`
    panelElement.style.maxHeight = `${window.innerHeight - top - gap}px`
  }

  const reposition = () => positionPanel()

  const onFixedScroll = (event: Event) => {
    if (panelElement && event.target instanceof Node && panelElement.contains(event.target)) {
      return
    }

    close()
  }

  const addFixedListeners = () => {
    window.addEventListener('scroll', onFixedScroll, { capture: true, passive: true })
    window.addEventListener('resize', reposition)
  }

  const removeFixedListeners = () => {
    window.removeEventListener('scroll', onFixedScroll, { capture: true })
    window.removeEventListener('resize', reposition)
  }

  onDestroy(removeFixedListeners)

  const toggle = async (event?: Event) => {
    if (event) {
      triggerElement = event.currentTarget as HTMLElement
    }

    open = !open

    if (open) {
      await tick()
      positionPanel()
      focusInitial()

      if (strategy === 'fixed') {
        addFixedListeners()
      }
    } else {
      removeFixedListeners()
    }
  }

  const close = (refocus = false) => {
    open = false
    removeFixedListeners()

    if (refocus && triggerElement) {
      triggerElement.focus()
    }
  }

  const onWindowClick = (event: MouseEvent) => {
    if (!open) return

    if (anchorElement && !anchorElement.contains(event.target as Node)) {
      close()
    }
  }

  const onWindowKeydown = (event: KeyboardEvent) => {
    if (open && event.key === 'Escape') {
      close(true)
    }
  }

  const onPanelKeydown = (event: KeyboardEvent) => {
    const list = getItems()

    if (list.length === 0) return

    const current = list.indexOf(document.activeElement as HTMLElement)

    if (event.key === 'ArrowDown') {
      event.preventDefault()
      list[(current + 1) % list.length].focus()
    } else if (event.key === 'ArrowUp') {
      event.preventDefault()
      list[(current - 1 + list.length) % list.length].focus()
    } else if (event.key === 'Home') {
      event.preventDefault()
      list[0].focus()
    } else if (event.key === 'End') {
      event.preventDefault()
      list[list.length - 1].focus()
    }
  }
</script>

<div class="popover-anchor" bind:this="{anchorElement}">
  <slot name="trigger" toggle="{toggle}" open="{open}" />

  {#if open}
    <div
      class="panel {strategy === 'fixed' ? 'fixed' : placement}"
      bind:this="{panelElement}"
      style="{width ? `width: ${width}` : ''}"
      role="menu"
      tabindex="-1"
      on:click|preventDefault|stopPropagation
      on:keydown="{onPanelKeydown}"
    >
      {#if $$slots.header}
        <div class="header"><slot name="header" close="{close}" /></div>
      {/if}

      <slot close="{close}" />
    </div>
  {/if}
</div>

<svelte:window on:click="{onWindowClick}" on:keydown="{onWindowKeydown}" />
