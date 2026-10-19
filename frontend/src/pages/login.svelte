<style>
  main {
    /*max-width: calc(100vw - 5rem);*/
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
    align-items: stretch;
    height: 10rem;
  }

  @media (--small-viewport) {
    main {
      width: 100vw;
    }
  }
</style>

<script lang="ts">
  import { onMount } from 'svelte'
  import { initFirebase } from '../lib/firebase'
  import { auth as authStore } from '../stores/auth'
  import Card from '../ui/components/card/Card.svelte'
  import Button from '../ui/components/button/Button.svelte'
  import { setToken } from '../lib/localstorage'
  import { useNavigate } from 'svelte-navigator'

  const navigate = useNavigate()

  onMount(() => {
    return authStore.subscribe((user) => {
      if (user) {
        user.getIdToken().then((idToken) => {
          setToken(idToken)
          navigate('/')
        })
      }
    })
  })

  const handleSignInButton = () => {
    const { auth, GoogleAuthProvider, signInWithPopup } = initFirebase()

    const provider = new GoogleAuthProvider()

    signInWithPopup(auth, provider)
      .then((result) => {})
      .catch((error) => {
        const errorCode = error.code
        const errorMessage = error.message

        console.log(errorCode, errorMessage)
        // ...
      })
  }
</script>

<main>
  <div class="card-container">
    <Card>
      <div class="card-content">
        <span>Sign in with Google</span>
        <Button variant="primary" onClick="{handleSignInButton}">Sign-in</Button>
      </div>
    </Card>
  </div>
</main>
