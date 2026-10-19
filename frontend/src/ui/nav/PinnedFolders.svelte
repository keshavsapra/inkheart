<style>
  .pinned-section {
    margin-bottom: 1rem;

    display: flex;
    flex-direction: column;
  }

  .pinned-header {
    margin-top: 18px;
    padding: 0;
  }

  .pinned-header summary {
    font-size: var(--font-2xs);
    font-weight: var(--weight-label);
    text-transform: uppercase;
    color: var(--text-color-light);
    padding: 0 10px;
    margin: 0 0 6px;
    letter-spacing: var(--tracking-label);
    display: flex;
    justify-content: space-between;
    align-items: center;

    cursor: pointer;
  }

  .pinned-header summary .chevron {
    display: inline-flex;
    align-items: center;
    color: var(--text-color-light);
    font-size: var(--font-sm);
    transition: transform 0.2s;
  }

  .pinned-header:not([open]) summary .chevron {
    transform: rotate(-90deg);
  }

  .pinned-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 7px 10px;
    border-radius: var(--radius-sm);
    cursor: pointer;
    transition: background-color 0.1s;
    text-decoration: none;
    text-align: left;
    color: var(--text-color-light);
    font-size: var(--font-sm);

    box-sizing: border-box;
    width: 100%;
  }

  .pinned-item.missing {
    color: var(--error-color);
    opacity: 0.7;
    cursor: default;
  }

  .pinned-item.missing .pinned-name {
    text-decoration: line-through;
  }

  .pinned-item.missing :global(.icon) {
    color: var(--error-color);
  }

  .remove-btn {
    padding: 0;
    background: none;
    border: none;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    transition:
      opacity 0.2s,
      color 0.2s;
    flex-shrink: 0;
  }

  .pinned-item:hover .remove-btn {
    opacity: 0.7;
  }

  .remove-btn:hover {
    opacity: 1 !important;
  }

  .pinned-item:not(.missing) .remove-btn:hover {
    color: var(--danger-color);
  }

  .pinned-item.missing .remove-btn {
    color: var(--error-color);
  }

  .pinned-item:not(.missing) .remove-btn {
    color: var(--text-color-light);
  }

  .pinned-item:hover {
    background-color: var(--card-color);
    color: var(--text-color-default);
  }

  .pinned-item.active {
    background-color: var(--foreground);
    color: var(--text-color-default);
  }

  .pinned-name {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .pinned-item :global(.icon) {
    flex-shrink: 0;
    color: var(--color-primary-400);
    font-size: var(--font-base);
  }
</style>

<script lang="ts">
  import { onMount } from 'svelte'
  import { useNavigate, useLocation } from 'svelte-navigator'
  import Icon from '@iconify/svelte'
  import { pinnedFolders, missingPins, loadPins } from '../../stores/pins'
  import { addPin, removeMissingPin, removePin } from '../../services/pin-service'
  import { toast } from '@zerodevx/svelte-toast'
  import { pushUndoToast } from '../../lib/undo-toast'

  const navigate = useNavigate()
  const location = useLocation()

  onMount(() => {
    loadPins()
  })

  const goToFolder = (id: string, isMissing: boolean) => {
    if (isMissing) return
    navigate(`/library/${id}`)
  }

  const restorePin = async (id: string, displayName?: string) => {
    try {
      await addPin(id, displayName || undefined)
      await loadPins()
    } catch (error) {
      console.error('Failed to restore pin:', error)
      toast.push('Failed to restore pin')
    }
  }

  const handleRemove = async (id: string, isMissing: boolean, displayName: string | undefined, event: Event) => {
    event.stopPropagation()

    try {
      if (isMissing) {
        await removeMissingPin(id)
        await loadPins()
        toast.push('Removed missing pin')
        return
      }

      await removePin(id)
      await loadPins()
      pushUndoToast('Folder unpinned', () => restorePin(id, displayName))
    } catch (error) {
      console.error('Failed to remove pin:', error)
      toast.push('Failed to remove pin')
    }
  }

  // Combine all pins into a single list
  $: allPins = [
    ...$pinnedFolders.map((f) => ({ ...f, isMissing: false })),
    ...$missingPins.map((p) => ({ ...p, isMissing: true })),
  ]

  $: currentPath = $location.pathname
  $: isActive = (id: string) => currentPath === `/library/${id}`
</script>

{#if allPins.length > 0}
  <details class="pinned-header" open>
    <summary><span>Pinned</span><span class="chevron"><Icon icon="mdi:chevron-down" /></span></summary>
    <div class="pinned-section">
      {#each allPins as pin}
        <div
          class="pinned-item"
          class:active="{!pin.isMissing && isActive(pin.id)}"
          class:missing="{pin.isMissing}"
          on:click="{() => goToFolder(pin.id, pin.isMissing)}"
          role="button"
          tabindex="0"
          on:keydown="{(e) => e.key === 'Enter' && goToFolder(pin.id, pin.isMissing)}"
          title="{pin.last_seen_path}"
        >
          <Icon icon="mdi:pin-outline" class="icon" />
          <span class="pinned-name">
            {pin.display_name ||
              (!pin.isMissing && 'current_item' in pin ? pin.current_item?.title : null) ||
              'Unknown'}
          </span>
          <button
            class="remove-btn"
            on:click="{(e) => handleRemove(pin.id, pin.isMissing, pin.display_name, e)}"
            title="{pin.isMissing ? 'Remove this pin' : 'Unpin folder'}"
          >
            <Icon icon="mdi:close" width="1rem" />
          </button>
        </div>
      {/each}
    </div>
  </details>
{/if}
