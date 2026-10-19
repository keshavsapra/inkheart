import { get } from 'svelte/store'
import type { Theme } from '../stores/options'
import { selectedTheme } from '../stores/options'

const initTheme = () => {
  const selected = get(selectedTheme)

  setThemeOverride(selected)

  selectedTheme.subscribe((theme) => {
    setThemeOverride(theme)
  })
}

const setThemeOverride = (theme: Theme) => {
  const root = document.querySelector('html')
  if (theme === 'system') {
    root.classList.remove('light', 'dark')
  } else if (theme === 'dark') {
    root.classList.remove('light')
    root.classList.add('dark')
  } else if (theme === 'light') {
    root.classList.remove('dark')
    root.classList.add('light')
  }
}

const getSystemTheme = (): 'dark' | 'light' => {
  const systemTheme: Theme =
    window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'

  return systemTheme
}

const getActiveTheme = (): 'light' | 'dark' => {
  const systemTheme = getSystemTheme()
  const selected: Theme = get(selectedTheme)

  const active = selected === 'system' ? systemTheme : selected

  return active
}

const updateTheme = (theme: Theme) => {
  const systemTheme = getSystemTheme()

  const themeToSet = theme === systemTheme ? 'system' : theme

  selectedTheme.set(themeToSet)
}

export { initTheme, getActiveTheme, updateTheme, selectedTheme }
