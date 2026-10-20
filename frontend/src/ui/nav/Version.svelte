<style>
  .version {
    display: flex;
    flex-direction: column;
    color: var(--text-color-light);
    font-size: var(--font-sm);
    line-height: 1.3;
  }

  .version-name {
    color: var(--dark-5);
    font-size: var(--font-xs);
  }

  .hidden {
    visibility: hidden;
  }
</style>

<script lang="ts">
  const version = __APP_VERSION__
  const codename = __APP_CODENAME__

  export let compact = false
</script>

<div class="version">
  <span class="version-number">{compact ? version : `Version ${version}`}</span>
  <span class="version-name" class:hidden="{compact}">({codename})</span>
</div>
