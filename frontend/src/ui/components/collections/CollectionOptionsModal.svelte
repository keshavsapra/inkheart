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
    justify-content: space-between;
    align-items: center;
    gap: 12px;
  }

  .footer-actions {
    display: flex;
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
  import ConfirmDialog from '../modal/ConfirmDialog.svelte'
  import Button from '../button/Button.svelte'
  import IconButton from '../button/IconButton.svelte'
  import Icon from '@iconify/svelte'
  import { renameCollection, deleteCollection } from '../../../services/collection-service'
  import { loadCollections } from '../../../stores/collections'
  import { toast } from '@zerodevx/svelte-toast'
  import { useNavigate } from 'svelte-navigator'

  export let open = false
  export let collectionName = ''
  export let collectionId = ''

  const navigate = useNavigate()

  let newName = ''
  let saving = false
  let showDeleteConfirm = false

  // Initialize name when modal opens
  $: if (open) {
    newName = collectionName
  }

  const handleClose = () => {
    if (!saving) {
      open = false
    }
  }

  const handleSave = async () => {
    if (!newName.trim() || newName === collectionName || saving) return

    try {
      saving = true
      await renameCollection(collectionId, newName.trim())
      await loadCollections()
      toast.push('Collection renamed')
      open = false
    } catch (error) {
      console.error('Failed to rename collection:', error)
      toast.push('Failed to rename collection')
    } finally {
      saving = false
    }
  }

  const handleDeleteClick = () => {
    showDeleteConfirm = true
  }

  const handleConfirmDelete = async () => {
    saving = true
    try {
      await deleteCollection(collectionId)
      toast.push('Collection deleted')
      await loadCollections()
      open = false
      navigate('/') // Navigate to library root after deletion
    } catch (error) {
      console.error('Failed to delete collection:', error)
      toast.push('Failed to delete collection')
    } finally {
      saving = false
    }
  }

  const handleKeydown = (e: KeyboardEvent) => {
    if (e.key === 'Enter' && !saving) {
      e.preventDefault()
      if (newName.trim()) {
        handleSave()
      }
    }
  }
</script>

<Modal bind:open="{open}">
  <div class="body">
    <div class="header">
      <h2 title="{collectionName}">Edit Collection</h2>
      <IconButton onClick="{handleClose}" ariaLabel="Close" disabled="{saving}">
        <Icon icon="mdi:close" width="18" />
      </IconButton>
    </div>

    <div class="content">
      <label for="collection-name">Collection Name</label>
      <input type="text" id="collection-name" bind:value="{newName}" on:keydown="{handleKeydown}" disabled="{saving}" />
    </div>

    <div class="footer">
      <Button variant="destructive" disabled="{saving}" onClick="{handleDeleteClick}">Delete collection</Button>
      <div class="footer-actions">
        <Button variant="secondary" disabled="{saving}" onClick="{handleClose}">Cancel</Button>
        <Button
          variant="primary"
          pending="{saving}"
          disabled="{!newName.trim() || newName === collectionName}"
          onClick="{handleSave}">Save</Button
        >
      </div>
    </div>
  </div>
</Modal>

<ConfirmDialog
  bind:open="{showDeleteConfirm}"
  title="Delete Collection"
  confirmText="Delete"
  cancelText="Cancel"
  onConfirm="{handleConfirmDelete}"
>
  <p>Are you sure you want to delete <strong>"{collectionName}"</strong>?</p>
  <p>This action cannot be undone.</p>
</ConfirmDialog>
