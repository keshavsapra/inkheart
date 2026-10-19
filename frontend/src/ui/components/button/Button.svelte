<style>
  .button {
    font-family: var(--default-font);
    font-size: var(--font-sm);
    font-weight: var(--weight-body);
    line-height: 1;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    border: 1px solid transparent;
    border-radius: var(--radius-sm);
    text-decoration: none;
    transition:
      background 120ms ease,
      border-color 120ms ease,
      color 120ms ease;
  }

  .button:focus-visible {
    outline: none;
    box-shadow: var(--focus-ring);
  }

  .button:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }

  .button.secondary {
    background: var(--card-subtle);
    border-color: var(--card-contrast);
    color: var(--text-color-default);
    padding: 9px 16px;
  }

  .button.secondary:hover:not(:disabled) {
    background: var(--card-color);
    border-color: var(--dark-5);
  }

  .button.primary {
    background: var(--button-primary);
    border-color: var(--button-primary);
    color: #fff;
    padding: 9px 18px;
  }

  .button.primary:hover:not(:disabled) {
    background: var(--color-primary-300);
  }

  .button.destructive {
    background: transparent;
    color: var(--danger-color);
    padding: 9px 14px;
  }

  .button.destructive:hover:not(:disabled) {
    background: color-mix(in oklab, var(--danger-color) 12%, transparent);
  }

  .button.destructive-solid {
    background: var(--danger-color);
    border-color: var(--danger-color);
    color: #fff;
    padding: 9px 18px;
    font-weight: var(--weight-bold);
  }

  .button.destructive-solid:hover:not(:disabled) {
    background: color-mix(in oklab, var(--danger-color) 88%, black);
  }

  /* link */
  .button.link {
    background: transparent;
    color: var(--color-primary-500);
    padding: 9px 8px;
    text-decoration: underline;
    text-underline-offset: 3px;
  }

  .button.link:hover:not(:disabled) {
    color: var(--text-color-default);
  }

  /* compact */
  .button.small {
    padding: 5px 12px;
    font-size: var(--font-xs);
  }

  .spinner {
    width: 14px;
    height: 14px;
    flex: 0 0 auto;
    border-radius: 50%;
    border: 2px solid currentColor;
    border-top-color: transparent;
    animation: button-spin 700ms linear infinite;
  }

  @keyframes button-spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>

<script lang="ts">
  export let variant: 'secondary' | 'primary' | 'destructive' | 'destructive-solid' | 'link' = 'secondary'
  export let size: 'md' | 'sm' = 'md'
  export let href = ''
  export let target = ''
  export let type: 'button' | 'submit' | 'reset' = 'button'
  export let disabled = false
  export let pending = false
  export let onClick: () => void = () => {}
</script>

{#if href}
  <a
    class="button"
    class:secondary="{variant === 'secondary'}"
    class:primary="{variant === 'primary'}"
    class:destructive="{variant === 'destructive'}"
    class:destructive-solid="{variant === 'destructive-solid'}"
    class:link="{variant === 'link'}"
    class:small="{size === 'sm'}"
    href="{href}"
    target="{target}"
    on:click="{onClick}"
  >
    {#if pending}<span class="spinner" aria-hidden="true"></span>{/if}
    <slot />
  </a>
{:else}
  <button
    class="button"
    class:secondary="{variant === 'secondary'}"
    class:primary="{variant === 'primary'}"
    class:destructive="{variant === 'destructive'}"
    class:destructive-solid="{variant === 'destructive-solid'}"
    class:link="{variant === 'link'}"
    class:small="{size === 'sm'}"
    type="{type}"
    disabled="{disabled || pending}"
    on:click="{onClick}"
  >
    {#if pending}<span class="spinner" aria-hidden="true"></span>{/if}
    <slot />
  </button>
{/if}
