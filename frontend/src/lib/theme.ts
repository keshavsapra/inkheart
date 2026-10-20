import { get } from 'svelte/store'
import type { Theme } from '../stores/options'
import { selectedTheme } from '../stores/options'

const ALL_THEMES: string[] = [
  'light',
  'dark',
  'glassmorphism',
  'neomorphism',
  'aurora',
  'neo-brutalism',
  'cyberpunk',
  'monochrome',
]

const getSystemTheme = (): 'dark' | 'light' => {
  const systemTheme: 'dark' | 'light' =
    typeof window !== 'undefined' && window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches
      ? 'dark'
      : 'light'

  return systemTheme
}

const isDarkTheme = (theme: Theme): boolean => {
  if (theme === 'system') {
    return getSystemTheme() === 'dark'
  }
  return (
    theme === 'dark' ||
    theme === 'glassmorphism' ||
    theme === 'aurora' ||
    theme === 'cyberpunk' ||
    theme === 'monochrome'
  )
}

const setThemeOverride = (theme: Theme) => {
  if (typeof document === 'undefined') return
  const root = document.querySelector('html')
  if (!root) return

  ALL_THEMES.forEach((t) => root.classList.remove(t))

  if (theme === 'system') {
    const sys = getSystemTheme()
    root.classList.add(sys)
  } else {
    root.classList.add(theme)
  }
}

const initTheme = () => {
  const selected = get(selectedTheme)
  setThemeOverride(selected)

  selectedTheme.subscribe((theme) => {
    setThemeOverride(theme)
  })

  if (typeof window !== 'undefined' && window.matchMedia) {
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
      const current = get(selectedTheme)
      if (current === 'system') {
        setThemeOverride('system')
      }
    })
  }
}

const getActiveTheme = (): Theme => {
  const selected: Theme = get(selectedTheme)
  if (selected === 'system') {
    return getSystemTheme()
  }
  return selected
}

const updateTheme = (theme: Theme) => {
  selectedTheme.set(theme)
}

export { initTheme, getActiveTheme, isDarkTheme, updateTheme, selectedTheme }
