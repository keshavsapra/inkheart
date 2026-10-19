<style>
  .layers-tree {
    display: flex;
    flex-direction: column;
  }

  .group-header {
    box-sizing: border-box;
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 5px 6px;
    border-radius: var(--radius-sm);
    color: var(--text-color-default);
    font-size: var(--font-xs);
    font-style: italic;
    cursor: pointer;
  }

  .group-header:hover {
    background: var(--card-subtle);
  }

  .group-header:focus-visible {
    outline: none;
    box-shadow: var(--focus-ring);
  }

  .toggler {
    all: unset;
    flex: 0 0 auto;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 14px;
    height: 14px;
    color: var(--text-color-light);
    transition: transform 120ms;
  }

  .toggler.open {
    transform: rotate(90deg);
  }

  .layer-row {
    all: unset;
    box-sizing: border-box;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 5px 6px;
    border-radius: var(--radius-sm);
    color: var(--text-color-default);
    font-size: var(--font-xs);
    cursor: pointer;
  }

  .layer-row:hover {
    background: var(--card-subtle);
  }

  .layer-row:focus-visible {
    outline: none;
    box-shadow: var(--focus-ring);
  }

  .layer-box {
    flex: 0 0 auto;
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

  .layer-row.on .layer-box {
    border-color: var(--color-primary-400);
    background: var(--color-primary-400);
  }

  .layer-name {
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import type { LayerNode } from '../../../../lib/reader/layers'

  export let layers: LayerNode[] = []
  export let onToggle: (id: string) => void = () => {}
  export let depth = 0

  let expanded: Record<number, boolean> = {}

  const isOpen = (index: number): boolean => expanded[index] ?? true

  const toggleGroup = (index: number) => {
    expanded = { ...expanded, [index]: !isOpen(index) }
  }

  const onGroupKeydown = (event: KeyboardEvent, index: number) => {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault()
      toggleGroup(index)
    }
  }
</script>

<div class="layers-tree">
  {#each layers as node, index (node.id || node.name + index)}
    {#if node.children}
      <div
        class="group-header"
        style="padding-left: {6 + depth * 14}px"
        role="button"
        tabindex="0"
        aria-expanded="{expanded[index] ?? true}"
        on:click="{() => toggleGroup(index)}"
        on:keydown="{(event) => onGroupKeydown(event, index)}"
      >
        <span class="toggler" class:open="{expanded[index] ?? true}">
          <Icon icon="mdi:chevron-right" width="14" />
        </span>

        <span class="layer-name">{node.name}</span>
      </div>

      {#if expanded[index] ?? true}
        <svelte:self layers="{node.children}" onToggle="{onToggle}" depth="{depth + 1}" />
      {/if}
    {:else if node.id}
      <button
        class="layer-row"
        class:on="{node.visible}"
        style="padding-left: {6 + depth * 14}px"
        type="button"
        role="checkbox"
        aria-checked="{node.visible}"
        on:click="{() => node.id && onToggle(node.id)}"
      >
        <span class="layer-box" aria-hidden="true">
          {#if node.visible}
            <Icon icon="mdi:check" width="11" />
          {/if}
        </span>

        <span class="layer-name">{node.name}</span>
      </button>
    {/if}
  {/each}
</div>
