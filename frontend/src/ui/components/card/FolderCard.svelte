<style>
  .folder {
    box-sizing: border-box;
    width: 15rem;
    aspect-ratio: 1.15 / 1;
    padding: 1rem;
    background: var(--card-color);
    border-radius: var(--radius-sm);
    position: relative;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    cursor: pointer;
    transition: background 0.12s ease;
  }

  .folder:hover {
    background: color-mix(in oklab, var(--card-color) 88%, var(--card-contrast));
  }

  .icon {
    flex: 1;
    min-height: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--dark-5);
  }

  .name {
    color: var(--text-color-default);
    font-size: var(--font-sm);
    text-align: left;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .folder.small {
    width: 10rem;
    padding: 0.75rem;
  }

  .folder.small .name {
    font-size: var(--font-xs);
  }

  @media (--small-viewport) {
    .folder {
      width: 9rem;
    }

    .folder.small {
      width: 7rem;
    }
  }
</style>

<script lang="ts">
  import Folder from '../../icons/Folder.svelte'
  import PinButton from './PinButton.svelte'

  export let title = ''
  export let small = false
  export let itemId = ''
</script>

<div class="folder pin-container" class:small="{small}">
  {#if itemId}
    <PinButton itemId="{itemId}" itemName="{title}" />
  {/if}

  <div class="icon">
    <Folder width="50%" height="auto" />
  </div>

  {#if title}
    <div class="name">{title}</div>
  {/if}
</div>
