<style>
  .option-row {
    box-sizing: border-box;
    display: flex;
    align-items: center;
    gap: 0.6rem;
    width: 100%;
    padding: 0.55rem 0.8rem;
    border: none;
    border-bottom: 1px solid var(--card-subtle);
    background: transparent;
    color: var(--text-color-default);
    font-family: inherit;
    font-size: var(--font-sm);
    text-align: left;
    cursor: pointer;
    transition: background 120ms ease;
  }

  .option-row:last-child {
    border-bottom: none;
  }

  .option-row:hover {
    background: var(--card-subtle);
  }

  .option-row:focus-visible {
    outline: none;
    box-shadow: inset 0 0 0 2px var(--color-primary-400);
  }

  .option-row.selected {
    color: var(--color-primary-500);
  }

  .label {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .leading,
  .trailing {
    display: inline-flex;
    align-items: center;
    flex: 0 0 auto;
    font-size: var(--font-lg);
  }

  .leading :global(svg),
  .trailing :global(svg) {
    display: block;
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'

  export let selected = false
  export let check = false
  export let onSelect: () => void = () => {}

  const onKeydown = (event: KeyboardEvent) => {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault()
      onSelect()
    }
  }
</script>

<div
  class="option-row"
  class:selected="{selected}"
  data-popover-item
  data-selected="{selected}"
  role="menuitem"
  tabindex="-1"
  on:click|preventDefault|stopPropagation="{onSelect}"
  on:keydown="{onKeydown}"
>
  {#if $$slots.leading}
    <span class="leading"><slot name="leading" /></span>
  {/if}

  <span class="label"><slot /></span>

  {#if $$slots.trailing}
    <span class="trailing"><slot name="trailing" /></span>
  {:else if check && selected}
    <span class="trailing"><Icon icon="mdi:check" /></span>
  {/if}
</div>
