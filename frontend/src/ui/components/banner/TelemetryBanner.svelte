<style>
  .banner {
    position: fixed;
    bottom: 1.5rem;
    left: 50%;
    transform: translateX(-50%);
    max-width: 60rem;
    width: calc(100% - 3rem);
    background: var(--card-subtle);
    border: 1px solid var(--card-color);
    border-radius: var(--radius-sm);
    padding: 18px 20px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 28px;
    z-index: 300;
    box-shadow: var(--elevation-2);
  }

  .message {
    flex: 1;
    min-width: 0;
    text-align: left;
  }

  .lead-paragraph {
    margin: 0;
    color: var(--text-color-default);
    font-size: var(--font-sm);
    line-height: 1.55;
    max-width: 72ch;
  }

  .subtext {
    margin: 8px 0 0;
    color: var(--text-color-light);
    font-size: var(--font-xs);
  }

  .actions {
    display: flex;
    gap: 0.75rem;
    align-items: center;
    flex-shrink: 0;
  }

  @media (max-width: 768px) {
    .banner {
      flex-direction: column;
      align-items: stretch;
      padding: 1rem;
      gap: 1rem;
    }

    .actions {
      flex-direction: column;
      width: 100%;
      align-items: stretch;
    }
  }
</style>

<script lang="ts">
  import { createPatchRequest } from '../../../lib/create-request'
  import { apiUrl } from '../../../utils/api'
  import TelemetryInfoModal from './TelemetryInfoModal.svelte'
  import Button from '../button/Button.svelte'

  export let onClose: () => void = () => {}

  let showInfoModal = false

  const handleLearnMore = () => {
    showInfoModal = true
  }

  const handleDisable = async () => {
    await updateTelemetrySetting(false)
    onClose()
  }

  const handleOkay = async () => {
    await updateTelemetrySetting(true)
    onClose()
  }

  const updateTelemetrySetting = async (enabled: boolean) => {
    try {
      const response = await fetch(
        ...createPatchRequest(
          apiUrl('api/settings'),
          JSON.stringify({ telemetry_enabled: enabled, telemetry_banner_shown: true }),
          { 'Content-Type': 'application/json' },
        ),
      )
      if (!response.ok) {
        console.error('Failed to update telemetry setting:', response.statusText)
      }
    } catch (e) {
      console.error('Failed to update telemetry setting:', e)
    }
  }
</script>

<div class="banner">
  <div class="message">
    <p class="lead-paragraph">
      Inkheart collects anonymous instance telemetry to help with development. Data collected includes: server activity,
      version, rough library size, country-level location, and CPU architecture. User data, files, and navigation are
      not tracked.
    </p>
    <p class="subtext">Telemetry can be toggled on or off in the Settings.</p>
  </div>
  <div class="actions">
    <Button variant="link" onClick="{handleLearnMore}">Learn More</Button>
    <Button variant="secondary" onClick="{handleDisable}">Disable</Button>
    <Button variant="primary" onClick="{handleOkay}">Okay</Button>
  </div>
</div>

<TelemetryInfoModal bind:open="{showInfoModal}" />
