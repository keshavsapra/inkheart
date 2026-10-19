<style>
  .grid {
    width: 100%;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(12rem, 1fr));
    gap: 2rem;
    padding: 2rem 3rem;
    align-items: start;
  }

  @media (--small-viewport) {
    .grid {
      padding: 1rem;
      gap: 1rem;
      grid-template-columns: repeat(auto-fill, minmax(10rem, 1fr));
    }
  }
</style>

<script lang="ts">
  import type { Collection } from '../../../../types/collections'
  import type { LibraryItem } from '../../../../types/library-item'
  import Item from '../library/Item.svelte'
  import { sortModeStore, sortPropStore } from '../../../../stores/options'

  export let collection: Collection

  // Filter out items where current_item is null (deleted files)
  $: validItems = collection.items.filter((item) => item.current_item != null).map((item) => item.current_item!)

  const sortItems = (items: LibraryItem[], order: 'asc' | 'dsc', prop: string) => {
    items.sort((a, b) => {
      if (order === 'asc') {
        return a[prop] < b[prop] ? -1 : a[prop] > b[prop] ? 1 : 0
      } else {
        return b[prop] < a[prop] ? -1 : b[prop] > a[prop] ? 1 : 0
      }
    })
    return items
  }

  // Sort items based on current sort settings
  $: sortedItems = sortItems([...validItems], $sortModeStore, $sortPropStore)
</script>

<div class="grid">
  {#each sortedItems as item (item.id)}
    <Item item="{item}" showCollectionButton="{true}" collectionButtonMode="toggle" />
  {/each}
</div>
