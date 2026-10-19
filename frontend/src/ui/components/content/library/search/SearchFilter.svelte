<style>
  .button {
    display: flex;
    flex-direction: row;
    align-items: center;

    background: none;
    border: none;
    padding: 0.3rem;
    border-radius: var(--radius-md);

    cursor: pointer;

    transition: background 0.15s;
  }

  .button:hover,
  .button.open {
    background: var(--card-color);
  }

  .icon {
    width: 1.2rem;
    height: 1.2rem;
    font-size: var(--font-lg);
    color: var(--foreground);

    display: flex;
    align-items: center;

    padding: 0.2rem;

    cursor: pointer;
  }

  .icon.active {
    color: var(--text-color-default);
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import type { SearchMode } from '../../../../../stores/options'
  import { searchModeStore } from '../../../../../stores/options'
  import Popover from '../../../popover/Popover.svelte'
  import OptionRow from '../../../popover/OptionRow.svelte'

  $: filterMode = $searchModeStore

  const changeMode = (mode: SearchMode) => {
    if (mode === filterMode) {
      $searchModeStore = 'all'
    } else {
      $searchModeStore = mode
    }
  }
</script>

<Popover strategy="fixed" width="9rem" let:close>
  <button
    slot="trigger"
    let:toggle
    let:open
    title="search filter"
    type="button"
    class="button {open ? 'open' : ''}"
    on:click="{toggle}"
    aria-haspopup="menu"
    aria-expanded="{open}"
  >
    <div class="icon {filterMode !== 'all' ? 'active' : ''}">
      {#if filterMode === 'file'}
        <Icon icon="carbon:document" />
      {:else if filterMode === 'dir'}
        <Icon icon="carbon:folder" />
      {:else}
        <Icon icon="ion:search" />
      {/if}
    </div>
  </button>

  <OptionRow
    selected="{filterMode === 'file'}"
    check="{true}"
    onSelect="{() => {
      changeMode('file')
      close()
    }}"
  >
    <Icon slot="leading" icon="carbon:document" />
    Files
  </OptionRow>

  <OptionRow
    selected="{filterMode === 'dir'}"
    check="{true}"
    onSelect="{() => {
      changeMode('dir')
      close()
    }}"
  >
    <Icon slot="leading" icon="carbon:folder" />
    Folders
  </OptionRow>
</Popover>
