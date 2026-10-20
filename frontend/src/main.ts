import './theme.css'
import './fonts.css'
import './app.css'
import { getMeta } from './utils/meta'
import AppRouter from './AppRouter.svelte'
import { initTheme } from './lib/theme'
import { basepath as basepathStore } from './stores/basepath'

initTheme()

const metaTags = getMeta()

const useAuth =
  metaTags['nextchapter:use-auth'] === '$NC:USEAUTH' || metaTags['inkheart:use-auth'] === '$INK:USEAUTH'
    ? false
    : metaTags['nextchapter:use-auth'] === 'true' || metaTags['inkheart:use-auth'] === 'true'

const basepath =
  metaTags['nextchapter:basepath'] === '$NC:BASEPATH' || metaTags['inkheart:basepath'] === '$INK:BASEPATH'
    ? '/'
    : metaTags['nextchapter:basepath'] || metaTags['inkheart:basepath'] || '/'

basepathStore.set(basepath)

const app = new AppRouter({
  target: document.getElementById('app'),
  props: { useAuth, basepath },
})

export default app
