<style>
  .body {
    width: 24rem;
    max-width: 90vw;
    background: var(--card-subtle);
    border: 1px solid var(--card-contrast);
    border-radius: var(--radius-sm);
    box-shadow: var(--elevation-2);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
    text-align: left;
  }

  .header {
    padding: 14px 24px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--card-color);
  }

  h2 {
    margin: 0;
    font-size: var(--font-md);
    font-weight: var(--weight-light);
    color: var(--text-color-default);
  }

  .content {
    padding: 20px 24px 24px;
    display: grid;
    grid-template-columns: 128px 1fr;
    gap: 12px 16px;
    align-items: baseline;

    max-height: 70vh;
    max-height: 70dvh;
    overflow-y: auto;
  }

  .property-label {
    font-size: var(--font-sm);
    color: var(--text-color-light);
  }

  .property-value {
    font-size: var(--font-sm);
    color: var(--text-color-default);
    min-width: 0;
    word-break: break-word;
    font-variant-numeric: tabular-nums;
  }

  .property-value.empty {
    color: var(--text-color-light);
  }

  @media (max-width: 768px) {
    .body {
      width: 90vw;
    }
  }
</style>

<script lang="ts">
  import Modal from '../../modal/Modal.svelte'
  import IconButton from '../../button/IconButton.svelte'
  import Icon from '@iconify/svelte'
  import type { DocProps } from '../../../../lib/reader/document-props'

  export let open = false

  export let properties: DocProps

  $: rows = [
    { label: 'File name', value: properties.fileName },
    { label: 'File size', value: properties.fileSize },
    { label: 'Title', value: properties.title },
    { label: 'Author', value: properties.author },
    { label: 'Subject', value: properties.subject },
    { label: 'Keywords', value: properties.keywords },
    { label: 'Creation Date', value: properties.creationDate },
    { label: 'Modification Date', value: properties.modificationDate },
    { label: 'Creator', value: properties.creator },
    { label: 'PDF Producer', value: properties.producer },
    { label: 'PDF Version', value: properties.pdfVersion },
    { label: 'Page Count', value: properties.pages },
    { label: 'Page Size', value: properties.pageSize },
    { label: 'Fast Web View', value: properties.fastWebView },
  ]

  const handleClose = () => {
    open = false
  }
</script>

<Modal bind:open="{open}">
  <div class="body">
    <div class="header">
      <h2>Document properties</h2>
      <IconButton onClick="{handleClose}" ariaLabel="Close">
        <Icon icon="mdi:close" width="18" />
      </IconButton>
    </div>

    <div class="content">
      {#each rows as row (row.label)}
        <span class="property-label">{row.label}</span>
        {#if row.value}
          <span class="property-value">{row.value}</span>
        {:else}
          <span class="property-value empty">-</span>
        {/if}
      {/each}
    </div>
  </div>
</Modal>
