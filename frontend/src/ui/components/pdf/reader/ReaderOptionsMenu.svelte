<style>
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

  .icon-btn:focus-visible {
    border-radius: var(--radius-sm);
    box-shadow: var(--focus-ring);
  }

  .group-header {
    padding: 8px 9px 2px;
    color: var(--text-color-light);
    font-size: var(--font-2xs);
    font-weight: var(--weight-bold);
    letter-spacing: var(--tracking-label);
    text-transform: uppercase;
  }

  .separator {
    height: 1px;
    margin: 4px 0;
    background: var(--card-subtle);
  }

  .leading {
    color: var(--text-color-light);
    font-size: var(--font-base);
  }

</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import Popover from '../../popover/Popover.svelte'
  import OptionRow from '../../popover/OptionRow.svelte'

  export let spreadMode: 'none' | 'odd' | 'even' = 'none'
  export let scrollMode: 'page' | 'vertical' | 'horizontal' | 'wrapped' = 'vertical'
  export let cursorMode: 'select' | 'hand' = 'select'
  export let showFullscreen = false
  export let showTheme = false
  export let isDark = false
  export let onToggleTheme = () => {}
  export let onPresentation = () => {}
  export let onFullscreen = () => {}
  export let onDownload = () => {}
  export let onEmbed = () => {}
  export let onSetSpread: (mode: string) => void = () => {}
  export let onSetScroll: (mode: string) => void = () => {}
  export let onSetCursor: (mode: string) => void = () => {}
  export let onRotateCw = () => {}
  export let onRotateCcw = () => {}
  export let onDocumentProperties = () => {}
</script>

<Popover strategy="fixed" placement="bottom-end" width="232px" let:close>
  <div slot="trigger" let:toggle let:open>
    <button
      class="icon-btn"
      type="button"
      on:click="{toggle}"
      aria-haspopup="menu"
      aria-expanded="{open}"
      aria-label="Options"
    >
      <Icon icon="mdi:dots-vertical" width="18" />
    </button>
  </div>

  <OptionRow
    onSelect="{() => {
      onPresentation()
      close()
    }}"
  >
    Presentation mode

    <span slot="leading" class="leading">
      <Icon icon="mdi:play-box-outline" />
    </span>
  </OptionRow>

  {#if showFullscreen}
    <OptionRow
      onSelect="{() => {
        onFullscreen()
        close()
      }}"
    >
      Open fullscreen reader

      <span slot="leading" class="leading">
        <Icon icon="mdi:open-in-new" />
      </span>
    </OptionRow>
  {/if}

  {#if showTheme}
    <OptionRow check="{true}" selected="{isDark}" onSelect="{onToggleTheme}">
      Dark mode

      <span slot="leading" class="leading">
        <Icon icon="ic:baseline-dark-mode" />
      </span>
    </OptionRow>
  {/if}

  <OptionRow
    onSelect="{() => {
      onDownload()
      close()
    }}"
  >
    Download

    <span slot="leading" class="leading">
      <Icon icon="mdi:download" />
    </span>
  </OptionRow>

  <OptionRow
    onSelect="{() => {
      onEmbed()
      close()
    }}"
  >
    Share / Embed

    <span slot="leading" class="leading">
      <Icon icon="mdi:link-variant" />
    </span>
  </OptionRow>

  <div class="separator"></div>
  <div class="group-header">Spread</div>

  <OptionRow check="{true}" selected="{spreadMode === 'none'}" onSelect="{() => onSetSpread('none')}">None</OptionRow>

  <OptionRow check="{true}" selected="{spreadMode === 'odd'}" onSelect="{() => onSetSpread('odd')}">Odd</OptionRow>

  <OptionRow check="{true}" selected="{spreadMode === 'even'}" onSelect="{() => onSetSpread('even')}">Even</OptionRow>

  <div class="separator"></div>
  <div class="group-header">Scroll</div>

  <OptionRow check="{true}" selected="{scrollMode === 'page'}" onSelect="{() => onSetScroll('page')}">Page</OptionRow>

  <OptionRow check="{true}" selected="{scrollMode === 'vertical'}" onSelect="{() => onSetScroll('vertical')}">
    Vertical
  </OptionRow>

  <OptionRow check="{true}" selected="{scrollMode === 'horizontal'}" onSelect="{() => onSetScroll('horizontal')}">
    Horizontal
  </OptionRow>

  <OptionRow check="{true}" selected="{scrollMode === 'wrapped'}" onSelect="{() => onSetScroll('wrapped')}">
    Wrapped
  </OptionRow>

  <div class="separator"></div>
  <div class="group-header">Cursor</div>

  <OptionRow check="{true}" selected="{cursorMode === 'select'}" onSelect="{() => onSetCursor('select')}">
    Text selection
  </OptionRow>

  <OptionRow check="{true}" selected="{cursorMode === 'hand'}" onSelect="{() => onSetCursor('hand')}">
    Hand tool
  </OptionRow>

  <div class="separator"></div>

  <OptionRow onSelect="{onRotateCw}">
    Rotate clockwise

    <span slot="leading" class="leading">
      <Icon icon="mdi:rotate-right" />
    </span>
  </OptionRow>

  <OptionRow onSelect="{onRotateCcw}">
    Rotate counterclockwise

    <span slot="leading" class="leading">
      <Icon icon="mdi:rotate-left" />
    </span>
  </OptionRow>

  <div class="separator"></div>

  <OptionRow
    onSelect="{() => {
      onDocumentProperties()
      close()
    }}"
  >
    Document properties

    <span slot="leading" class="leading">
      <Icon icon="mdi:information-outline" />
    </span>
  </OptionRow>
</Popover>
