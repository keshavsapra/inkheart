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
  import Button from '../components/button/Button.svelte'

  const reloadPage = () => {
    window.location.reload()
  }
</script>

<div class="toast">
  <span class="message">Indexing finished.</span>
  <Button variant="primary" size="sm" onClick="{reloadPage}">Refresh</Button>
</div>
