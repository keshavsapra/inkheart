<style>
  .grid {
    padding: 2rem;
    /* Grid Fallback */
    display: flex;
    flex-wrap: wrap;

    /* Supports Grid */
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(15rem, 1fr));
    grid-auto-rows: minmax(auto, auto);
    grid-gap: 2rem 2rem;
  }

  @supports (display: grid) {
    .module {
      margin: 0;
    }
  }

  @media (--small-viewport) {
    .grid {
      padding: 1rem 0.5rem 2rem 0.5rem;
      grid-template-columns: repeat(auto-fit, minmax(min(100%/2, max(4rem, 100%/2)), 1fr));
      grid-auto-rows: minmax(auto, auto);
      grid-gap: 2rem 0rem;
    }
  }
</style>

<script lang="ts">
  import type { LibraryItem } from '../../../../types/library-item'
  import Item from './Item.svelte'

  export let items: LibraryItem[] = []
</script>

<div class="grid">
  {#each items as item}
    <Item item="{item}" showCollectionButton="{true}" collectionButtonMode="popover"></Item>
  {/each}
</div>
