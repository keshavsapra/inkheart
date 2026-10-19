<style>
  .body {
    width: 50rem;
    max-width: 90vw;
    height: 35rem;
    max-height: 80vh;
    max-height: 80dvh;
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
    padding: 20px 24px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--card-color);
    flex-shrink: 0;
  }

  h2 {
    margin: 0;
    font-size: var(--font-lg);
    font-weight: var(--weight-light);
    color: var(--text-color-default);
  }

  .content {
    padding: 16px 24px;
    overflow-y: auto;
    flex: 1;
  }

  .setting-section {
    display: flex;
    flex-direction: column;
  }

  .setting-section + .setting-section {
    margin-top: 18px;
    padding-top: 16px;
    border-top: 1px solid var(--card-color);
  }

  .setting-heading {
    margin: 0 0 2px;
    font-size: var(--font-sm);
    font-weight: var(--weight-bold);
    color: var(--text-color-default);
  }

  .setting-heading .qualifier {
    margin-left: 8px;
    font-size: var(--font-xs);
    font-weight: var(--weight-regular);
    color: var(--text-color-light);
  }

  .setting-row {
    display: flex;
    align-items: flex-start;
    gap: 20px;
    padding: 14px 0;
  }

  .setting-row + .setting-row {
    border-top: 1px solid var(--card-color);
  }

  .meta {
    flex: 1;
    min-width: 0;
  }

  .meta .title {
    font-size: var(--font-sm);
    font-weight: var(--weight-bold);
    color: var(--text-color-default);
  }

  .meta .description {
    margin-top: 4px;
    max-width: 56ch;
    font-size: var(--font-xs);
    line-height: 1.5;
    color: var(--text-color-light);
  }

  .control {
    flex: 0 0 auto;
    display: flex;
    align-items: center;
    padding-top: 2px;
  }

  select {
    height: 36px;
    width: 184px;
    padding: 0 38px 0 14px;
    background: var(--card-color);
    border: 1px solid var(--card-contrast);
    border-radius: var(--radius-sm);
    color: var(--text-color-default);
    font-family: var(--default-font);
    font-size: var(--font-sm);
    cursor: pointer;
    appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%239394a5' stroke-width='2.4' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='M6 9l6 6 6-6'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 14px center;
  }

  select:focus {
    outline: none;
    border-color: var(--color-primary-400);
  }

  .learn {
    all: unset;
    display: inline-block;
    margin-top: 8px;
    font-size: var(--font-xs);
    color: var(--color-primary-500);
    text-decoration: underline;
    cursor: pointer;
  }

  .learn:hover {
    color: var(--text-color-default);
  }

  .loader {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 2rem 0;
  }

  .footer {
    padding: 16px 24px;
    display: flex;
    align-items: center;
    border-top: 1px solid var(--card-color);
    flex-shrink: 0;
  }

  .footer .spacer {
    flex: 1;
  }

  @media (max-width: 768px) {
    .body {
      width: 95vw;
      max-width: 90vw;
      height: auto;
      max-height: 85vh;
      max-height: 85dvh;
    }
  }
</style>

<script lang="ts">
  import Modal from '../modal/Modal.svelte'
  import Icon from '@iconify/svelte'
  import IconButton from '../button/IconButton.svelte'
  import Toggle from '../input/Toggle.svelte'
  import { SyncLoader } from 'svelte-loading-spinners'
  import { createGetRequest, createPatchRequest } from '../../../lib/create-request'
  import { apiUrl } from '../../../utils/api'
  import TelemetryInfoModal from '../banner/TelemetryInfoModal.svelte'
  import {
    progressTrackingStore,
    pageSpreadModeStore,
    defaultZoomStore,
    showProgressOnCoversStore,
    useLegacyReaderStore,
    collapsibleToolbarStore,
  } from '../../../stores/reader'

  export let open = false

  interface UserSettingsResponse {
    telemetry_enabled: boolean
    telemetry_banner_shown: boolean
    telemetry_config_override: boolean
  }

  let telemetryEnabled = false
  let telemetryConfigOverride = false
  let loading = true
  let initialized = false
  let previousTelemetryValue: boolean | null = null
  let showTelemetryInfo = false

  const handleClose = () => {
    open = false
  }

  const fetchSettings = async () => {
    try {
      loading = true
      const response = await fetch(...createGetRequest(apiUrl('api/settings')))
      if (response.ok) {
        const data: UserSettingsResponse = await response.json()
        telemetryEnabled = data.telemetry_enabled
        telemetryConfigOverride = data.telemetry_config_override
        previousTelemetryValue = data.telemetry_enabled
        initialized = true
      } else {
        console.error('Failed to fetch settings:', response.statusText)
      }
    } catch (e) {
      console.error('Failed to fetch settings:', e)
    } finally {
      loading = false
    }
  }

  const updateTelemetrySetting = async () => {
    try {
      const response = await fetch(
        ...createPatchRequest(apiUrl('api/settings'), JSON.stringify({ telemetry_enabled: telemetryEnabled }), {
          'Content-Type': 'application/json',
        }),
      )
      if (!response.ok) {
        console.error('Failed to update telemetry setting:', response.statusText)
      } else {
        previousTelemetryValue = telemetryEnabled
      }
    } catch (e) {
      console.error('Failed to update telemetry setting:', e)
    }
  }

  $: if (open && !initialized) {
    fetchSettings()
  }

  $: if (initialized && telemetryEnabled !== previousTelemetryValue) {
    updateTelemetrySetting()
  }
