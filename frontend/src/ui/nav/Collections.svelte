<style>
  .collections-section {
    margin-bottom: 1rem;

    display: flex;
    flex-direction: column;
  }

  .collections-header {
    margin-top: 18px;
    padding: 0;
  }

  .collections-header summary {
    font-size: var(--font-2xs);
    font-weight: var(--weight-label);
    text-transform: uppercase;
    color: var(--text-color-light);
    padding: 0 10px;
    margin: 0 0 6px;
    letter-spacing: var(--tracking-label);
    display: flex;
    align-items: center;
    justify-content: space-between;

    cursor: pointer;
  }

  .actions {
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }

  .add-btn {
    padding: 0;
    background: none;
    border: none;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--text-color-light);
    transition: color 0.2s;
    flex-shrink: 0;
    font-size: var(--font-sm);
  }

  .add-btn:hover {
    color: var(--text-color-default);
  }

  .chevron {
    display: inline-flex;
    align-items: center;
    color: var(--text-color-light);
    font-size: var(--font-sm);
    transition: transform 0.2s;
  }

  .collections-header:not([open]) summary .chevron {
    transform: rotate(-90deg);
  }

  .collection-item {
    position: relative;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 7px 10px;
    border-radius: var(--radius-sm);
    cursor: pointer;
    transition: background-color 0.1s;
    text-decoration: none;
    text-align: left;
    color: var(--text-color-light);
    font-size: var(--font-sm);

    box-sizing: border-box;
    width: 100%;
  }

  .collection-item:hover {
    background-color: var(--card-color);
    color: var(--text-color-default);
  }

  .collection-item.active {
    background-color: var(--foreground);
    color: var(--text-color-default);
  }

  .collection-item :global(.icon) {
    flex-shrink: 0;
    color: var(--color-primary-400);
    font-size: var(--font-base);
  }

  .collection-name {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .menu-btn {
    padding: 0;
    background: none;
    border: none;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    transition:
      opacity 0.2s,
      color 0.2s;
    flex-shrink: 0;
    color: var(--text-color-light);
  }

  .collection-item:hover .menu-btn {
    opacity: 0.7;
  }

  .menu-btn:hover {
    opacity: 1 !important;
    color: var(--text-color-default);
  }
</style>

<script lang="ts">
  import { onMount } from 'svelte'
  import { useNavigate, useLocation } from 'svelte-navigator'
  import Icon from '@iconify/svelte'
  import CollectionOptionsModal from '../components/collections/CollectionOptionsModal.svelte'
  import NewCollectionModal from '../components/collections/NewCollectionModal.svelte'
  import { collections, loadCollections } from '../../stores/collections'

  const navigate = useNavigate()
  const location = useLocation()

  // Modal state
  let optionsModalOpen = false
  let newCollectionModalOpen = false
  let selectedCollectionId = ''
  let selectedCollectionName = ''

  onMount(() => {
    loadCollections()
  })

  const goToCollection = (id: string) => {
    navigate(`/collection/${id}`)
  }

  const handleMenuClick = (id: string, name: string, event: Event) => {
    event.stopPropagation()
    selectedCollectionId = id
    selectedCollectionName = name
    optionsModalOpen = true
  }

  const handleAddClick = (event: Event) => {
    event.stopPropagation()
    event.preventDefault()
    newCollectionModalOpen = true
  }

  $: currentPath = $location.pathname
  $: isActive = (id: string) => currentPath === `/collection/${id}`
</script>

<details class="collections-header" open>
  <summary>
    <span>Collections</span>
    <span class="actions">
      <button class="add-btn" on:click="{handleAddClick}" title="New collection">
        <Icon icon="mdi:plus" />
      </button>
      <span class="chevron"><Icon icon="mdi:chevron-down" /></span>
    </span>
  </summary>
  {#if $collections.length > 0}
    <div class="collections-section">
      {#each $collections as collection}
        <div
          class="collection-item"
          class:active="{isActive(collection.id)}"
          on:click="{() => goToCollection(collection.id)}"
          role="button"
          tabindex="0"
          on:keydown="{(e) => e.key === 'Enter' && goToCollection(collection.id)}"
          title="{collection.name}"
        >
          <Icon icon="mdi:layers-outline" class="icon" />
          <span class="collection-name">
            {collection.name}
          </span>
          <button
            class="menu-btn"
            on:click="{(e) => handleMenuClick(collection.id, collection.name, e)}"
            title="Collection options"
          >
            <Icon icon="mdi:dots-horizontal" width="1.2rem" />
          </button>
        </div>
      {/each}
    </div>
  {/if}
</details>

<CollectionOptionsModal
  bind:open="{optionsModalOpen}"
  collectionId="{selectedCollectionId}"
  collectionName="{selectedCollectionName}"
/>
<NewCollectionModal bind:open="{newCollectionModalOpen}" />
