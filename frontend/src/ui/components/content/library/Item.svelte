<style>
  .module {
    /* Demo-Specific Styles */
    display: flex;
    align-items: center;
    justify-content: center;

    /* Flex Fallback */
    margin-left: 5px;
    margin-right: 5px;
    flex: 1 1 200px;
  }

  .module.small {
    flex: 1 1;
  }

  @media (--small-viewport) {
    .module {
      flex: 1 1;
    }
  }
</style>

<script lang="ts">
  import { Link, link } from 'svelte-navigator'
  import type { LibraryItem } from '../../../../types/library-item'
  import { documentCardModeStore } from '../../../../stores/reader'
  import CoverCard from '../../card/CoverCard.svelte'
  import FolderCard from '../../card/FolderCard.svelte'
  import NewCollectionModal from '../../collections/NewCollectionModal.svelte'

  export let item: LibraryItem

  export let small: boolean = false
  export let showCollectionButton: boolean = false
  export let collectionButtonMode: 'toggle' | 'popover' = 'popover'
  export let pageNum: number | undefined = undefined
  export let mode: 'poster' | 'minimal' | null = null

  $: cardMode = mode ?? $documentCardModeStore

  $: readerLink = pageNum !== undefined
    ? `/reader/${item.id}?page=${pageNum}`
    : `/reader/${item.id}`

  let showNewCollectionModal = false

  const handleShowModal = () => {
    showNewCollectionModal = true
  }
</script>

<div class="module" class:small="{small}">
  {#if item.is_dir}
    <Link to="{`/library/${item.id}`}">
      <FolderCard small="{small}" title="{item.title}" itemId="{item.id}" />
    </Link>
  {:else}
    <Link to="{readerLink}" state="{{ previewTitle: item.title, filename: item.filename }}">
      <CoverCard
        small="{small}"
        src="{`cover/${item.cover_name}`}"
        title="{item.title}"
        itemId="{item.id}"
        pages="{item.pages}"
        showCollectionButton="{showCollectionButton}"
        collectionButtonMode="{collectionButtonMode}"
        onShowModal="{collectionButtonMode === 'popover' ? handleShowModal : null}"
        mode="{cardMode}"
      />
    </Link>
  {/if}
</div>

<NewCollectionModal bind:open="{showNewCollectionModal}" />
