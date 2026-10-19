<style>
  .bar {
    box-sizing: border-box;
    display: flex;
    align-items: center;
    gap: 4px;
    height: 36px;
    padding: 0 8px;
    background: var(--background);
    border-bottom: 1px solid var(--card-color);
  }

  .cluster {
    flex: 1 1 0;
    display: flex;
    align-items: center;
    gap: 2px;
  }

  .cluster:last-child {
    justify-content: flex-end;
  }

  .title {
    flex: 0 1 auto;
    min-width: 0;
    margin: 0 6px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    text-align: center;
    color: var(--text-color-default);
    font-size: var(--font-xs);
  }

  .spacer {
    flex: 1;
  }

  .icon-btn {
    all: unset;
    box-sizing: border-box;
    width: 28px;
    height: 28px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--text-color-light);
    cursor: pointer;
    transition: color 120ms ease;
  }

  .icon-btn:hover {
    color: var(--text-color-default);
  }

  .icon-btn.on {
    color: var(--color-primary-500);
  }

  .icon-btn:focus-visible {
    border-radius: var(--radius-sm);
    box-shadow: var(--focus-ring);
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'

  export let title = ''
  export let mode: 'reader' | 'fullscreen' | 'embed' = 'reader'
  export let isMobile = false
  export let sidebarOpen = false
  export let isDark = false
  export let onBack = () => {}
  export let onToggleSidebar = () => {}
  export let onOpenFind = () => {}
  export let onToggleTheme = () => {}
  export let onFullscreen = () => {}
</script>

<div class="bar">
  <div class="cluster">
    {#if !isMobile && mode === 'reader'}
      <button class="icon-btn" type="button" on:click="{onBack}" aria-label="Back" title="Back">
        <Icon icon="mdi:arrow-left" width="18" />
      </button>
    {/if}

    <button
      class="icon-btn"
      class:on="{sidebarOpen}"
      type="button"
      on:click="{onToggleSidebar}"
      aria-label="Toggle sidebar"
      aria-pressed="{sidebarOpen}"
      title="Toggle sidebar"
    >
      <Icon icon="mdi:dock-left" width="18" />
    </button>
  </div>

  {#if isMobile}
    <div class="spacer"></div>
  {:else}
    <span class="title">{title}</span>
  {/if}

  <div class="cluster">
    <button class="icon-btn" type="button" on:click="{onOpenFind}" aria-label="Find" title="Find">
      <Icon icon="mdi:magnify" width="18" />
    </button>

    {#if !isMobile}
      <button
        class="icon-btn"
        class:on="{isDark}"
        type="button"
        on:click="{onToggleTheme}"
        aria-label="Toggle theme"
        aria-pressed="{isDark}"
        title="Toggle theme"
      >
        <Icon icon="ic:baseline-dark-mode" width="18" />
      </button>

      {#if mode === 'reader'}
        <button
          class="icon-btn"
          type="button"
          on:click="{onFullscreen}"
          aria-label="Open fullscreen reader"
          title="Open fullscreen reader"
        >
          <Icon icon="mdi:open-in-new" width="18" />
        </button>
      {/if}
    {/if}

    <slot name="options" />
  </div>
</div>
