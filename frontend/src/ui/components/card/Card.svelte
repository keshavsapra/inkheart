<style>
  .container {
    padding: 2rem;
  }

  .no-padding {
    padding: 0 0 0 0;
  }

  @media (--small-viewport) {
    .container {
    }
  }
</style>

<script lang="ts">
  export let padding: boolean = true
</script>

<div class="container card" class:no-padding="{!padding}">
  <slot />
</div>
