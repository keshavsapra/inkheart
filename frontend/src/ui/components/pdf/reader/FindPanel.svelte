<style>
  .find-panel {
    position: absolute;
    top: 36px;
    right: 8px;
    z-index: 6;
    box-sizing: border-box;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 7px 10px;
    border: 1px solid var(--card-color);
    border-top: none;
    border-radius: 0 0 var(--radius-md) var(--radius-md);
    background: var(--card-subtle);
    box-shadow: var(--elevation-2);
    white-space: nowrap;
  }

  .find-panel.mobile {
    top: 0;
    left: 0;
    right: 0;

    display: flex;

    border-radius: 0;
    border-top: none;
    border-left: none;
    border-right: none;
  }

  .find-panel.mobile .find-input {
    flex: 1;
    width: auto;
    min-width: 0;
  }

  .find-input {
    all: unset;
    box-sizing: border-box;
    width: 170px;
    padding: 2px 1px;
    border-bottom: 1px solid var(--card-contrast);
    color: var(--text-color-default);
    font-family: inherit;
    font-size: var(--font-sm);
    text-align: left;
  }

  .find-input::placeholder {
    color: var(--text-color-light);
  }

  .find-input:focus-visible {
    border-radius: var(--radius-sm);
    box-shadow: var(--focus-ring);
  }

  .counter {
    color: var(--text-color-light);
    font-size: var(--font-xs);
    font-variant-numeric: tabular-nums;
  }

  .highlight-toggle {
    all: unset;
    box-sizing: border-box;
    display: inline-flex;
    align-items: center;
    gap: 5px;
    color: var(--text-color-light);
    font-size: var(--font-xs);
    cursor: pointer;
    user-select: none;
  }

  .highlight-toggle.on {
    color: var(--text-color-default);
  }

  .highlight-box {
    box-sizing: border-box;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 13px;
    height: 13px;
    border: 1px solid var(--card-contrast);
    border-radius: var(--radius-xs);
    color: var(--text-color-contrast);
    font-size: var(--font-xs);
  }

  .highlight-toggle.on .highlight-box {
    border-color: var(--color-primary-400);
    background: var(--color-primary-400);
  }

  .highlight-toggle:focus-visible {
    outline: none;
    border-radius: var(--radius-sm);
    box-shadow: var(--focus-ring);
  }

  .divider {
    flex: 0 0 auto;
    width: 1px;
    height: 16px;
    background: var(--card-color);
  }

  .icon-btn {
    all: unset;
    box-sizing: border-box;
    width: 28px;
    height: 28px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--text-color-light);
    cursor: pointer;
    transition: color 120ms ease;
  }

  .icon-btn:hover {
    color: var(--text-color-default);
  }

  .icon-btn:focus-visible {
    border-radius: var(--radius-sm);
    box-shadow: var(--focus-ring);
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import { onMount } from 'svelte'
  import Popover from '../../popover/Popover.svelte'
  import OptionRow from '../../popover/OptionRow.svelte'

  export let query = ''
  export let matchCurrent = 0
  export let matchTotal = 0
  export let highlightAll = true
  export let caseSensitive = false
  export let entireWord = false
  export let matchDiacritics = false
  export let onQueryChange: (value: string) => void = () => {}
  export let onNext = () => {}
  export let onPrev = () => {}
  export let onToggleHighlight = () => {}
  export let onToggleCaseSensitive = () => {}
  export let onToggleEntireWord = () => {}
  export let onToggleMatchDiacritics = () => {}
  export let onClose = () => {}
  export let isMobile = false

  let inputEl: HTMLInputElement | undefined

  $: counterText = deriveCounterText(query, matchCurrent, matchTotal)

  const deriveCounterText = (value: string, current: number, total: number): string => {
    if (total > 0) {
      return `${current} / ${total}`
    }

    if (value.length > 0) {
      return 'No results'
    }

    return ''
  }

  const onInput = (event: Event) => {
    const target = event.currentTarget as HTMLInputElement
    onQueryChange(target.value)
  }

  const onKeydown = (event: KeyboardEvent) => {
    if (event.key === 'Enter') {
      event.preventDefault()

      if (event.shiftKey) {
        onPrev()
      } else {
        onNext()
      }
    } else if (event.key === 'Escape') {
      event.preventDefault()
      onClose()
    }
  }

  onMount(() => {
    if (inputEl) {
      inputEl.focus()

      if (query.length > 0) {
        inputEl.select()
      }
    }
  })
</script>

<div class="find-panel" class:mobile="{isMobile}">
  <input
    class="find-input"
    type="text"
    bind:this="{inputEl}"
    value="{query}"
    placeholder="Find in document"
    on:input="{onInput}"
    on:keydown="{onKeydown}"
    aria-label="Find in document"
  />

  <span class="counter">{counterText}</span>

  <button
    class="highlight-toggle"
    class:on="{highlightAll}"
    type="button"
    on:click="{onToggleHighlight}"
    role="checkbox"
    aria-checked="{highlightAll}"
  >
    <span class="highlight-box" aria-hidden="true">
      {#if highlightAll}
        <Icon icon="mdi:check" width="11" />
      {/if}
    </span>
    <span>Highlight all</span>
  </button>

  <Popover strategy="fixed" placement="bottom-end" width="184px">
    <div slot="trigger" let:toggle let:open>
      <button
        class="icon-btn"
        type="button"
        on:click="{toggle}"
        aria-haspopup="menu"
        aria-expanded="{open}"
        aria-label="Search options"
        title="Search options"
      >
        <Icon icon="mdi:tune-variant" width="17" />
      </button>
    </div>

    <OptionRow check="{true}" selected="{caseSensitive}" onSelect="{onToggleCaseSensitive}">Match case</OptionRow>

    <OptionRow check="{true}" selected="{entireWord}" onSelect="{onToggleEntireWord}">Whole words</OptionRow>

    <OptionRow check="{true}" selected="{matchDiacritics}" onSelect="{onToggleMatchDiacritics}">Match diacritics</OptionRow>
  </Popover>

  <span class="divider" aria-hidden="true"></span>

  <button class="icon-btn" type="button" on:click="{onPrev}" aria-label="Previous match">
    <Icon icon="mdi:chevron-up" width="18" />
  </button>

  <button class="icon-btn" type="button" on:click="{onNext}" aria-label="Next match">
    <Icon icon="mdi:chevron-down" width="18" />
  </button>

  <button class="icon-btn" type="button" on:click="{onClose}" aria-label="Close find">
    <Icon icon="mdi:close" width="18" />
  </button>
</div>
