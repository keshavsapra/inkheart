<style>
  .container {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding: 0 0.3rem;
  }

  .progress-text {
    font-size: var(--font-sm);
  }

  button {
    background: none;
    border: none;
    color: var(--text-color-default);
    cursor: pointer;
    font-size: var(--font-md);
    padding: 0;
    margin-left: 0.5rem;
    opacity: 0.7;
  }

  button:hover {
    opacity: 1;
  }
</style>

<script lang="ts">
  import { toast } from '@zerodevx/svelte-toast'

  export let toastId: number
  export let total: number = 0
  export let processed: number = 0

  const dismiss = () => {
    toast.pop(toastId)
  }
</script>

<div class="container">
  <span class="progress-text">
    {#if total > 0}
      Building search index: {processed}/{total}
    {:else}
      Building search index...
    {/if}
  </span>
  <button on:click="{dismiss}">&#x2715;</button>
</div>
