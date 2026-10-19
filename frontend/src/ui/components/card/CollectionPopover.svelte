<style>
  .state-icon {
    display: inline-flex;
    color: var(--text-color-light);
  }

  .state-icon.in-collection {
    color: var(--button-primary);
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import { toast } from '@zerodevx/svelte-toast'
  import { collections, loadCollections } from '../../../stores/collections'
  import { addItemToCollection, removeItemFromCollection } from '../../../services/collection-service'
  import OptionRow from '../popover/OptionRow.svelte'

  export let itemId: string
  export let onClose: () => void
  export let onCreateNew: () => void

  // Check if item is already in each collection
  const isInCollection = (collectionId: string) => {
    const collection = $collections.find((c) => c.id === collectionId)

    return collection?.items.some((item) => item.id === itemId) || false
  }

  const handleToggleCollection = async (collectionId: string) => {
    const collection = $collections.find((c) => c.id === collectionId)
    const name = collection?.name || 'collection'

    try {
      if (isInCollection(collectionId)) {
        await removeItemFromCollection(collectionId, itemId)
        toast.push(`Removed from ${name}`)
      } else {
        await addItemToCollection(collectionId, itemId)
        toast.push(`Added to ${name}`)
      }

      await loadCollections()
    } catch (error) {
      console.error('Failed to update collection:', error)
      toast.push('Failed to update collection')
    }
  }

  const handleCreateNew = () => {
    onClose()
    onCreateNew()
  }
</script>

{#each $collections as collection (collection.id)}
  <OptionRow onSelect="{() => handleToggleCollection(collection.id)}">
    <span slot="leading" class="state-icon" class:in-collection="{isInCollection(collection.id)}">
      <Icon icon="{isInCollection(collection.id) ? 'mdi:check' : 'mdi:plus'}" />
    </span>

    {collection.name}
  </OptionRow>
{/each}

<OptionRow selected="{true}" onSelect="{handleCreateNew}">
  <span slot="leading"><Icon icon="mdi:plus" /></span>

  Create new collection
</OptionRow>
