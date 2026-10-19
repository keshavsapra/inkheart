<style>
  .container {
    width: 15rem;
    height: 20rem;
    overflow: hidden;

    display: flex;
    flex-wrap: wrap;
  }

  img {
    width: 50%;
    height: 50%;
    object-fit: cover;
  }

  .details {
    padding: 0.5rem;
    color: var(--text-color-light);
    font-size: var(--font-lg);
    font-weight: var(--weight-body);
    width: calc(15rem - 1rem);

    display: flex;
    flex-direction: row;
    justify-content: space-between;
  }

  .title {
    font-size: var(--font-title);
    font-weight: var(--weight-body);
    padding-left: 0.3rem;

    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  @media (--small-viewport) {
    .container {
      max-width: 80vw;
    }
  }
</style>

<script lang="ts">
  import Card from './Card.svelte'

  export let src = ''
  export let title = ''
</script>

<Card padding="{false}">
  <div class="container">
    <img src="{src}" alt="cover" />
    <img src="{src}" alt="cover" />
    <img src="{src}" alt="cover" />
    <img src="{src}" alt="cover" />
  </div>
  {#if title}
    <div class="details"><span class="title">{title}</span></div>
  {/if}
</Card>
