<style>
  .search {
    width: 100%;
    min-width: 0;
    box-sizing: border-box;

    display: flex;
    flex-direction: row;
    align-items: center;

    border: 1px solid var(--card-contrast);
    background: var(--card-subtle);
    padding: 0 0.6rem 0 0.4rem;
    border-radius: var(--radius-xl);
    height: 2.75rem;

    color: var(--text-color-default);

    transition:
      border-color 140ms ease,
      box-shadow 140ms ease;

    position: relative;
  }

  .search:focus-within {
    border-color: var(--color-primary-400);
  }

  .filter-area {
    flex-shrink: 0;
    display: flex;
    align-items: center;
  }

  .scope-anchor {
    min-width: 0;
    display: flex;
    align-items: center;
  }

  .scope-label {
    flex-shrink: 1;
    min-width: 0;

    display: flex;
    align-items: center;
    gap: 0.25rem;

    background: none;
    border: none;
    color: var(--text-color-default);
    font-size: var(--font-sm);
    font-family: inherit;

    padding: 0.3rem 0.5rem;
    margin-left: 0.2rem;
    border-radius: var(--radius-md);

    cursor: pointer;

    transition: background 0.15s;
  }

  .scope-label:hover,
  .scope-label.open {
    background: var(--card-color);
  }

  .scope-label.global {
    color: var(--text-color-light);
  }

  .scope-icon {
    flex-shrink: 0;
    display: none;
    align-items: center;
  }

  .scope-name {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 8rem;
  }

  @media (--small-viewport) {
    .scope-label,
    .divider {
      display: none;
    }

    .scope-icon {
      display: flex;
    }

    .scope-name {
      display: none;
    }

    .search:focus-within .scope-label,
    .search:focus-within .divider,
    .scope-label.has-value,
    .scope-label.has-value + .divider {
      display: flex;
    }
  }

  .scope-caret {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    transition: transform 200ms ease-out;
  }

  .scope-caret.open {
    transform: rotate(180deg);
  }

  .divider {
    flex-shrink: 0;
    width: 1px;
    height: 1.4rem;
    background: var(--card-contrast);
    margin: 0 0.5rem;
  }

  .searchBox {
    flex: 1 1 0;
    min-width: 0;
    width: 0;
    box-sizing: border-box;

    border: none;
    background: transparent;
    padding: 0.5rem 0.5rem;
    outline: none;
    color: var(--text-color-default);

    font-size: var(--font-base);
  }

  .closeIcon {
    flex-shrink: 0;
    opacity: 0;
    z-index: 1;
    margin-left: 0.3rem;

    display: flex;
    flex-direction: row;
    align-items: center;

    font-size: var(--font-lg);

    cursor: pointer;
    pointer-events: none;
  }

  .closeIcon.enabled {
    opacity: 1;

    pointer-events: all;
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import SearchFilter from './SearchFilter.svelte'
  import ScopePopover from './ScopePopover.svelte'
  import Popover from '../../../popover/Popover.svelte'
  import { searchScope } from '../../../../../stores/search-scope'
  import type { AncestorRecord } from '../../../../../types/library-item'

  // Current folder context, passed in by parent so the popover knows what
  // "scope to current folder" means.
  export let currentFolderId: string = ''
  export let currentFolderName: string = ''
  export let ancestors: AncestorRecord[] = []

  export let onInput = () => {}
  export let onClear = () => {}
  export let onFocus = () => {}
  export let onBlur = () => {}

  let value = ''

  $: scopeLabel = $searchScope.type === 'folder' ? $searchScope.folderName : 'All files'
  $: isGlobal = $searchScope.type === 'global'

  const handleClear = () => {
    value = ''
    onClear()
  }
</script>

<div class="search">
  <div class="filter-area">
    <SearchFilter></SearchFilter>
  </div>

  <Popover strategy="fixed" placement="bottom-start" width="12rem" let:close>
    <div slot="trigger" let:toggle let:open class="scope-anchor">
      <button
        class="scope-label {open ? 'open' : ''} {isGlobal ? 'global' : ''} {value.length > 0 ? 'has-value' : ''}"
        on:mousedown="{(e) => e.preventDefault()}"
        on:click="{toggle}"
        title="Search scope: {scopeLabel}"
        type="button"
        aria-haspopup="menu"
        aria-expanded="{open}"
      >
        <span class="scope-icon">
          <Icon icon="{isGlobal ? 'mdi:collection' : 'carbon:folder'}" width="1rem" />
        </span>
        <span class="scope-name">{scopeLabel}</span>
        <span class="scope-caret {open ? 'open' : ''}">
          <Icon icon="mdi:chevron-down" width="1rem" />
        </span>
      </button>

      <div class="divider"></div>
    </div>

    <ScopePopover
      currentFolderId="{currentFolderId || null}"
      currentFolderName="{currentFolderName}"
      ancestors="{ancestors}"
      close="{close}"
    />
  </Popover>

  <input
    bind:value="{value}"
    on:input="{onInput}"
    on:focus="{onFocus}"
    on:blur="{onBlur}"
    class="searchBox"
    type="text"
    placeholder="Search..."
  />

  <div
    class="closeIcon {value.length > 0 ? 'enabled' : ''}"
    on:click="{handleClear}"
    role="button"
    tabindex="-1"
    on:keyup="{handleClear}"
  >
    <Icon icon="ion:close" />
  </div>
</div>
