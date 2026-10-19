<style>
  .body {
    width: 25.5rem;
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
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  label {
    color: var(--text-color-default);
    font-size: var(--font-sm);
    font-weight: var(--weight-bold);
  }

  input[type='text'] {
    width: 100%;
    height: 36px;
    padding: 0 14px;
    background: var(--card-color);
    border: 1px solid var(--card-contrast);
    border-radius: var(--radius-sm);
    color: var(--text-color-default);
    font-size: var(--font-sm);
    box-sizing: border-box;
    transition:
      border-color 0.15s,
      box-shadow 0.15s;
  }

  input[type='text']::placeholder {
    color: var(--text-color-light);
  }

  input[type='text']:focus {
    outline: none;
    border-color: var(--color-primary-400);
  }

  .footer {
    padding: 0 24px 24px;
    display: flex;
    justify-content: flex-end;
    gap: 12px;
  }

  @media (max-width: 768px) {
    .body {
      width: 95vw;
    }
  }
</style>

<script lang="ts">
  import Modal from '../modal/Modal.svelte'
  import Button from '../button/Button.svelte'
  import IconButton from '../button/IconButton.svelte'
  import Icon from '@iconify/svelte'
  import { createCollection } from '../../../services/collection-service'
  import { loadCollections } from '../../../stores/collections'
  import { toast } from '@zerodevx/svelte-toast'

  export let open = false

  let name = ''
  let creating = false

  // Reset name when modal opens
  $: if (open) {
    name = ''
  }

  const handleClose = () => {
    if (!creating) {
      open = false
    }
  }

  const handleCreate = async () => {
    if (!name.trim() || creating) return

    try {
      creating = true
      await createCollection(name.trim())
      await loadCollections()
      toast.push('Collection created')
      open = false
    } catch (error) {
      console.error('Failed to create collection:', error)
      toast.push('Failed to create collection')
    } finally {
      creating = false
    }
  }

  const handleKeydown = (e: KeyboardEvent) => {
    if (e.key === 'Enter' && !creating) {
      e.preventDefault()
      if (name.trim()) {
        handleCreate()
      }
    }
  }
</script>

<Modal bind:open="{open}">
  <div class="body">
    <div class="header">
      <h2>New Collection</h2>
      <IconButton onClick="{handleClose}" ariaLabel="Close" disabled="{creating}">
        <Icon icon="mdi:close" width="18" />
      </IconButton>
    </div>

    <div class="content">
      <label for="collection-name">Collection Name</label>
      <input
        type="text"
        id="collection-name"
        bind:value="{name}"
        on:keydown="{handleKeydown}"
        disabled="{creating}"
        placeholder="Enter collection name..."
      />
    </div>

    <div class="footer">
      <Button variant="secondary" onClick="{handleClose}" disabled="{creating}">Cancel</Button>
      <Button variant="primary" pending="{creating}" disabled="{!name.trim()}" onClick="{handleCreate}">Create</Button>
    </div>
  </div>
</Modal>
