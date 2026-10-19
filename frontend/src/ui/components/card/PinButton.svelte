<style>
  .pin-icon {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    cursor: pointer;
    opacity: 0;
    transition: opacity 0.2s;
    z-index: 10;
    color: var(--card-contrast);
  }

  .pin-icon.pinned {
    opacity: 1;
    color: var(--button-primary);
  }

  :global(.pin-container:hover) .pin-icon {
    opacity: 1;
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import { toast } from '@zerodevx/svelte-toast'
  import { addPin, removePin } from '../../../services/pin-service'
  import { loadPins, pinnedFolders } from '../../../stores/pins'

  export let itemId: string
  export let itemName: string = ''

  $: isPinned = $pinnedFolders.some((p) => p.id === itemId)

  const handleTogglePin = async (event: Event) => {
    event.preventDefault()
    event.stopPropagation()

    try {
      if (isPinned) {
        await removePin(itemId)
        toast.push('Folder unpinned')
      } else {
        await addPin(itemId, itemName)
        toast.push('Folder pinned')
      }
      await loadPins()
    } catch (error) {
      console.error('Failed to toggle pin:', error)
      toast.push(`Failed to ${isPinned ? 'unpin' : 'pin'} folder`)
    }
  }
</script>

<div
  class="pin-icon"
  class:pinned="{isPinned}"
  on:click="{handleTogglePin}"
  on:keydown="{(e) => e.key === 'Enter' && handleTogglePin(e)}"
  role="button"
  tabindex="0"
  title="{isPinned ? 'Unpin folder' : 'Pin folder'}"
>
  <Icon icon="{isPinned ? 'mdi:pin' : 'mdi:pin-outline'}" width="1.25rem" />
</div>
