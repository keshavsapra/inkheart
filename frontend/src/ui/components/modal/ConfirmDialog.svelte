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
    color: var(--text-color-default);
    font-size: var(--font-sm);
    line-height: 1.55;
  }

  .content :global(p) {
    margin: 0;
    max-width: 56ch;
  }

  .content :global(p + p) {
    margin-top: 12px;
  }

  .content :global(strong) {
    font-weight: var(--weight-bold);
    color: var(--text-color-default);
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
  import Modal from './Modal.svelte'
  import Button from '../button/Button.svelte'
  import IconButton from '../button/IconButton.svelte'
  import Icon from '@iconify/svelte'

  export let open = false
  export let title = 'Confirm'
  export let confirmText = 'Confirm'
  export let cancelText = 'Cancel'
  export let onConfirm: () => void
  export let onCancel: (() => void) | null = null

  const handleConfirm = () => {
    onConfirm()
    open = false
  }

  const handleCancel = () => {
    if (onCancel) {
      onCancel()
    }

    open = false
  }
</script>

<Modal bind:open="{open}">
  <div class="body">
    <div class="header">
      <h2>{title}</h2>
      <IconButton onClick="{handleCancel}" ariaLabel="Close">
        <Icon icon="mdi:close" width="18" />
      </IconButton>
    </div>

    <div class="content">
      <slot />
    </div>

    <div class="footer">
      <Button variant="secondary" onClick="{handleCancel}">{cancelText}</Button>
      <Button variant="destructive-solid" onClick="{handleConfirm}">{confirmText}</Button>
    </div>
  </div>
</Modal>
