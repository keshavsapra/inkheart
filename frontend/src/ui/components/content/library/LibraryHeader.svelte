<style>
  .wrapper {
    position: sticky;
    top: 0;
    z-index: 100;
  }

  .header {
    width: 100%;
    height: 5.5rem;

    display: grid;
    grid-template-columns: 1fr 3fr;
    grid-template-rows: 1fr auto;
    grid-template-areas: 'title main' 'handle handle';

    background: var(--background);
    box-shadow: var(--elevation-1);
  }

  .toolbar-handle {
    display: none;

    max-height: 2rem;
    font-size: var(--font-3xl);

    height: 2rem;

    background: var(--background);
    box-shadow: var(--elevation-1);

    cursor: pointer;

    overflow: hidden;

    transition: max-height 100ms ease 0ms;
  }

  .handle-icon {
    opacity: 1;

    transition: opacity 50ms ease 0ms;
  }

  .header-main {
    grid-area: main;

    display: flex;
    flex-direction: row;

    justify-content: left;
    align-items: center;

    margin-left: 10%;
    margin-right: 3rem;
  }

  .item-title {
    grid-area: title;
    display: flex;
    flex-direction: row;

    align-items: center;

    margin-left: 2rem;
    margin-right: 2rem;

    overflow: hidden;
  }

  .item-title .back-button {
    margin-right: 1rem;
    --btn-height: 1.4rem;
    cursor: pointer;
  }

  .item-title .back-button.hidden {
    visibility: hidden;
  }

  .title-stack {
    display: flex;
    flex-direction: column;
    justify-content: center;

    gap: 0.15rem;

    min-width: 0;
    overflow: hidden;

    text-align: left;
  }

  .title-row {
    display: flex;
    align-items: baseline;

    min-width: 0;
  }

  .crumb-spacer {
    font-size: var(--font-xs);
  }

  .item-title h3 {
    margin: 0;

    min-width: 0;

    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .item-title h3.hidden {
    visibility: hidden;
  }

  .item-count {
    display: none;
  }

  h3:focus {
    border: none;
    outline: none;
  }

  .search {
    width: 100%;
    min-width: 0;
    max-width: 40rem;
    margin-left: -5rem;

    display: flex;
  }

  .buttons {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-left: 1rem;
  }

  .view-options {
    display: flex;
  }

  @media (--medium-viewport) {
    .collapse-shell {
      display: grid;
      grid-template-rows: 1fr;

      transition: grid-template-rows 100ms ease;
    }

    .collapse-shell.hide {
      grid-template-rows: 0fr;
    }

    .header {
      display: flex;
      flex-direction: column;

      height: auto;
      min-height: 0;

      overflow: hidden;
    }

    .item-title {
      margin: 0 2rem;
      padding-top: 1rem;

      align-items: flex-end;
    }

    .title-stack {
      flex: 1;
    }

    .item-title.empty {
      display: none;
    }

    .item-title .back-button {
      margin-right: 0.6rem;
      margin-bottom: 0.15rem;
    }

    .item-title h3 {
      font-size: var(--font-md);
      font-weight: var(--weight-regular);
    }

    .item-count {
      display: block;

      margin-left: auto;
      padding-left: 0.75rem;

      flex-shrink: 0;

      font-size: var(--font-xs);
      color: var(--text-color-light);
    }

    .header-main {
      margin: 0 2rem;
      padding: 1.375rem 0;

      justify-content: space-between;
    }

    .search {
      margin-left: 0rem;
      max-width: none;
    }

    .toolbar-handle {
      display: block;
    }

    .toolbar-handle.hide {
      max-height: 0rem;

      transition: max-height 100ms ease 0ms;
    }

    .toolbar-handle.hide .handle-icon {
      opacity: 0;

      transition: opacity 50ms ease 0ms;
    }
  }

  @media (--small-viewport) {
    .item-title {
      display: none;
    }

    .header-main {
      margin: 0 1rem;
    }

    .hide-small {
      display: none;
    }
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import SearchBox from './search/SearchBox.svelte'
  import SortMenu from './sort/SortMenu.svelte'
  import ViewMenu from './view-options/ViewMenu.svelte'
  import LeftChevron from '../../../icons/LeftChevron.svelte'
  import Breadcrumb from './Breadcrumb.svelte'
  import type { AncestorRecord } from '../../../../types/library-item'

  export let title = ''
  export let currentFolderId: string = ''
  export let ancestors: AncestorRecord[] = []
  export let itemCount: number | null = null

  export let onSearchChange
  export let onClear

  export let sortableProps

  export let showToolbar = true

  export let onHandle = () => {}
  export let onBack = () => {}

  let searchFocused = false

  const onFocus = () => {
    searchFocused = true
  }

  const onBlur = () => {
    setTimeout(() => {
      searchFocused = false
    }, 100)
  }
</script>

<div class="wrapper">
  <div class="collapse-shell" class:hide="{!showToolbar}">
    <div class="header">
      <div class="item-title" class:empty="{title === ''}">
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div
          class="back-button"
          class:hidden="{title === ''}"
          on:click="{onBack}"
          role="button"
          tabindex="-1"
        >
          <LeftChevron primaryColor="var(--text-color-default)" height="var(--btn-height)"></LeftChevron>
        </div>

        <div class="title-stack">
          {#if title !== ''}
            <Breadcrumb ancestors="{ancestors}" />
          {:else}
            <div class="crumb-spacer" aria-hidden="true">&nbsp;</div>
          {/if}

          <div class="title-row">
            <h3 class:hidden="{title === ''}" title="{title || 'Files'}">{title || 'Files'}</h3>

            {#if itemCount !== null}
              <span class="item-count">{itemCount} {itemCount === 1 ? 'item' : 'items'}</span>
            {/if}
          </div>
        </div>
      </div>
      <div class="header-main">
        <div class="search">
          <SearchBox
            currentFolderId="{currentFolderId}"
            currentFolderName="{title}"
            ancestors="{ancestors}"
            onInput="{onSearchChange}"
            onClear="{onClear}"
            onFocus="{onFocus}"
            onBlur="{onBlur}"
          ></SearchBox>
        </div>
        <div class="buttons {searchFocused ? 'hide-small' : ''}">
          <SortMenu sortableProps="{sortableProps}"></SortMenu>
          <div class="view-options"><ViewMenu withHeader="{title !== ''}"></ViewMenu></div>
        </div>
      </div>
    </div>
  </div>
  <div
    class="toolbar-handle"
    class:hide="{showToolbar}"
    on:click="{onHandle}"
    on:keyup="{onHandle}"
    role="button"
    tabindex="-1"
  >
    <div class="handle-icon">
      <Icon icon="material-symbols-light:drag-handle" />
    </div>
  </div>
  <slot />
</div>
