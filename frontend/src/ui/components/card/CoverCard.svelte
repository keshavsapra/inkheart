<style>
  .cover-card {
    position: relative;
    width: 15rem;
    aspect-ratio: 3 / 4;
    overflow: hidden;
    background: var(--card-color);
    cursor: pointer;
  }

  .cover-card.small {
    width: 10rem;
  }

  .art {
    position: absolute;
    inset: 0;
  }

  .art img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    object-position: 50% 0;
  }

  .scrim {
    position: absolute;
    left: 0;
    right: 0;
    bottom: 0;
    top: 45%;
    background: linear-gradient(180deg, transparent, rgba(6, 5, 12, 0.35) 42%, rgba(6, 5, 12, 0.96));
    pointer-events: none;
  }

  .caption {
    position: absolute;
    left: 10px;
    right: 10px;
    bottom: 12px;
    z-index: 3;
    pointer-events: none;
  }

  .name {
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
    color: #fff;
    font-size: var(--font-xs);
    font-weight: var(--weight-bold);
    line-height: 1.25;
    text-align: left;
    text-shadow: 0 1px 4px rgba(0, 0, 0, 0.6);
  }

  .subtitle {
    margin-top: 2px;
    color: rgba(255, 255, 255, 0.7);
    font-size: var(--font-2xs);
    text-align: left;
    text-shadow: 0 1px 4px rgba(0, 0, 0, 0.6);
  }

  .progress-meter {
    position: absolute;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 4;
    height: 3px;
    background: rgba(0, 0, 0, 0.45);
  }

  .progress-meter > i {
    display: block;
    height: 100%;
    background: var(--color-primary-400);
  }

  .progress-meter.done > i {
    background: var(--primary-green);
  }

  .cover-card.minimal .scrim {
    inset: 0;
    background: linear-gradient(180deg, rgba(10, 8, 16, 0.15), rgba(10, 8, 16, 0.45) 45%, rgba(10, 8, 16, 0.96));
    opacity: 0;
    transition: opacity 160ms ease;
  }

  .cover-card.minimal:hover .scrim {
    opacity: 1;
  }

  .cover-card.minimal .caption {
    opacity: 0;
    transition: opacity 160ms ease;
  }

  .cover-card.minimal:hover .caption {
    opacity: 1;
  }

  .skeleton {
    position: absolute;
    inset: 0;
    background: linear-gradient(90deg, var(--card-color) 25%, var(--card-contrast) 37%, var(--card-color) 63%);
    background-size: 400% 100%;
    animation: shimmer 1.5s infinite;
  }

  @keyframes shimmer {
    0% {
      background-position: -200% 0;
    }
    100% {
      background-position: 200% 0;
    }
  }

  @media (--small-viewport) {
    .cover-card {
      width: 9rem;
    }

    .cover-card.small {
      width: 7rem;
    }
  }
</style>

<script lang="ts">
  import { onMount } from 'svelte'
  import { loadPageProgress, showProgressOnCoversStore } from '../../../stores/reader'
  import CollectionButton from './CollectionButton.svelte'

  export let src = ''
  export let title = ''
  export let small = false
  export let itemId = ''
  export let pages: number | undefined = undefined
  export let showCollectionButton = false
  export let collectionButtonMode: 'toggle' | 'popover' = 'toggle'
  export let onShowModal: (() => void) | null = null
  export let mode: 'poster' | 'minimal' = 'poster'

  $: savedPage = itemId ? loadPageProgress(itemId) : null
  $: totalPages = pages ?? 0
  $: hasProgress = $showProgressOnCoversStore && savedPage !== null && totalPages > 0
  $: isDone = hasProgress && (savedPage ?? 0) >= totalPages
  $: progressPct = hasProgress ? Math.min(100, Math.round(((savedPage ?? 0) / totalPages) * 100)) : 0

  $: pageLabel = pages === undefined ? '' : hasProgress && !isDone ? `p.${savedPage} / ${totalPages}` : `${totalPages} pages`

  let containerElement: HTMLDivElement
  let isVisible = false
  let loaded = false
  let observer: IntersectionObserver

  onMount(() => {
    if (!containerElement) return

    observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          isVisible = true
          observer.disconnect()
        }
      },
      {
        rootMargin: '100px',
        threshold: 0.1,
      },
    )

    observer.observe(containerElement)

    return () => observer.disconnect()
  })

  const onLoad = () => {
    loaded = true
  }
</script>

<div class="cover-card pin-container" class:minimal="{mode === 'minimal'}" class:small="{small}">
  {#if showCollectionButton && itemId}
    <CollectionButton itemId="{itemId}" mode="{collectionButtonMode}" onShowModal="{onShowModal}" />
  {/if}

  <div class="art" bind:this="{containerElement}">
    {#if !loaded}
      <div class="skeleton"></div>
    {/if}
    {#if isVisible}
      <img src="{src}" alt="{title}" on:load="{onLoad}" style="display: {loaded ? 'block' : 'none'}" />
    {/if}
  </div>

  {#if title}
    <div class="scrim"></div>
    <div class="caption">
      <span class="name">{title}</span>
      {#if pageLabel}
        <div class="subtitle">{pageLabel}</div>
      {/if}
    </div>
  {/if}

  {#if hasProgress}
    <div class="progress-meter" class:done="{isDone}"><i style="width: {progressPct}%"></i></div>
  {/if}
</div>
