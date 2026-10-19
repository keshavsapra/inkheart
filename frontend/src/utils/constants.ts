const DEGREE_CHAR = '°'

const SCREEN_SIZE = {
  get mobile() {
    return window.getComputedStyle(document.documentElement).getPropertyValue('--small-viewport')
  },
}

export { DEGREE_CHAR, SCREEN_SIZE }
