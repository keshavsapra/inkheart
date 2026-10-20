<style>
  .container {
    display: grid;
    grid-template-columns: auto 1fr;
    grid-template-rows: 1fr;
    grid-template-areas: 'sidebar main';

    height: 100%;
  }

  .container.loading {
    display: flex;
    justify-content: center;
  }

  .sidebar {
    grid-area: sidebar;
    height: 100%;
  }

  .content {
    grid-area: main;

    min-height: 0;
    min-width: 0;
    overflow: hidden;
  }

  .loader-container {
    display: flex;
    width: 100%;
    height: 100%;
    justify-content: center;
    align-items: center;
  }

  .hamburger {
    grid-area: header;
    z-index: 150;

    display: none;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    height: var(--app-bar-height);

    padding-left: 0;
    padding-right: 1rem;

    box-sizing: border-box;

    background: var(--card-subtle);
    border-bottom: 1px solid var(--card-color);
  }

  h2 {
    margin: 0 1em 0 0;

    flex-shrink: 0;

    font-size: var(--font-md);
    font-weight: var(--weight-wordmark);
    letter-spacing: var(--tracking-wordmark);
    color: var(--text-color-default);
    line-height: 1;
  }

  .hamburger :global(button.hamburger) {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .bar-nav {
    display: flex;
    align-items: center;
    gap: 0;

    flex: 1;
    min-width: 0;
    margin-left: -0.25rem;
    margin-top: 0.15rem;
  }

  .bar-back {
    display: inline-flex;
    align-items: center;

    flex-shrink: 0;
    margin-top: 0.1rem;

    cursor: pointer;
  }

  .bar-title {
    min-width: 0;
    margin-top: -0.1rem;

    font-size: var(--font-md);
    color: var(--text-color-default);

    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .overlay {
    display: none;
  }

  .overlay.show {
    grid-area: main;
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100%;

    background: black;
    opacity: 0.5;

    z-index: 50;

    display: none;
  }

  .nav-item-wrapper {
    flex-shrink: 0;
  }

  .scroll-container {
    overflow-y: auto;
    overflow-x: hidden;
    flex: 1 1 0;
    min-height: 0;
  }

  .scroll-container::-webkit-scrollbar {
    width: 0.5rem;
  }

  .scroll-container::-webkit-scrollbar-track {
    background: transparent;
  }

  .scroll-container::-webkit-scrollbar-thumb {
    background: var(--card-subtle);
    border-radius: var(--radius-sm);
  }

  .scroll-container::-webkit-scrollbar-thumb:hover {
    background: var(--text-color-light);
  }

  @media (--small-viewport) {
    .container {
      grid-template-columns: auto 1fr;
      grid-template-rows: auto 1fr;
      grid-template-areas: 'header header' 'sidebar main';
    }

    .hamburger {
      display: flex;
    }

    .overlay.show {
      display: block;
    }
  }
</style>

<script lang="ts">
  import { Router, Link, Route, useNavigate, useLocation } from 'svelte-navigator'

  import { Jumper } from 'svelte-loading-spinners'
  import { SvelteToast } from '@zerodevx/svelte-toast'

  import Main from './pages/main.svelte'
  import SideNav from './ui/nav/SideNav.svelte'
  import NavItem from './ui/nav/NavItem.svelte'
  import PinnedFolders from './ui/nav/PinnedFolders.svelte'
  import Collections from './ui/nav/Collections.svelte'
  import PdfReader from './pages/pdfreader.svelte'
  import CollectionPage from './pages/collection.svelte'
  import Login from './pages/login.svelte'

  import { onMount } from 'svelte'
  import { initFirebase } from './lib/firebase'
  import { auth as authStore, usingAuth } from './stores/auth'
  import { setToken, setUsingAuth } from './lib/localstorage'
  import FullscreenReader from './pages/fullscreen-reader.svelte'
  import { Hamburger } from 'svelte-hamburgers'
  import { createGetRequest } from './lib/create-request'
  import { apiUrl } from './utils/api'
  import { initTextIndexToast } from './services/text-index-toast'
  import { folderContext } from './stores/folder-context'
  import { readerContext } from './stores/reader-context'
  import LeftChevron from './ui/icons/LeftChevron.svelte'
  import { overflowFade } from './lib/overflow-fade'
  import Breadcrumb from './ui/components/content/library/Breadcrumb.svelte'

  export let useAuth = false

  const navigate = useNavigate()
  const location = useLocation()

  $: loading = true

  $: logged_in = false

  $: show_app_chrome = !($location.pathname.startsWith('/fullscreen') || $location.pathname.startsWith('/embed'))

  let sidebarOpen: boolean

  const onBarBackKeydown = (event: KeyboardEvent) => {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault()
      $folderContext?.onBack()
    }
  }

  const onReaderBackKeydown = (event: KeyboardEvent) => {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault()
      $readerContext?.onBack()
    }
  }

  onMount(() => {
    usingAuth.set(useAuth)
    setUsingAuth($usingAuth)

    if ($usingAuth) {
      const { auth, onAuthStateChanged } = initFirebase()
      onAuthStateChanged(auth, authStore.set)
      onAuthStateChanged(auth, checkAuth)
    } else {
      loading = false
      logged_in = true
      initTextIndexToast()
    }
  })

  const checkAuth = () => {
    return authStore.subscribe((user) => {
      if (!user) {
        logged_in = false
        loading = false
        navigate('/login')
      } else {
        logged_in = true
        user.getIdToken().then((idToken) => {
          setToken(idToken)
          loading = false
          initTextIndexToast()
        })
      }
    })
  }
