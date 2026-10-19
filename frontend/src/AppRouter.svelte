<script>
  import { Router } from 'svelte-navigator'
  import App from './App.svelte'

  export let useAuth = false
  export let basepath = '/'
</script>

<Router basepath="{basepath}">
  <App useAuth="{useAuth}"></App>
</Router>
