<style>
  main {
    /* max-width: calc(100vw - 5rem); */
    padding: 2rem;
    width: 100%;
    height: 80vh;

    display: flex;
    justify-content: center;
    align-items: center;
  }

  .card-container {
    width: 15rem;
  }

  .card-content {
    display: flex;
    flex-direction: column;
    justify-content: space-evenly;
    align-items: center;
  }

  @media (--small-viewport) {
    main {
      width: 100vw;
    }
  }
</style>

<script>
  import Card from '../card/Card.svelte'

  export let message = 'Something went wrong'
</script>

<main>
  <div class="card-container">
    <Card>
      <div class="card-content">
        <h3>Error</h3>
        <span>{message}</span>
      </div>
    </Card>
  </div>
</main>