</script>

<Modal bind:open="{open}">
  <TelemetryInfoModal bind:open="{showTelemetryInfo}" />

  <div class="body">
    <div class="header">
      <h2>Settings</h2>
      <IconButton onClick="{handleClose}" ariaLabel="Close">
        <Icon icon="mdi:close" width="18" />
      </IconButton>
    </div>

    <div class="content">
      <div class="setting-section">
        <p class="setting-heading">Reader <span class="qualifier">(Browser only)</span></p>

        <div class="setting-row">
          <div class="meta">
            <div class="title">Reading progress tracking</div>
            <div class="description">
              Remember your reading position in each document. Progress is saved locally in your browser.
            </div>
          </div>
          <div class="control">
            <Toggle bind:checked="{$progressTrackingStore}" ariaLabel="Reading progress tracking" />
          </div>
        </div>

        <div class="setting-row">
          <div class="meta">
            <div class="title">Show progress on covers</div>
            <div class="description">Display a reading meter and page position on document covers.</div>
          </div>
          <div class="control">
            <Toggle bind:checked="{$showProgressOnCoversStore}" ariaLabel="Show progress on covers" />
          </div>
        </div>

        <div class="setting-row">
          <div class="meta">
            <div class="title">Default page spread mode</div>
            <div class="description">Choose how pages are displayed in the reader by default.</div>
          </div>
          <div class="control">
            <select bind:value="{$pageSpreadModeStore}" aria-label="Default page spread mode">
              <option value="none">No spreads</option>
              <option value="odd">Odd spreads</option>
              <option value="even">Even spreads</option>
            </select>
          </div>
        </div>

        <div class="setting-row">
          <div class="meta">
            <div class="title">Default zoom level</div>
            <div class="description">Choose the zoom level the reader opens with by default.</div>
          </div>
          <div class="control">
            <select bind:value="{$defaultZoomStore}" aria-label="Default zoom level">
              <option value="auto">Automatic</option>
              <option value="page-actual">Actual size</option>
              <option value="page-fit">Page fit</option>
              <option value="page-width">Page width</option>
              <option value="0.5">50%</option>
              <option value="0.75">75%</option>
              <option value="1">100%</option>
              <option value="1.25">125%</option>
              <option value="1.5">150%</option>
              <option value="2">200%</option>
              <option value="3">300%</option>
              <option value="4">400%</option>
            </select>
          </div>
        </div>

        <div class="setting-row">
          <div class="meta">
            <div class="title">Collapsible reader toolbar</div>
            <div class="description">
              Slide the reader toolbar out of view while scrolling on small screens. Scroll up or tap the handle to
              bring it back.
            </div>
          </div>
          <div class="control">
            <Toggle bind:checked="{$collapsibleToolbarStore}" ariaLabel="Collapsible reader toolbar" />
          </div>
        </div>

        <div class="setting-row">
          <div class="meta">
            <div class="title">Use legacy reader</div>
            <div class="description">
              Open documents with the previous PDF reader. This preference is saved locally in your browser.
            </div>
          </div>
          <div class="control">
            <Toggle bind:checked="{$useLegacyReaderStore}" ariaLabel="Use legacy reader" />
          </div>
        </div>
      </div>

      <div class="setting-section">
        <p class="setting-heading">Privacy</p>

        {#if loading}
          <div class="loader">
            <SyncLoader size="3" color="var(--button-primary)" unit="rem" duration="1s" />
          </div>
        {:else}
          <div class="setting-row">
            <div class="meta">
              <div class="title">Enable Telemetry</div>
              <div class="description">
                Help improve Inkheart by sending anonymous instance telemetry (version, library size range,
                country-level location, and CPU architecture). This information helps guide development priorities and
                understand how Inkheart is used, without tracking individual users or documents.
                {#if telemetryConfigOverride}
                  <br />
                  <strong>Note:</strong> Telemetry has been disabled in the application configuration.
                {/if}
                <br />
                <button type="button" class="learn" on:click="{() => (showTelemetryInfo = true)}">Learn more</button>
              </div>
            </div>
            <div class="control">
              <Toggle
                bind:checked="{telemetryEnabled}"
                disabled="{telemetryConfigOverride || loading}"
                ariaLabel="Enable Telemetry"
              />
            </div>
          </div>
        {/if}
      </div>
    </div>

    <div class="footer">
      <span class="spacer"></span>
      <a href="https://ko-fi.com/U7U06VWIR" target="_blank" rel="noreferrer">
        <img
          height="36"
          style="border:0px;height:36px;"
          src="https://storage.ko-fi.com/cdn/kofi6.png?v=6"
          alt="Buy Me a Coffee at ko-fi.com"
        />
      </a>
    </div>
  </div>
</Modal>
