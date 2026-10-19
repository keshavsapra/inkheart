<style>
  .wrapper {
    width: 100%;
  }

  .inner-wrapper {
    display: grid;
    align-items: start;
    grid-template-columns: repeat(auto-fill, minmax(12rem, 1fr));

    grid-gap: 2rem 0rem;
  }

  .with-content {
    padding: 2rem 4rem;
  }

  .result-item {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .ribbon-anchor {
    position: relative;
    width: calc(10rem + 10px);
  }

  .ribbon {
    position: absolute;
    top: 0;
    left: 5px;
    background: var(--button-primary, #6f5dc4);
    color: white;
    padding: 0.1rem 0.4rem 0.1rem 0.35rem;
    font-size: var(--font-2xs);
    font-weight: var(--weight-bold);
    border-bottom-right-radius: var(--radius-sm);
    box-shadow: 1px 1px 3px rgba(0, 0, 0, 0.3);
    z-index: 1;
    cursor: default;
    display: flex;
    align-items: center;
  }

  .ribbon .count {
    margin-left: 0.3rem;
    padding-left: 0.3rem;
    border-left: 1px solid rgba(255, 255, 255, 0.4);
    opacity: 0.85;
    font-weight: var(--weight-body);
  }

  @media (--small-viewport) {
    .inner-wrapper {
      grid-template-columns: repeat(auto-fill, minmax(8rem, 1fr));

      grid-gap: 2rem 0rem;
    }

    .with-content {
      padding: 2rem 1rem;
    }

    .ribbon-anchor {
      width: calc(7rem + 10px);
    }

    .ribbon {
      font-size: var(--font-2xs);
      padding: 0.1rem 0.35rem 0.1rem 0.3rem;
    }
  }
</style>

<script lang="ts">
  import type { SearchResultItem } from '../../../../../types/library-item'
  import Item from '../Item.svelte'
  import { activeCollectionId } from '../../../../../stores/collections'

  export let results: SearchResultItem[] = []

  // Use toggle mode when in collection, popover mode otherwise
  let buttonMode: 'toggle' | 'popover'
  $: buttonMode = $activeCollectionId ? 'toggle' : 'popover'
</script>

<div class="wrapper">
  <div class="inner-wrapper {results.length > 0 ? 'with-content' : ''}">
    {#each results as item}
      <div class="result-item">
        <div class="ribbon-anchor">
          {#if item.search_source === 'content' && item.page_num !== undefined}
            <span
              class="ribbon"
              title="Best match: page {item.page_num}{item.match_count !== undefined && item.match_count > 1
                ? `, ${item.match_count} total pages matched`
                : ''}"
            >
              p. {item.page_num}
              {#if item.match_count !== undefined && item.match_count > 1}
                <span class="count">{item.match_count}</span>
              {/if}
            </span>
          {/if}

          <Item
            item="{item}"
            small="{true}"
            showCollectionButton="{true}"
            collectionButtonMode="{buttonMode}"
            pageNum="{item.page_num}"
            mode="poster"
          />
        </div>
      </div>
    {/each}
  </div>
</div>
