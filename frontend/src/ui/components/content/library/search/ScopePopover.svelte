<style>
  .up-hint {
    display: inline-flex;

    color: var(--text-color-light);
    font-size: var(--font-md);
    opacity: 0.6;
  }
</style>

<script lang="ts">
  import Icon from '@iconify/svelte'
  import OptionRow from '../../../popover/OptionRow.svelte'
  import { searchScope, setFolderScope, setGlobalScope } from '../../../../../stores/search-scope'
  import type { AncestorRecord } from '../../../../../types/library-item'

  export let currentFolderId: string | null = null
  export let currentFolderName: string = ''
  export let ancestors: AncestorRecord[] = []
  export let close: () => void = () => {}

  const selectFolder = () => {
    if (currentFolderId && currentFolderName) {
      setFolderScope(currentFolderId, currentFolderName)
      close()
    }
  }

  const selectAncestor = (ancestor: AncestorRecord) => {
    setFolderScope(ancestor.id, ancestor.title)
    close()
  }

  const selectGlobal = () => {
    setGlobalScope()
    close()
  }

  $: activeFolderId = $searchScope.type === 'folder' ? $searchScope.folderId : null
  $: isGlobalActive = $searchScope.type === 'global'
  $: ancestorRows = ancestors.slice(1).reverse()
</script>

{#if currentFolderId && currentFolderName}
  <OptionRow selected="{activeFolderId === currentFolderId}" check="{true}" onSelect="{selectFolder}">
    <span slot="leading"><Icon icon="carbon:folder" /></span>
    {currentFolderName}
  </OptionRow>
{/if}

{#each ancestorRows as ancestor (ancestor.id)}
  <OptionRow selected="{activeFolderId === ancestor.id}" check="{true}" onSelect="{() => selectAncestor(ancestor)}">
    <span slot="leading" class="up-hint"><Icon icon="lucide:corner-left-up" /></span>
    {ancestor.title}
  </OptionRow>
{/each}

<OptionRow selected="{isGlobalActive}" check="{true}" onSelect="{selectGlobal}">
  <span slot="leading"><Icon icon="mdi:collection" /></span>
  All files
</OptionRow>
