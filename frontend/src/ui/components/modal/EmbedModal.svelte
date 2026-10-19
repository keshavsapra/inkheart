<style>
  .body {
    width: 34.5rem;
    max-width: 90vw;
    background: var(--card-subtle);
    border: 1px solid var(--card-contrast);
    border-radius: var(--radius-sm);
    box-shadow: var(--elevation-2);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
    text-align: left;
  }

  .header {
    padding: 14px 24px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--card-color);
  }

  h2 {
    margin: 0;
    font-size: var(--font-md);
    font-weight: var(--weight-light);
    color: var(--text-color-default);
  }

  .content {
    padding: 20px 24px 24px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .embed-row {
    display: grid;
    grid-template-columns: 104px 1fr;
    gap: 16px;
    align-items: start;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-width: 0;
  }

  label {
    font-size: var(--font-sm);
    font-weight: var(--weight-bold);
    color: var(--text-color-default);
  }

  input {
    width: 100%;
    height: 36px;
    padding: 0 14px;
    background: var(--card-color);
    border: 1px solid var(--card-contrast);
    border-radius: var(--radius-sm);
    color: var(--text-color-default);
    font-size: var(--font-sm);
    box-sizing: border-box;
    transition:
      border-color 0.15s,
      box-shadow 0.15s;
  }

  input:focus {
    outline: none;
    border-color: var(--color-primary-400);
  }

  input.readonly {
    background: var(--background);
    color: var(--text-color-light);
  }

  .stepper {
    position: relative;
  }

  .stepper input {
    padding-right: 34px;
    -moz-appearance: textfield;
    appearance: textfield;
  }

  .stepper input::-webkit-outer-spin-button,
  .stepper input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  .stepper .stepper-buttons {
    position: absolute;
    right: 7px;
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .stepper .stepper-buttons button {
    all: unset;
    width: 22px;
    height: 15px;
    border-radius: var(--radius-xs);
    background: var(--card-contrast);
    color: var(--text-color-default);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .stepper .stepper-buttons button:hover {
    background: var(--dark-5);
  }

  .embed-output {
    display: flex;
    gap: 8px;
    align-items: center;
    padding-top: 20px;
    border-top: 1px solid var(--card-color);
  }

  .embed-output input {
    flex: 1;
    min-width: 0;
  }

  @media (max-width: 768px) {
    .body {
      width: 90vw;
    }
  }
</style>

<script lang="ts">
  import { toast } from '@zerodevx/svelte-toast'
  import { copyTextToClipboard } from '../../../lib/copy-to-clipboard'
  import Modal from './Modal.svelte'
  import Button from '../button/Button.svelte'
  import IconButton from '../button/IconButton.svelte'
  import Icon from '@iconify/svelte'

  export let open = false

  export let pageNumber = 1
  export let previewText = ''

  export let baseUrl = ''

  $: url = `${baseUrl}?page=${pageNumber}&preview=${encodeURIComponent(previewText)}`

  const handleClose = () => {
    open = false
  }

  const handleCopy = () => {
    copyTextToClipboard(url)
    toast.push('Link copied!')
  }

  const increment = () => {
    pageNumber = (pageNumber || 0) + 1
  }

  const decrement = () => {
    pageNumber = Math.max(1, (pageNumber || 1) - 1)
  }
</script>

<Modal bind:open="{open}">
  <div class="body">
    <div class="header">
      <h2>Embed</h2>
      <IconButton onClick="{handleClose}" ariaLabel="Close">
        <Icon icon="mdi:close" width="18" />
      </IconButton>
    </div>

    <div class="content">
      <div class="embed-row">
        <div class="field">
          <label for="embed-page-number">Page Number</label>
          <div class="stepper">
            <input id="embed-page-number" type="number" min="1" bind:value="{pageNumber}" />
            <div class="stepper-buttons">
              <button type="button" aria-label="Increase page" on:click="{increment}">
                <Icon icon="mdi:chevron-up" width="14" />
              </button>
              <button type="button" aria-label="Decrease page" on:click="{decrement}">
                <Icon icon="mdi:chevron-down" width="14" />
              </button>
            </div>
          </div>
        </div>
        <div class="field">
          <label for="embed-preview-text">Preview Text</label>
          <input id="embed-preview-text" bind:value="{previewText}" />
        </div>
      </div>

      <div class="embed-output">
        <input class="readonly" value="{url}" readonly aria-label="Embed link" />
        <Button variant="primary" onClick="{handleCopy}">Copy Embed Link</Button>
      </div>
    </div>
  </div>
</Modal>
