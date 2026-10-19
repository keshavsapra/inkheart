<style>
  .lifecycle-icon {
    position: relative;
    display: flex;
    align-items: center;
    width: 1.4rem;
    height: 1.4rem;
    color: var(--button-secondary);
  }

  .icon-wrapper {
    position: absolute;
    top: 0;
    left: 0;
    display: flex;
    align-items: center;
  }

  .checkmark-icon {
    color: var(--button-primary);
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import { startIndexer } from '../../../../../services/indexer-service'
  import { IndexerWatcher } from '../../../../../services/indexer-watcher'
  import { onMount, onDestroy } from 'svelte'
  import { Circle } from 'svelte-loading-spinners'
  import { fade } from 'svelte/transition'
  import { get } from 'svelte/store'
  import OptionRow from '../../../popover/OptionRow.svelte'

  let indexerRunning = false
  let showCheckmark = false
  let clickable = true
  let checkmarkTimeout: ReturnType<typeof setTimeout> | null = null
  let unsubscribe = () => {}

  // Helper to subscribe watcher with 500ms delay (avoid race condition)
  const subscribeWatcherWithDelay = () => {
    setTimeout(async () => {
      await IndexerWatcher.start()
      unsubscribe = IndexerWatcher.subscribe((running: boolean) => {
        if (!running) {
          unsubscribe()
          indexerRunning = false
          showCheckmark = true
          checkmarkTimeout = setTimeout(() => {
            showCheckmark = false
            clickable = true
            checkmarkTimeout = null
          }, 1000) // Show checkmark for 1000ms
        }
      })
    }, 500)
  }

  onMount(() => {
    const initialRunning = get(IndexerWatcher)

    if (initialRunning) {
      indexerRunning = true
      clickable = false
      unsubscribe = IndexerWatcher.subscribe((running: boolean) => {
        if (!running) {
          unsubscribe()
          indexerRunning = false
          showCheckmark = true
          checkmarkTimeout = setTimeout(() => {
            showCheckmark = false
            clickable = true
            checkmarkTimeout = null
          }, 1000)
        }
      })
    }
  })

  const handleClick = async () => {
    if (!clickable) return

    clickable = false
    showCheckmark = false
    indexerRunning = true

    await startIndexer()
    subscribeWatcherWithDelay()
  }

  onDestroy(() => {
    unsubscribe()

    if (checkmarkTimeout) clearTimeout(checkmarkTimeout)
  })
</script>

<OptionRow onSelect="{handleClick}">
  Re-index

  <span slot="trailing" class="lifecycle-icon">
    {#if indexerRunning}
      <span class="icon-wrapper" transition:fade="{{ duration: 100 }}">
        <Circle size="1.4" unit="rem" color="var(--button-primary)" />
      </span>
    {/if}

    {#if showCheckmark}
      <span class="icon-wrapper checkmark-icon" transition:fade="{{ duration: 300 }}">
        <Icon icon="ic:baseline-check" />
      </span>
    {/if}

    {#if !indexerRunning && !showCheckmark}
      <span class="icon-wrapper" transition:fade="{{ duration: 100 }}">
        <Icon icon="ic:baseline-refresh" />
      </span>
    {/if}
  </span>
</OptionRow>