</script>

<div class="container" class:loading="{loading}">
  {#if !loading}
    {#if show_app_chrome}
      <div class="overlay" class:show="{sidebarOpen}"></div>
      <div class="hamburger" class:open="{sidebarOpen}">
        <Hamburger
          bind:open="{sidebarOpen}"
          type="spin"
          --layer-width="1.4rem"
          --layer-height="0.125rem"
          --color="var(--text-color-light)"
        />

        {#if $readerContext}
          <div class="bar-nav">
            <div
              class="bar-back"
              on:click="{$readerContext.onBack}"
              on:keydown="{onReaderBackKeydown}"
              role="button"
              tabindex="0"
              aria-label="Back"
            >
              <LeftChevron primaryColor="var(--text-color-default)" height="1.2rem"></LeftChevron>
            </div>

            <span class="bar-title" use:overflowFade>{$readerContext.title}</span>
          </div>
        {:else if $folderContext}
          <div class="bar-nav">
            <div
              class="bar-back"
              on:click="{$folderContext.onBack}"
              on:keydown="{onBarBackKeydown}"
              role="button"
              tabindex="0"
              aria-label="Back"
            >
              <LeftChevron primaryColor="var(--text-color-default)" height="1.2rem"></LeftChevron>
            </div>

            <span class="bar-title">{$folderContext.title}</span>

            {#if $folderContext.ancestors.length > 0}
              <Breadcrumb ancestors="{$folderContext.ancestors}" ellipsisOnly="{true}" />
            {/if}
          </div>
        {/if}

        {#if !$readerContext}
          <h2>Inkheart</h2>
        {/if}
      </div>
    {/if}
    {#if logged_in && show_app_chrome}
      <div class="sidebar">
        <SideNav bind:open="{sidebarOpen}" let:expanded>
          {#if expanded}
            <div class="nav-item-wrapper">
              <NavItem path="{'/'}">Files</NavItem>
            </div>
            <div class="scroll-container">
              <PinnedFolders />
              <Collections />
            </div>
          {/if}
        </SideNav>
      </div>
    {/if}
    <div class="content">
      <Route path="/"><Main /></Route>
      <Route path="/login"><Login /></Route>

      <Route path="/fullscreen/:file" let:params>
        <FullscreenReader file="{params.file}" mode="fullscreen" /></Route
      >
      <Route path="/embed/:file" let:params>
        <FullscreenReader file="{params.file}" mode="embed" />
      </Route>
      <Route path="/reader/:file" let:params>
        <PdfReader file="{params.file}" />
      </Route>
      <Route path="/collection/:id" let:params>
        <CollectionPage collectionId="{params.id}" />
      </Route>
      <Route path="/library/:item" let:params>
        <Main directory="{params.item}" />
      </Route>
    </div>
  {:else}
    <div class="loader-container">
      <div class="loader">
        <Jumper size="10" color="var(--button-primary)" unit="rem" duration="1s" />
      </div>
    </div>
  {/if}
  <SvelteToast />
</div>
