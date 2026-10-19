<style>
  .list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .row {
    box-sizing: border-box;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px;
    border-radius: var(--radius-sm);
    transition: background 120ms ease;
  }

  .row:hover {
    background: var(--card-subtle);
  }

  .clip {
    flex: none;
    display: inline-flex;
    color: var(--text-color-light);
  }

  .filename {
    flex: 1;
    min-width: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    color: var(--text-color-default);
    font-size: var(--font-sm);
  }

  .size {
    flex: none;
    color: var(--text-color-light);
    font-size: var(--font-xs);
    font-variant-numeric: tabular-nums;
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
  import { formatBytes } from '../../../../utils/format'

  export let attachments: { filename: string; size: number }[]
  export let onDownload: (index: number) => void = () => {}
</script>

{#if attachments.length > 0}

  <div class="list">
    {#each attachments as attachment, index (index)}

      <div class="row">
        <span class="clip">
          <Icon icon="mdi:paperclip" width="16" />
        </span>

        <span class="filename" title="{attachment.filename}">{attachment.filename}</span>

        <span class="size">{formatBytes(attachment.size)}</span>

        <button class="icon-btn" type="button" on:click="{() => onDownload(index)}" aria-label="Download {attachment.filename}">
          <Icon icon="mdi:download" width="18" />
        </button>
      </div>

    {/each}
  </div>

{/if}
