<style>
  .wrapper {
    display: flex;
    align-items: center;
  }

  .button {
    box-sizing: border-box;
    display: flex;
    align-items: center;
    justify-content: center;
    height: 42px;
    padding: 0 0.4rem;

    border: 1px solid var(--card-contrast);
    border-radius: var(--radius-md);

    background: transparent;
    color: var(--text-color-default);

    transition: background 120ms ease;
  }

  .button:hover {
    background: var(--card-color);
  }

  .button-main {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
  }

  .sort-icon {
    all: unset;

    font-size: var(--font-3xl);
    color: var(--text-color-default);

    display: flex;
    align-items: center;
    justify-content: center;

    cursor: pointer;
  }

  .sort-icon:focus-visible {
    border-radius: var(--radius-sm);
    box-shadow: var(--focus-ring);
  }

  .row-icon {
    display: flex;
    align-items: center;
    color: var(--button-secondary);
  }

  .row-icon.active {
    color: var(--button-primary);
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import { getActiveTheme, updateTheme } from '../../../../../lib/theme'
  import { selectedTheme } from '../../../../../stores/options'
  import { documentCardModeStore } from '../../../../../stores/reader'
  import Popover from '../../../popover/Popover.svelte'
  import OptionRow from '../../../popover/OptionRow.svelte'
  import ReIndexMenuItem from './ReIndexMenuItem.svelte'

  export let withHeader = false

  $: isDarkMode = $selectedTheme && getActiveTheme() === 'dark'

  const toggleDarkmode = () => {
    const active = getActiveTheme()
    const newTheme = active === 'dark' ? 'light' : 'dark'

    updateTheme(newTheme)
  }

  const toggleCoverMode = () => {
    $documentCardModeStore = $documentCardModeStore === 'minimal' ? 'poster' : 'minimal'
  }
</script>

<div class="wrapper">
  <Popover strategy="fixed" placement="bottom-end" width="11rem">
    <div slot="trigger" let:toggle let:open class="button">
      <div class="button-main">
        <button
          class="sort-icon"
          type="button"
          on:click="{toggle}"
          aria-haspopup="menu"
          aria-expanded="{open}"
          aria-label="View options"
        >
          <Icon icon="iconamoon:options-thin" />
        </button>
      </div>
    </div>

    <OptionRow onSelect="{toggleDarkmode}">
      Dark Mode

      <span slot="trailing" class="row-icon" class:active="{isDarkMode}">
        <Icon icon="ic:baseline-dark-mode" />
      </span>
    </OptionRow>

    <OptionRow onSelect="{toggleCoverMode}">
      Minimal covers

      <span slot="trailing" class="row-icon" class:active="{$documentCardModeStore === 'minimal'}">
        <Icon icon="mdi:image-multiple" />
      </span>
    </OptionRow>

    <ReIndexMenuItem />
  </Popover>
</div>
