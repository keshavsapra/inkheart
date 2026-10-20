<style>
  .container {
    height: 100%;
    position: relative;
  }

  .nav {
    z-index: 200;
    height: 100%;
    width: 13rem;
    position: relative;
    box-sizing: border-box;
    overflow: hidden;

    display: flex;
    flex-direction: column;

    padding: 16px 12px 14px;

    background-color: var(--card-subtle);
    border-right: 1px solid var(--card-color);

    transition: width 100ms ease-in-out;
  }

  /* for mobile open and close*/
  .nav.open {
    transform: translateX(0);

    transition: transform 50ms linear;
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 2px 8px 16px;
    margin-bottom: 14px;
    border-bottom: 1px solid var(--card-color);
  }

  h2 {
    margin: 0;
    font-size: var(--font-md);
    font-weight: var(--weight-wordmark);
    letter-spacing: var(--tracking-wordmark);
    color: var(--text-color-default);
    white-space: nowrap;
    overflow: hidden;
    transition: opacity 80ms ease 70ms;
  }

  .collapse-btn {
    all: unset;
    cursor: pointer;
    color: var(--text-color-light);
    width: 22px;
    height: 22px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-sm);
    font-size: var(--font-md);
  }

  .collapse-btn:hover {
    color: var(--text-color-default);
  }

  nav {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    overflow-x: hidden;
    display: flex;
    flex-direction: column;
  }

  .footer {
    margin-top: auto;
    padding: 14px 10px 10px;
    display: flex;
    flex-direction: column;
    color: var(--text-color-light);
    text-align: center;
  }

  .version-wrapper {
    align-self: center;
  }

  .settings-wrapper {
    align-self: flex-end;
    margin-top: 16px;
  }

  .collapsed {
    width: 4rem;
    overflow: hidden;

    transition: width 100ms ease-in-out;
  }

  .collapsed nav {
    display: none;
  }

  .collapsed .header {
    justify-content: center;
    padding: 2px 0 8px;
    margin-bottom: 0;
    border-bottom: none;
  }

  .collapsed .header h2 {
    width: 0;
    opacity: 0;
    transition: opacity 60ms ease;
  }

  .collapsed .footer {
    padding-left: 0;
    padding-right: 0;
  }

  .collapsed .settings-wrapper {
    align-self: center;
  }

  @media (--small-viewport) {
    .nav {
      position: fixed;
      height: calc(100% - var(--app-bar-height));
      /* for mobile open and close*/
      transform: translateX(-13rem);

      transition: transform 50ms linear;
    }

    .header {
      display: none;
    }
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import SettingsButton from '../components/settings/SettingsButton.svelte'
  import Version from './Version.svelte'
  import SidebarRail from './SidebarRail.svelte'

  export let open
  export let isOpen = true

  $: collapseSideBar = !isOpen && !open

  const toggleSidebar = () => {
    isOpen = !isOpen
  }
</script>

<div class="container">
  <div class="nav" class:open="{open}" class:collapsed="{collapseSideBar}">
    <div class="header">
      <h2>NextChapter</h2>
      <button class="collapse-btn" on:click="{toggleSidebar}" aria-label="Collapse sidebar">
        <Icon icon="{isOpen ? 'mdi:chevron-left' : 'mdi:chevron-right'}" />
      </button>
    </div>
    <nav>
      <slot expanded="{!collapseSideBar}" />
    </nav>
    {#if collapseSideBar}
      <SidebarRail />
    {/if}
    <div class="footer">
      <div class="version-wrapper"><Version /></div>
      <div class="settings-wrapper"><SettingsButton /></div>
    </div>
  </div>
</div>
