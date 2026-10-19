<style>
  .thumbs {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px 10px;
  }

  .thumb {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;

    cursor: pointer;
  }

  .thumb-card {
    box-sizing: border-box;
    display: flex;
    align-items: center;
    justify-content: center;

    width: 100%;
    aspect-ratio: 3 / 4;

    overflow: hidden;

    background: var(--card-color);
    border: 2px solid transparent;
    border-radius: var(--radius-xs);
  }

  .thumb.current .thumb-card {
    border-color: var(--color-primary-400);
  }

  .thumb-card :global(canvas) {
    display: block;

    max-width: 100%;
    max-height: 100%;
  }

  .thumb-label {
    color: var(--text-color-light);
    font-size: var(--font-xs);
    font-variant-numeric: tabular-nums;
  }

  .thumb.current .thumb-label {
    color: var(--color-primary-500);
  }
</style>

<script lang="ts">
  import { onMount, onDestroy } from 'svelte'

  export let numPages = 0
  export let currentPage = 1
  export let onNavigate: (page: number) => void = () => {}
  export let renderThumb: (pageNumber: number, container: HTMLElement) => void = () => {}

  const EAGER_COUNT = 12

  let rootEl: HTMLDivElement | undefined
  const rendered = new Set<number>()

  const renderCell = (el: HTMLElement) => {
    const pageNumber = Number(el.dataset.page)

    if (!pageNumber || rendered.has(pageNumber)) {
      return
    }

    rendered.add(pageNumber)
    const card = el.querySelector('.thumb-card') as HTMLElement | null

    if (card) {
      renderThumb(pageNumber, card)
    }
  }

  // Created eagerly for use:observe actions
  const observer = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        if (entry.isIntersecting) {
          renderCell(entry.target as HTMLElement)
        }
      }
    },
    { root: null, rootMargin: '200px' },
  )

  $: pages = Array.from({ length: numPages }, (_, i) => i + 1)

  const observe = (node: HTMLElement, pageNumber: number) => {
    node.dataset.page = String(pageNumber)
    observer.observe(node)

    return {
      destroy() {
        observer.unobserve(node)
      },
    }
  }

  const onKey = (event: KeyboardEvent, page: number) => {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault()
      onNavigate(page)
    }
  }

  onMount(() => {
    const cells = rootEl ? Array.from(rootEl.querySelectorAll<HTMLElement>('.thumb')) : []

    for (const cell of cells.slice(0, EAGER_COUNT)) {
      renderCell(cell)
    }
  })

  onDestroy(() => {
    observer.disconnect()
  })
</script>

<div class="thumbs" bind:this="{rootEl}">
  {#each pages as page (page)}
    <div
      class="thumb"
      class:current="{currentPage === page}"
      use:observe="{page}"
      on:click="{() => onNavigate(page)}"
      on:keydown="{(event) => onKey(event, page)}"
      role="button"
      tabindex="0"
      aria-label="Go to page {page}"
      aria-current="{currentPage === page ? 'page' : undefined}"
    >
      <div class="thumb-card"></div>
      <div class="thumb-label">{page}</div>
    </div>
  {/each}
</div>
