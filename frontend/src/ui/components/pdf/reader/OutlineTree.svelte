<style>
  .outline-tree {
    display: flex;
    flex-direction: column;
  }

  .outline-row {
    all: unset;
    box-sizing: border-box;
    display: flex;
    align-items: center;
    gap: 4px;
    width: 100%;
    padding: 4px 6px;
    border-radius: var(--radius-sm);
    color: var(--text-color-default);
    font-size: var(--font-xs);
    cursor: pointer;
  }

  .outline-row:hover {
    background: var(--card-subtle);
  }

  .outline-row.active {
    background: var(--card-color);
  }

  .outline-row.active .outline-title {
    color: var(--color-primary-500);
  }

  .outline-row:focus-visible {
    box-shadow: var(--focus-ring);
  }

  .toggler {
    all: unset;
    flex: 0 0 auto;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    color: var(--text-color-light);
    cursor: pointer;
    transition:
      transform 120ms ease,
      color 120ms ease;
  }

  .toggler:hover {
    color: var(--text-color-default);
  }

  .toggler.expanded {
    transform: rotate(90deg);
  }

  .toggler:focus-visible {
    border-radius: var(--radius-sm);
    box-shadow: var(--focus-ring);
  }

  .toggler-spacer {
    flex: 0 0 auto;
    width: 16px;
    height: 16px;
  }

  .outline-title {
    flex: 1;
    min-width: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .outline-page {
    flex: 0 0 auto;
    color: var(--text-color-light);
    font-size: var(--font-xs);
    font-variant-numeric: tabular-nums;
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import type { OutlineNode } from '../../../../lib/reader/outline'

  export let nodes: OutlineNode[] = []
  export let selectedNode: OutlineNode | null = null
  export let onNavigate: (node: OutlineNode) => void = () => {}
  export let onSelect: (node: OutlineNode) => void = () => {}
  export let depth = 0

  const activateNode = (node: OutlineNode) => {
    onSelect(node)
    onNavigate(node)
  }

  let expanded: boolean[] = []

  $: syncExpanded(nodes)

  const syncExpanded = (list: OutlineNode[]) => {
    if (expanded.length !== list.length) {
      expanded = list.map(() => true)
    }
  }

  const toggle = (index: number) => {
    expanded[index] = !expanded[index]
    expanded = expanded
  }
</script>

<div class="outline-tree">
  {#each nodes as node, index (index)}
    {@const active = node === selectedNode}
    <div
      class="outline-row"
      class:active="{active}"
      data-active="{active}"
      role="button"
      tabindex="0"
      aria-current="{active ? 'true' : undefined}"
      style="padding-left: {6 + depth * 14}px"
      on:click="{() => activateNode(node)}"
      on:keydown="{(event) => {
        if (event.key === 'Enter' || event.key === ' ') {
          event.preventDefault()
          activateNode(node)
        }
      }}"
    >
      {#if node.items && node.items.length > 0}
        <button
          class="toggler"
          class:expanded="{expanded[index]}"
          type="button"
          aria-label="{expanded[index] ? 'Collapse' : 'Expand'}"
          on:click="{(event) => {
            event.stopPropagation()
            toggle(index)
          }}"
        >
          <Icon icon="mdi:chevron-right" width="14" />
        </button>
      {:else}
        <span class="toggler-spacer" aria-hidden="true"></span>
      {/if}

      <span class="outline-title">{node.title}</span>

      {#if node.pageNumber !== null}
        <span class="outline-page">{node.pageNumber}</span>
      {/if}
    </div>

    {#if expanded[index] && node.items && node.items.length > 0}
      <svelte:self
        nodes="{node.items}"
        selectedNode="{selectedNode}"
        onNavigate="{onNavigate}"
        onSelect="{onSelect}"
        depth="{depth + 1}"
      />
    {/if}
  {/each}
</div>
