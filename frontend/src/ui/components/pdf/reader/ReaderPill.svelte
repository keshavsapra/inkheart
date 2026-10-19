<style>
  .reader-pill {
    position: absolute;
    bottom: 20px;
    left: 50%;
    transform: translateX(-50%);
    z-index: 5;
    box-sizing: border-box;
    display: inline-flex;
    align-items: center;
    gap: 2px;
    width: max-content;
    height: 34px;
    padding: 0 6px;
    border: 1px solid var(--card-color);
    border-radius: 999px;
    background: var(--card-subtle);
    box-shadow: var(--elevation-2);
    white-space: nowrap;
    transition: opacity 400ms ease;
  }

  .reader-pill.faded {
    opacity: 0.3;
  }

  .icon-button {
    all: unset;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    color: var(--text-color-light);
    font-size: var(--font-lg);
    cursor: pointer;
    transition: color 120ms ease;
  }

  .icon-button:hover {
    color: var(--text-color-default);
  }

  .icon-button:focus-visible {
    border-radius: var(--radius-sm);
    box-shadow: var(--focus-ring);
  }

  .page-group {
    all: unset;
    display: inline-flex;
    align-items: baseline;
    gap: 0.25rem;
    padding: 0 0.35rem;
    color: var(--text-color-default);
    font-size: var(--font-sm);
    font-variant-numeric: tabular-nums;
    cursor: pointer;
  }

  .page-group:focus-visible {
    border-radius: var(--radius-sm);
    box-shadow: var(--focus-ring);
  }

  .page-current {
    color: var(--text-color-default);
    font-variant-numeric: tabular-nums;
  }

  .page-total {
    color: var(--text-color-light);
    font-size: var(--font-xs);
    font-variant-numeric: tabular-nums;
  }

  .page-input {
    width: 3ch;
    padding: 0;
    border: none;
    background: transparent;
    color: var(--text-color-default);
    font-family: inherit;
    font-size: var(--font-sm);
    font-variant-numeric: tabular-nums;
    text-align: center;
    -moz-appearance: textfield;
    appearance: textfield;
  }

  .page-input::-webkit-outer-spin-button,
  .page-input::-webkit-inner-spin-button {
    margin: 0;
    -webkit-appearance: none;
  }

  .page-input:focus-visible {
    outline: none;
    border-radius: var(--radius-sm);
    box-shadow: var(--focus-ring);
  }

  .divider {
    flex: 0 0 auto;
    width: 1px;
    height: 16px;
    margin: 0 2px;
    background: var(--card-color);
  }

  .zoom-control {
    all: unset;
    display: inline-flex;
    align-items: center;
    gap: 0.15rem;
    padding: 0 0.4rem;
    color: var(--text-color-light);
    font-size: var(--font-sm);
    font-variant-numeric: tabular-nums;
    cursor: pointer;
    transition: color 120ms ease;
  }

  .zoom-control:hover {
    color: var(--text-color-default);
  }

  .zoom-control:focus-visible {
    border-radius: var(--radius-sm);
    box-shadow: var(--focus-ring);
  }

  .zoom-caret {
    display: inline-flex;
    align-items: center;
    font-size: var(--font-base);
  }

  .zoom-menu {
    position: absolute;
    bottom: calc(100% + 8px);
    left: 50%;
    transform: translateX(-50%);
    box-sizing: border-box;
    min-width: 170px;
    padding: 5px;
    border: 1px solid var(--card-color);
    border-radius: var(--radius-md);
    background: var(--card-subtle);
    box-shadow: var(--elevation-2);
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import { onMount, onDestroy, tick } from 'svelte'
  import OptionRow from '../../popover/OptionRow.svelte'

  export let page = 1
  export let numPages = 0
  export let zoomValue = 'auto'
  export let onPrev: () => void = () => {}
  export let onNext: () => void = () => {}
  export let onGotoPage: (n: number) => void = () => {}
  export let onZoomOut: () => void = () => {}
  export let onZoomIn: () => void = () => {}
  export let onSetZoom: (value: string) => void = () => {}

  interface ZoomLevel {
    value: string
    label: string
  }

  const zoomLevels: ZoomLevel[] = [
    { value: 'auto', label: 'Auto' },
    { value: 'page-actual', label: 'Actual size' },
    { value: 'page-fit', label: 'Page fit' },
    { value: 'page-width', label: 'Page width' },
    { value: '0.5', label: '50%' },
    { value: '0.75', label: '75%' },
    { value: '1', label: '100%' },
    { value: '1.25', label: '125%' },
    { value: '1.5', label: '150%' },
    { value: '2', label: '200%' },
    { value: '3', label: '300%' },
    { value: '4', label: '400%' },
  ]

  const IDLE_MS = 1500
  const IDLE_TOUCH_MS = 5000

  let lastPointerType = 'mouse'

  let faded = false
  let idleTimer: ReturnType<typeof setTimeout> | undefined

  let editing = false
  let pageInput: HTMLInputElement | undefined
  let draftPage = ''

  let menuOpen = false

  $: zoomLabel = deriveZoomLabel(zoomValue)

  const deriveZoomLabel = (value: string): string => {
    const match = zoomLevels.find((level) => level.value === value)

    if (match) {
      return match.label
    }

    const numeric = Number(value)

    if (!Number.isNaN(numeric)) {
      return Math.round(numeric * 100) + '%'
    }

    return value
  }

  const clamp = (n: number, min: number, max: number): number => {
    return Math.min(Math.max(n, min), max)
  }

  const resetIdle = () => {
    faded = false

    if (idleTimer) {
      clearTimeout(idleTimer)
    }

    idleTimer = setTimeout(
      () => {
        if (menuOpen || editing) {
          resetIdle()

          return
        }

        faded = true
      },
      lastPointerType === 'touch' ? IDLE_TOUCH_MS : IDLE_MS,
    )
  }

  export function bump() {
    resetIdle()
  }

  const startEditing = async () => {
    resetIdle()
    draftPage = String(page)
    editing = true
    await tick()

    if (pageInput) {
      pageInput.focus()
      pageInput.select()
    }
  }

  const commitEditing = () => {
    const parsed = Number.parseInt(draftPage, 10)

    if (!Number.isNaN(parsed)) {
      onGotoPage(clamp(parsed, 1, numPages))
    }

    editing = false
  }

  const cancelEditing = () => {
    editing = false
  }

  const onInputKeydown = (event: KeyboardEvent) => {
    resetIdle()

    if (event.key === 'Enter') {
      event.preventDefault()
      commitEditing()
    } else if (event.key === 'Escape') {
      event.preventDefault()
      cancelEditing()
    }
  }

  const toggleMenu = () => {
    resetIdle()
    menuOpen = !menuOpen
  }

  const closeMenu = () => {
    menuOpen = false
  }

  const selectZoom = (value: string) => {
    resetIdle()
    onSetZoom(value)
    closeMenu()
  }

  const onPrevClick = () => {
    resetIdle()
    onPrev()
  }

  const onNextClick = () => {
    resetIdle()
    onNext()
  }

  const onZoomOutClick = () => {
    resetIdle()
    onZoomOut()
  }

  const onZoomInClick = () => {
    resetIdle()
    onZoomIn()
  }

  let pillEl: HTMLDivElement | undefined
  let swallowClick = false

  const onPillHover = (event: PointerEvent) => {
    if (event.pointerType === 'touch') {
      return
    }

    resetIdle()
  }

  // wake pill on tap.
  const onPillPointerDown = (event: PointerEvent) => {
    lastPointerType = event.pointerType || 'mouse'

    if (event.pointerType === 'touch' && faded) {
      swallowClick = true
      resetIdle()
    }
  }

  const onPillClick = (event: MouseEvent) => {
    if (swallowClick) {
      swallowClick = false
      event.preventDefault()
      event.stopPropagation()
    }
  }

  const onWindowPointerDown = (event: PointerEvent) => {
    if (!menuOpen) {
      return
    }

    const target = event.target as Node | null

    if (pillEl && target && !pillEl.contains(target)) {
      closeMenu()
    }
  }

  const onWindowKeydown = (event: KeyboardEvent) => {
    if (menuOpen && event.key === 'Escape') {
      event.preventDefault()
      closeMenu()
    }
  }

  onMount(() => {
    resetIdle()
    window.addEventListener('pointerdown', onWindowPointerDown, true)
    window.addEventListener('keydown', onWindowKeydown)
  })

  onDestroy(() => {
    if (idleTimer) {
      clearTimeout(idleTimer)
    }

    window.removeEventListener('pointerdown', onWindowPointerDown, true)
    window.removeEventListener('keydown', onWindowKeydown)
  })
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
<div
  class="reader-pill"
  class:faded="{faded}"
  bind:this="{pillEl}"
  on:pointerenter="{onPillHover}"
  on:pointermove="{onPillHover}"
  on:pointerdown|capture="{onPillPointerDown}"
  on:click|capture="{onPillClick}"
>
  <button class="icon-button" type="button" on:click="{onPrevClick}" aria-label="Previous page">
    <Icon icon="mdi:chevron-left" />
  </button>

  {#if editing}
    <input
      class="page-input"
      type="number"
      bind:this="{pageInput}"
      bind:value="{draftPage}"
      on:keydown="{onInputKeydown}"
      on:blur="{cancelEditing}"
      aria-label="Go to page"
    />
    <span class="page-total">/ {numPages}</span>
  {:else}
    <button class="page-group" type="button" on:click="{startEditing}" aria-label="Go to page">
      <span class="page-current">{page}</span>
      <span class="page-total">/ {numPages}</span>
    </button>
  {/if}

  <button class="icon-button" type="button" on:click="{onNextClick}" aria-label="Next page">
    <Icon icon="mdi:chevron-right" />
  </button>

  <span class="divider" aria-hidden="true"></span>

  <button class="icon-button" type="button" on:click="{onZoomOutClick}" aria-label="Zoom out">
    <Icon icon="mdi:minus" />
  </button>

  <button class="zoom-control" type="button" on:click="{toggleMenu}" aria-haspopup="menu" aria-expanded="{menuOpen}">
    <span class="zoom-value">{zoomLabel}</span>
    <span class="zoom-caret"><Icon icon="mdi:chevron-down" /></span>
  </button>

  <button class="icon-button" type="button" on:click="{onZoomInClick}" aria-label="Zoom in">
    <Icon icon="mdi:plus" />
  </button>

  {#if menuOpen}
    <div class="zoom-menu" role="menu">
      {#each zoomLevels as level (level.value)}
        <OptionRow selected="{level.value === zoomValue}" check="{true}" onSelect="{() => selectZoom(level.value)}">
          {level.label}
        </OptionRow>
      {/each}
    </div>
  {/if}
</div>
