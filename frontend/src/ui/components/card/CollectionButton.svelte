<style>
  .collection-icon {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    cursor: pointer;
    opacity: 0;
    transition:
      opacity 0.2s,
      color 0.2s,
      background 0.2s;
    z-index: 10;
    color: var(--light-2);
    background: rgba(0, 0, 0, 0.5);
    border-radius: var(--radius-full);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
  }

  .collection-icon.in-collection {
    opacity: 1;
    color: var(--button-primary);
    background: rgba(0, 0, 0, 0.3);
  }

  .collection-icon.in-collection:hover {
    color: var(--danger-color);
    background: rgba(0, 0, 0, 0.5);
  }

  :global(.pin-container:hover) .collection-icon {
    opacity: 1;
  }

  .collection-popover-anchor {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    z-index: 10;
  }

  .collection-popover-anchor .collection-icon {
    position: static;
    top: auto;
    right: auto;
  }

  :global(.pin-container:hover) .collection-popover-anchor .collection-icon {
    opacity: 1;
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import { toast } from '@zerodevx/svelte-toast'
  import { addItemToCollection, removeItemFromCollection } from '../../../services/collection-service'
  import { collections, activeCollectionId, loadCollections } from '../../../stores/collections'
  import Popover from '../popover/Popover.svelte'
  import CollectionPopover from './CollectionPopover.svelte'

  export let itemId: string
  export let mode: 'toggle' | 'popover' = 'toggle' // Toggle for collection view, popover for library/search
  export let onShowModal: (() => void) | null = null

  let isHovering = false
  let showPopover = false

  // Check if item is in the active collection (only for toggle mode)
  $: activeCollection = $activeCollectionId ? $collections.find((c) => c.id === $activeCollectionId) : null
  $: isInCollection = mode === 'toggle' ? activeCollection?.items.some((item) => item.id === itemId) || false : false

  // Icon logic
  $: iconToShow =
    mode === 'popover'
      ? 'mdi:plus-circle'
      : isInCollection && isHovering
        ? 'mdi:close-circle'
        : isInCollection
          ? 'mdi:check-circle'
          : 'mdi:plus-circle'

  const handleToggle = async () => {
    if (!$activeCollectionId) return

    try {
      if (isInCollection) {
        await removeItemFromCollection($activeCollectionId, itemId)
        toast.push('Removed from collection')
      } else {
        await addItemToCollection($activeCollectionId, itemId)
        toast.push('Added to collection')
      }

      await loadCollections()
    } catch (error) {
      console.error('Failed to toggle collection item:', error)
      toast.push(`Failed to ${isInCollection ? 'remove from' : 'add to'} collection`)
    }
  }

  const closePopover = () => {
    showPopover = false
  }

  const handleCreateNew = () => {
    if (onShowModal) {
      onShowModal()
    }
  }
</script>

{#if mode === 'popover'}
  <div class="collection-popover-anchor">
    <Popover bind:open="{showPopover}" placement="bottom-end" width="14rem">
      <div
        slot="trigger"
        let:toggle
        class="collection-icon"
        on:click|preventDefault|stopPropagation="{toggle}"
        on:keydown="{(e) => e.key === 'Enter' && toggle(e)}"
        role="button"
        tabindex="0"
        title="Add to collection"
      >
        <Icon icon="{iconToShow}" width="1.25rem" />
      </div>

      <CollectionPopover itemId="{itemId}" onClose="{closePopover}" onCreateNew="{handleCreateNew}" />
    </Popover>
  </div>
{:else}
  <div
    class="collection-icon"
    class:in-collection="{isInCollection}"
    on:click|preventDefault|stopPropagation="{handleToggle}"
    on:keydown="{(e) => e.key === 'Enter' && handleToggle()}"
    on:mouseenter="{() => (isHovering = true)}"
    on:mouseleave="{() => (isHovering = false)}"
    role="button"
    tabindex="0"
    title="{isInCollection ? 'Remove from collection' : 'Add to collection'}"
  >
    <Icon icon="{iconToShow}" width="1.25rem" />
  </div>
{/if}
