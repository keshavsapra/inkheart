import { writable } from 'svelte/store'

export function createScrollCollapse(revealDistance = 200) {
  const showToolbar = writable(true)

  let scrollbox: HTMLElement | null = null
  let lastScrollPosition = 0
  let lowerScrollBound = 0
  let suppressUntil = 0

  const suppress = (durationMs = 300) => {
    suppressUntil = Date.now() + durationMs
  }

  const onScroll = () => {
    if (!scrollbox) {
      return
    }

    if (Date.now() < suppressUntil) {
      lastScrollPosition = scrollbox.scrollTop
      lowerScrollBound = scrollbox.scrollTop
      return
    }

    const current = scrollbox.scrollTop
    const distance = lowerScrollBound - current

    if (current > lastScrollPosition) {
      showToolbar.set(false)
      lowerScrollBound = current
    } else if (distance > revealDistance || current === 0) {
      showToolbar.set(true)
      lowerScrollBound = current
    }

    lastScrollPosition = current
  }

  const register = (node: HTMLElement) => {
    scrollbox = node
    node.addEventListener('scroll', onScroll)

    return {
      destroy() {
        node.removeEventListener('scroll', onScroll)
        scrollbox = null
      },
    }
  }

  const reset = () => {
    if (scrollbox && scrollbox.scrollTop === scrollbox.scrollHeight - scrollbox.offsetHeight) {
      scrollbox.scrollTop = scrollbox.scrollTop - revealDistance
    }

    showToolbar.set(true)
  }

  return { showToolbar, register, reset, suppress }
}
