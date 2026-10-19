type ModeEnum = Record<string, number>

interface PresentationViewer {
  currentScaleValue: string
  scrollMode: number
  spreadMode: number
}

interface PresentationDeps {
  getViewer: () => PresentationViewer | null
  getContainer: () => HTMLElement | null
  getScrollModeEnum: () => ModeEnum | null
  getSpreadModeEnum: () => ModeEnum | null
  getUserScrollMode: () => string
  getUserSpreadMode: () => string
  getFallbackScale: () => string
  getPage: () => number
  getNumPages: () => number
  gotoPage: (page: number) => void
}

export const createPresentation = (deps: PresentationDeps) => {
  let active = false
  let prevScale = ''

  const next = () => deps.gotoPage(Math.min(deps.getPage() + 1, deps.getNumPages()))

  const prev = () => deps.gotoPage(Math.max(deps.getPage() - 1, 1))

  const onKeydown = (event: KeyboardEvent) => {
    if (!active) {
      return
    }

    if (event.key === 'ArrowRight' || event.key === 'ArrowDown' || event.key === 'PageDown' || event.key === ' ') {
      event.preventDefault()
      next()
    } else if (event.key === 'ArrowLeft' || event.key === 'ArrowUp' || event.key === 'PageUp') {
      event.preventDefault()
      prev()
    }
  }

  const onWheel = (event: WheelEvent) => {
    if (!active) {
      return
    }

    event.preventDefault()

    if (event.deltaY > 0) {
      next()
    } else if (event.deltaY < 0) {
      prev()
    }
  }

  const onFullscreenChange = () => {
    if (!document.fullscreenElement) {
      exit()
    }
  }

  const removeListeners = () => {
    window.removeEventListener('keydown', onKeydown)
    deps.getContainer()?.removeEventListener('wheel', onWheel)
    document.removeEventListener('fullscreenchange', onFullscreenChange)
  }

  const enter = async () => {
    const viewer = deps.getViewer()
    const container = deps.getContainer()
    const scrollEnum = deps.getScrollModeEnum()
    const spreadEnum = deps.getSpreadModeEnum()

    if (!viewer || !container || !scrollEnum || !spreadEnum) {
      return
    }

    prevScale = viewer.currentScaleValue

    await container.requestFullscreen()

    active = true
    viewer.scrollMode = scrollEnum.PAGE
    viewer.spreadMode = spreadEnum.NONE
    viewer.currentScaleValue = 'page-fit'

    window.addEventListener('keydown', onKeydown)
    container.addEventListener('wheel', onWheel, { passive: false })
    document.addEventListener('fullscreenchange', onFullscreenChange)
  }

  const exit = () => {
    active = false
    removeListeners()

    const viewer = deps.getViewer()
    const scrollEnum = deps.getScrollModeEnum()
    const spreadEnum = deps.getSpreadModeEnum()

    if (viewer && scrollEnum && spreadEnum) {
      viewer.scrollMode = scrollEnum[deps.getUserScrollMode().toUpperCase()]
      viewer.spreadMode = spreadEnum[deps.getUserSpreadMode().toUpperCase()]
      viewer.currentScaleValue = prevScale || deps.getFallbackScale()
    }
  }

  const destroy = () => {
    removeListeners()
  }

  return { enter, destroy }
}
