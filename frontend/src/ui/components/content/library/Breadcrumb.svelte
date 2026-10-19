<style>
  .breadcrumb {
    display: flex;
    flex-direction: row;
    align-items: center;
    flex-wrap: nowrap;

    min-width: 0;

    font-size: var(--font-xs);
    color: var(--text-color-light);
  }

  .breadcrumb :global(a) {
    display: inline-block;

    max-width: 12rem;

    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;

    color: inherit;
    text-decoration: none;
    opacity: 0.8;
  }

  .breadcrumb :global(a:hover) {
    color: var(--text-color-default);
    text-decoration: underline;
    opacity: 1;
  }

  .separator {
    flex-shrink: 0;

    margin: 0 0.35rem;
    opacity: 0.5;
  }

  .ellipsis-trigger {
    flex-shrink: 0;

    margin: 0;
    padding: 0;
    border: none;
    border-radius: 0;
    background: none;

    font: inherit;
    color: inherit;
    opacity: 0.8;

    cursor: pointer;
  }

  .ellipsis-trigger:hover {
    color: var(--text-color-default);
    opacity: 1;
  }

  .ellipsis-trigger.solo {
    display: inline-flex;
    align-items: center;
    justify-content: center;

    width: 1.75rem;
    height: 1.75rem;
    padding: 0;
    margin-top: 0.1rem;
    margin-left: 1rem;

    font-size: var(--font-base);
    color: var(--text-color-light);
    opacity: 1;
  }

  .ellipsis-trigger.solo:hover {
    color: var(--text-color-default);
  }

  .up-hint {
    display: inline-flex;

    color: var(--text-color-light);
    font-size: var(--font-md);
    opacity: 0.6;
  }
</style>

<script lang="ts">
  import { Link, useNavigate } from 'svelte-navigator'
  import Icon from '@iconify/svelte'
  import Popover from '../../popover/Popover.svelte'
  import OptionRow from '../../popover/OptionRow.svelte'
  import type { AncestorRecord } from '../../../../types/library-item'

  export let ancestors: AncestorRecord[] = []
  export let ellipsisOnly = false

  const navigate = useNavigate()

  const collapseThreshold = 3
  const rootLabel = 'Files'

  $: crumbs = ancestors.map((ancestor, index) => ({
    id: ancestor.id,
    label: index === 0 ? rootLabel : ancestor.title,
    to: index === 0 ? '/' : `/library/${ancestor.id}`,
  }))

  $: collapsed = crumbs.length > collapseThreshold

  $: last = crumbs[crumbs.length - 1]

  $: hidden = collapsed ? crumbs.slice(1, crumbs.length - 1) : []

  const goTo = (to: string, close: () => void) => {
    navigate(to)
    close()
  }
</script>

{#if crumbs.length > 0}
  <nav class="breadcrumb" aria-label="Folder path">
    {#if ellipsisOnly}
      <Popover strategy="fixed" placement="bottom-start" let:close>
        <button
          slot="trigger"
          let:toggle
          let:open
          class="ellipsis-trigger solo"
          type="button"
          aria-haspopup="menu"
          aria-expanded="{open}"
          aria-label="Show folder path"
          on:click="{toggle}"
        >
          <Icon icon="mdi:dots-horizontal" />
        </button>

        {#each crumbs as crumb}
          <OptionRow onSelect="{() => goTo(crumb.to, close)}">
            <span slot="leading" class="up-hint"><Icon icon="lucide:corner-left-up" /></span>

            {crumb.label}
          </OptionRow>
        {/each}
      </Popover>
    {:else if collapsed}
      <Link to="{crumbs[0].to}">{crumbs[0].label}</Link>
      <span class="separator">/</span>

      <Popover strategy="fixed" placement="bottom-start" let:close>
        <button
          slot="trigger"
          let:toggle
          let:open
          class="ellipsis-trigger"
          type="button"
          aria-haspopup="menu"
          aria-expanded="{open}"
          aria-label="Show hidden folders"
          on:click="{toggle}"
        >
          …
        </button>

        {#each hidden as crumb}
          <OptionRow onSelect="{() => goTo(crumb.to, close)}">
            <span slot="leading" class="up-hint"><Icon icon="lucide:corner-left-up" /></span>

            {crumb.label}
          </OptionRow>
        {/each}
      </Popover>

      <span class="separator">/</span>
      <Link to="{last.to}">{last.label}</Link>
    {:else}
      {#each crumbs as crumb, index}
        {#if index > 0}
          <span class="separator">/</span>
        {/if}

        <Link to="{crumb.to}">{crumb.label}</Link>
      {/each}
    {/if}
  </nav>
{/if}
