<style>
  .icon-button {
    all: unset;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 30px;
    height: 30px;
    border-radius: var(--radius-sm);
    color: var(--text-color-light);
    background: transparent;
    flex-shrink: 0;
    transition: color 0.15s;
  }

  .icon-button:hover:not(:disabled) {
    color: var(--text-color-default);
  }

  .icon-button:focus-visible {
    outline: none;
    box-shadow: var(--focus-ring);
  }

  .icon-button:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }
</style>

<script lang="ts">
  export let onClick: () => void = () => {}
  export let ariaLabel = 'Button'
  export let disabled = false
</script>

<button class="icon-button" type="button" on:click="{onClick}" aria-label="{ariaLabel}" disabled="{disabled}">
  <slot />
</button>
