<style>
  .body {
    width: 50rem;
    max-width: 90vw;
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
    padding: 20px 24px 24px;
    overflow-y: auto;
    flex: 1;
  }

  h3 {
    margin: 0 0 8px 0;
    font-size: var(--font-sm);
    font-weight: var(--weight-bold);
    color: var(--text-color-default);
  }

  p {
    margin: 0 0 16px 0;
    color: var(--text-color-default);
    font-size: var(--font-sm);
    line-height: 1.55;
  }

  p:last-child {
    margin-bottom: 0;
  }

  ul {
    margin: 0 0 16px 0;
    padding-left: 20px;
    color: var(--text-color-default);
    font-size: var(--font-sm);
    line-height: 1.55;
  }

  li {
    margin-bottom: 6px;
  }

  li::marker {
    color: var(--color-primary-500);
  }

  .section {
    margin-bottom: 20px;
  }

  .section:last-child {
    margin-bottom: 0;
  }

  .technical-note p {
    color: var(--text-color-light);
  }

  @media (max-width: 768px) {
    .body {
      width: 90vw;
      max-height: 85vh;
      max-height: 85dvh;
    }
  }
</style>

<script lang="ts">
  import Modal from '../modal/Modal.svelte'
  import Icon from '@iconify/svelte'
  import IconButton from '../button/IconButton.svelte'

  export let open = false

  const handleClose = () => {
    open = false
  }
</script>

<Modal bind:open="{open}">
  <div class="body">
    <div class="header">
      <h2>Privacy & Telemetry</h2>
      <IconButton onClick="{handleClose}" ariaLabel="Close">
        <Icon icon="mdi:close" width="18" />
      </IconButton>
    </div>

    <div class="content">
      <p>Inkheart sends anonymous usage statistics to count active installations and guide development priorities.</p>

      <div class="section">
        <h3>What is collected</h3>
        <ul>
          <li>Server ping when the index page is loaded</li>
          <li>Version number (e.g., "1.0.5")</li>
          <li>Library size bucket (e.g., "100-500 PDFs" - not exact count)</li>
          <li>Country-level location (derived from server IP; IP itself is not stored)</li>
          <li>CPU architecture (e.g., x86_64, arm64)</li>
        </ul>
      </div>

      <div class="section">
        <h3>What is NOT collected</h3>
        <ul>
          <li>Individual user pageviews or navigation</li>
          <li>File names, paths, or PDF contents</li>
          <li>User behavior or interaction tracking</li>
          <li>Any personally identifying information</li>
        </ul>
      </div>

      <div class="section technical-note">
        <h3>Technical note</h3>
        <p>
          Events are labeled as "pageview" for data visualization, but represent instance activity - not actual user
          browsing data.
        </p>
      </div>

      <div class="section technical-note">
        <p>Telemetry preferences can be changed at any time in Settings.</p>
      </div>
    </div>
  </div>
</Modal>
