<style>
  .trigger {
    box-sizing: border-box;
    display: flex;
    align-items: center;
    gap: 0.4rem;
    height: 42px;
    padding: 0 0.75rem;
    border: 1px solid var(--card-contrast);
    border-radius: var(--radius-md);
    background: transparent;
    color: var(--text-color-default);
    transition: background 120ms ease;
  }

  .trigger:hover {
    background: var(--card-color);
  }

  .property-btn {
    all: unset;
    color: var(--text-color-default);
    font-size: var(--font-base);
    cursor: pointer;
  }

  .property-btn:focus-visible {
    border-radius: var(--radius-sm);
    box-shadow: var(--focus-ring);
  }

  .direction {
    all: unset;
    display: flex;
    align-items: center;
    color: var(--text-color-default);
    font-size: var(--font-2xl);
    cursor: pointer;
  }

  .direction:focus-visible {
    border-radius: var(--radius-sm);
    box-shadow: var(--focus-ring);
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import { sortModeStore, sortPropStore } from '../../../../../stores/options'
  import Popover from '../../../popover/Popover.svelte'
  import OptionRow from '../../../popover/OptionRow.svelte'

  interface Prop {
    key: string
    display: string
  }

  export let sortableProps: Prop[] = []

  $: sortMode = $sortModeStore
  $: sortProp = $sortPropStore
  $: activeProp = sortableProps.find((p) => p.key === sortProp)?.display ?? ''

  const toggleOrdering = () => {
    $sortModeStore = sortMode === 'asc' ? 'dsc' : 'asc'
  }

  const setSortProp = (prop: string) => {
    $sortPropStore = prop
  }
</script>

<Popover strategy="fixed" placement="bottom-end" width="9rem" let:close>
  <div slot="trigger" let:toggle let:open class="trigger">
    <button class="property-btn" type="button" on:click="{toggle}" aria-haspopup="menu" aria-expanded="{open}">
      {activeProp}
    </button>
    <button class="direction" type="button" on:click="{toggleOrdering}" aria-label="Toggle sort direction">
      {#if sortMode === 'asc'}
        <Icon icon="prime:sort-amount-up" />
      {:else if sortMode === 'dsc'}
        <Icon icon="prime:sort-amount-down" />
      {/if}
    </button>
  </div>

  {#each sortableProps as prop}
    <OptionRow
      selected="{sortProp === prop.key}"
      check="{true}"
      onSelect="{() => {
        setSortProp(prop.key)
        close()
      }}"
    >
      {prop.display}
    </OptionRow>
  {/each}
</Popover>
