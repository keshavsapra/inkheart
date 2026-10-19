interface GrabPanOptions {
  enabled: boolean
  onPanningChange?: (panning: boolean) => void
}

export const grabPan = (node: HTMLElement, options: GrabPanOptions) => {
  let current = options
  let panning = false
  let startX = 0
  let startY = 0
  let scrollStartX = 0
  let scrollStartY = 0

  const setPanning = (value: boolean) => {
    if (panning === value) {
      return
    }

    panning = value
    current.onPanningChange?.(value)
  }

  const onMouseDown = (event: MouseEvent) => {
    if (!current.enabled) {
      return
    }

    setPanning(true)
    startX = event.clientX
    startY = event.clientY
    scrollStartX = node.scrollLeft
    scrollStartY = node.scrollTop
    event.preventDefault()
  }

  const onMouseMove = (event: MouseEvent) => {
    if (!panning) {
      return
    }

    node.scrollLeft = scrollStartX - (event.clientX - startX)
    node.scrollTop = scrollStartY - (event.clientY - startY)
  }

  const onMouseUp = () => {
    setPanning(false)
  }

  node.addEventListener('mousedown', onMouseDown)
  window.addEventListener('mousemove', onMouseMove)
  window.addEventListener('mouseup', onMouseUp)

  return {
    update(options: GrabPanOptions) {
      current = options
    },
    destroy() {
      node.removeEventListener('mousedown', onMouseDown)
      window.removeEventListener('mousemove', onMouseMove)
      window.removeEventListener('mouseup', onMouseUp)
    },
  }
}
