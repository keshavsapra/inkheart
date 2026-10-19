<style>
  .sidebar {
    box-sizing: border-box;
    display: flex;
    flex-direction: column;

    width: 232px;
    height: 100%;

    background: var(--background);
    border-right: 1px solid var(--card-color);
  }

  .tabs {
    display: flex;
    align-items: center;
    gap: 2px;

    padding: 6px;

    border-bottom: 1px solid var(--card-color);
  }

  .utility {
    display: flex;
    align-items: center;
    gap: 2px;

    margin-left: auto;
  }

  .body {
    flex: 1;
    min-height: 0;
  }

  .panel {
    box-sizing: border-box;
    height: 100%;

    overflow-y: auto;
    overflow-x: hidden;

    padding: 10px;
  }

  .panel-hidden {
    display: none;
  }

  .panel::-webkit-scrollbar {
    width: 0.5rem;
  }

  .panel::-webkit-scrollbar-track {
    background: transparent;
  }

  .panel::-webkit-scrollbar-thumb {
    background: var(--card-subtle);
    border-radius: var(--radius-sm);
  }

  .panel::-webkit-scrollbar-thumb:hover {
    background: var(--text-color-light);
  }

  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;

    height: 100%;

    padding: 0 18px;

    color: var(--text-color-light);
    font-size: var(--font-xs);
    text-align: center;
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

  .icon-btn.active {
    background: var(--card-color);
    color: var(--text-color-default);
    border-radius: var(--radius-sm);
  }

  .icon-btn:focus-visible {
    border-radius: var(--radius-sm);
    box-shadow: var(--focus-ring);
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import ThumbnailStrip from './ThumbnailStrip.svelte'
  import OutlineTree from './OutlineTree.svelte'
  import AttachmentsList from './AttachmentsList.svelte'
  import LayersTree from './LayersTree.svelte'
  import { tick } from 'svelte'
  import { findCurrentNode, type OutlineNode } from '../../../../lib/reader/outline'
  import type { LayerNode } from '../../../../lib/reader/layers'

  export let outlineNodes: OutlineNode[] = []
  export let currentPage = 1
  export let numPages = 0
  export let attachments: { filename: string; size: number }[] = []
  export let layers: LayerNode[] = []
  export let onNavigateOutline: (node: OutlineNode) => void = () => {}
  export let onNavigatePage: (page: number) => void = () => {}
  export let renderThumb: (pageNumber: number, container: HTMLElement) => void = () => {}
  export let onDownloadAttachment: (index: number) => void = () => {}
  export let onToggleLayer: (id: string) => void = () => {}
  export let onResetLayers: () => void = () => {}
  export let isMobile = false
  export let onClose: () => void = () => {}

  type Tab = 'thumbnails' | 'outline' | 'attachments' | 'layers'

  let activeTab: Tab = 'thumbnails'
  let selectedOutlineNode: OutlineNode | null = null
  let bodyEl: HTMLDivElement | undefined

  $: hasAttachments = attachments.length > 0
  $: hasLayers = layers.length > 0

  const revealCurrent = async () => {
    const node = findCurrentNode(outlineNodes, currentPage)

    if (!node) {
      return
    }

    selectedOutlineNode = node
    await tick()
    bodyEl?.querySelector('[data-active="true"]')?.scrollIntoView({ block: 'center' })
  }
</script>

<div class="sidebar">
  <div class="tabs">
    <button
      class="icon-btn"
      class:active="{activeTab === 'thumbnails'}"
      type="button"
      on:click="{() => (activeTab = 'thumbnails')}"
      aria-label="Thumbnails"
      aria-pressed="{activeTab === 'thumbnails'}"
      title="Thumbnails"
    >
      <Icon icon="mdi:view-grid-outline" width="17" />
    </button>

    <button
      class="icon-btn"
      class:active="{activeTab === 'outline'}"
      type="button"
      on:click="{() => (activeTab = 'outline')}"
      aria-label="Outline"
      aria-pressed="{activeTab === 'outline'}"
      title="Outline"
    >
      <Icon icon="mdi:format-list-bulleted" width="17" />
    </button>

    {#if hasAttachments}
      <button
        class="icon-btn"
        class:active="{activeTab === 'attachments'}"
        type="button"
        on:click="{() => (activeTab = 'attachments')}"
        aria-label="Attachments"
        aria-pressed="{activeTab === 'attachments'}"
        title="Attachments"
      >
        <Icon icon="mdi:paperclip" width="16" />
      </button>
    {/if}

    {#if hasLayers}
      <button
        class="icon-btn"
        class:active="{activeTab === 'layers'}"
        type="button"
        on:click="{() => (activeTab = 'layers')}"
        aria-label="Layers"
        aria-pressed="{activeTab === 'layers'}"
        title="Layers"
      >
        <Icon icon="mdi:layers-outline" width="17" />
      </button>
    {/if}

    <div class="utility">
      {#if activeTab === 'outline'}
        <button
          class="icon-btn"
          type="button"
          on:click="{revealCurrent}"
          aria-label="Reveal current section"
          title="Reveal current section"
        >
          <Icon icon="mdi:image-filter-center-focus" width="16" />
        </button>
      {/if}

      {#if activeTab === 'layers'}
        <button
          class="icon-btn"
          type="button"
          on:click="{onResetLayers}"
          aria-label="Reset layers"
          title="Reset layers"
        >
          <Icon icon="mdi:restore" width="16" />
        </button>
      {/if}

      {#if isMobile}
        <button class="icon-btn" type="button" on:click="{onClose}" aria-label="Close sidebar" title="Close sidebar">
          <Icon icon="mdi:close" width="16" />
        </button>
      {/if}
    </div>
  </div>

  <div class="body" bind:this="{bodyEl}">
    <div class="panel" class:panel-hidden="{activeTab !== 'thumbnails'}">
      <ThumbnailStrip
        numPages="{numPages}"
        currentPage="{currentPage}"
        onNavigate="{onNavigatePage}"
        renderThumb="{renderThumb}"
      />
    </div>

    <div class="panel" class:panel-hidden="{activeTab !== 'outline'}">
      {#if outlineNodes.length > 0}
        <OutlineTree
          nodes="{outlineNodes}"
          selectedNode="{selectedOutlineNode}"
          onNavigate="{onNavigateOutline}"
          onSelect="{(node) => (selectedOutlineNode = node)}"
        />
      {:else}
        <div class="empty">
          <Icon icon="mdi:format-list-bulleted" width="22" />
          <span>This document has no outline.</span>
        </div>
      {/if}
    </div>

    {#if hasAttachments}
      <div class="panel" class:panel-hidden="{activeTab !== 'attachments'}">
        <AttachmentsList attachments="{attachments}" onDownload="{onDownloadAttachment}" />
      </div>
    {/if}

    {#if hasLayers}
      <div class="panel" class:panel-hidden="{activeTab !== 'layers'}">
        <LayersTree layers="{layers}" onToggle="{onToggleLayer}" />
      </div>
    {/if}
  </div>
</div>
