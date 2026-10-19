<style>
  main {
    height: 100%;
  }

  @media (--small-viewport) {
    main {
      width: 100vw;
      max-width: 100vw;
    }
  }
</style>

<script lang="ts">
  import CollectionView from '../ui/components/content/collection/CollectionView.svelte'

  export let collectionId = ''
</script>

<main>
  <CollectionView collectionId="{collectionId}" />
</main>
