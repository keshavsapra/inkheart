<style>
  main {
    height: 100%;
    /*max-width: calc(100vw - 5rem);*/
  }

  @media (--small-viewport) {
    main {
      width: 100vw;
      max-width: 100vw;
    }
  }
</style>

<script lang="ts">
  import Library from '../ui/components/content/library/Library.svelte'

  export let directory = ''
</script>

<main>
  <Library directory="{directory}" />
</main>
