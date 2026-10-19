<style>
  .toast {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .message {
    flex: 1;
    min-width: 0;
    font-size: var(--font-sm);
    color: var(--text-color-default);
  }
</style>

<script lang="ts">
  import { toast } from '@zerodevx/svelte-toast'
  import Button from '../components/button/Button.svelte'

  export let toastId: number
  export let message: string
  export let onUndo: () => void | Promise<void>

  const handleUndo = async () => {
    await onUndo()
    toast.pop(toastId)
  }
</script>

<div class="toast">
  <span class="message">{message}</span>
  <Button variant="primary" size="sm" onClick="{handleUndo}">Undo</Button>
</div>
