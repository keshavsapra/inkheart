const FADE_WIDTH = 20

export const overflowFade = (node: HTMLElement) => {
  let appliedMask = ''

  const applyMask = () => {
    const maxScroll = node.scrollWidth - node.clientWidth
    const fadeLeft = node.scrollLeft > 1
    const fadeRight = node.scrollLeft < maxScroll - 1

    let mask = ''

    if (fadeLeft && fadeRight) {
      mask = `linear-gradient(to right, transparent 0, black ${FADE_WIDTH}px, black calc(100% - ${FADE_WIDTH}px), transparent 100%)`
    } else if (fadeRight) {
      mask = `linear-gradient(to right, black calc(100% - ${FADE_WIDTH}px), transparent 100%)`
    } else if (fadeLeft) {
      mask = `linear-gradient(to right, transparent 0, black ${FADE_WIDTH}px)`
    }

    if (mask === appliedMask) {
      return
    }

    appliedMask = mask

    if (mask) {
      node.style.setProperty('-webkit-mask-image', mask)
      node.style.setProperty('mask-image', mask)
    } else {
      node.style.removeProperty('-webkit-mask-image')
      node.style.removeProperty('mask-image')
    }
  }

  node.style.setProperty('overflow-x', 'auto')
  node.style.setProperty('text-overflow', 'clip')
  node.style.setProperty('scrollbar-width', 'none')
  node.style.setProperty('overscroll-behavior-x', 'contain')

  const onScroll = () => applyMask()

  const resizeObserver = new ResizeObserver(() => applyMask())
  resizeObserver.observe(node)

  const mutationObserver = new MutationObserver(() => {
    node.scrollLeft = 0
    applyMask()
  })
  mutationObserver.observe(node, { childList: true, characterData: true, subtree: true })

  node.addEventListener('scroll', onScroll, { passive: true })
  applyMask()

  document.fonts.ready.then(() => applyMask())

  return {
    destroy() {
      node.removeEventListener('scroll', onScroll)
      resizeObserver.disconnect()
      mutationObserver.disconnect()
    },
  }
}
