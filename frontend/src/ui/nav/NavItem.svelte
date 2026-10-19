<style>
  .nav-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 10px;
    border-radius: var(--radius-sm);
    color: var(--text-color-default);
    font-size: var(--font-sm);
    cursor: pointer;
    text-decoration: none;
  }

  .nav-item .nav-icon {
    color: var(--color-primary-400);
    display: inline-flex;
    flex: 0 0 auto;
    font-size: var(--font-base);
  }

  .nav-item:hover {
    background: var(--card-color);
  }

  .nav-item.active {
    background: var(--foreground);
  }
</style>

<script lang="ts">
  import { Link, useLocation } from 'svelte-navigator'
  import Icon from '@iconify/svelte'

  export let path

  const location = useLocation()

  $: active = $location.pathname === path
</script>

<Link to="{path}">
  <div class="nav-item" class:active="{active}">
    <span class="nav-icon"><Icon icon="mdi:folder-outline" /></span>
    <slot />
  </div>
</Link>
