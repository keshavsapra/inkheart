<style>
  .settings-icon {
    all: unset;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--text-color-light);
    font-size: var(--font-base);
  }

  .settings-icon:hover {
    color: var(--text-color-default);
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import SettingsModal from './SettingsModal.svelte'

  export let compact = false

  let settingsOpen = false

  const handleClick = () => {
    settingsOpen = true
  }
</script>

<button class="settings-icon" on:click="{handleClick}" aria-label="Settings">
  <Icon icon="mdi:cog" />
</button>

<SettingsModal bind:open="{settingsOpen}" />
