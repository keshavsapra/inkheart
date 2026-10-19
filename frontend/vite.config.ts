import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import postcss from './postcss.config'
import { readFileSync } from 'fs'

interface VersionData {
  version?: string
  codename?: string
}

// Read version file
const versionContent = readFileSync('../VERSION', 'utf-8')
const versionData: VersionData = {}
versionContent.split('\n').forEach((line) => {
  const [key, value] = line.split('=')
  if (key && value) {
    versionData[key.trim() as keyof VersionData] = value.trim()
  }
})

// https://vitejs.dev/config/
export default defineConfig({
  css: {
    postcss,
  },
  plugins: [svelte()],
  base: process.env.BASE_PATH || './',
  publicDir: 'public',
  optimizeDeps: { exclude: ['svelte-navigator', 'pdfjs-dist'] },
  build: {
    target: 'esnext',
    rollupOptions: {
      output: {
        assetFileNames: (assetInfo) => {
          let extType = assetInfo.name?.split('.').at(1)
          if (/png|jpe?g|svg|gif|tiff|bmp|ico/i.test(extType ?? '')) {
            extType = 'img'
          }
          return `static/assets/${extType}/[name]-[hash][extname]`
        },
        chunkFileNames: 'static/assets/js/[name]-[hash].js',
        entryFileNames: 'static/assets/js/[name]-[hash].js',
      },
    },
  },
  server: {
    host: true,
    port: 5173,
  },
  define: {
    __APP_VERSION__: JSON.stringify(versionData.version),
    __APP_CODENAME__: JSON.stringify(versionData.codename),
  },
})
