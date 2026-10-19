declare module 'epubjs' {
  export interface NavItem {
    id: string
    href: string
    label: string
    subitems?: NavItem[]
  }

  export interface Navigation {
    toc: NavItem[]
  }

  export interface LocationItem {
    cfi: string
    displayed: {
      page: number
      total: number
    }
    href: string
    index: number
    percentage: number
  }

  export interface RenditionLocation {
    start: LocationItem
    end: LocationItem
    atStart?: boolean
    atEnd?: boolean
  }

  export interface Themes {
    register(name: string, styles: Record<string, any> | string): void
    select(name: string): void
    default(styles: Record<string, any> | string): void
    fontSize(size: string): void
  }

  export interface Rendition {
    display(target?: string | number): Promise<void>
    next(): Promise<void>
    prev(): Promise<void>
    on(event: string, listener: (...args: any[]) => void): void
    off(event: string, listener: (...args: any[]) => void): void
    themes: Themes
    destroy(): void
    location: RenditionLocation
    currentLocation(): RenditionLocation
    resize(width?: number | string, height?: number | string): void
  }

  export interface BookSpine {
    get(target: any): any
    spineItems: any[]
    length: number
  }

  export interface BookLocations {
    generate(chars?: number): Promise<string[]>
    percentageFromCfi(cfi: string): number
    cfiFromPercentage(pct: number): string
    total: number
  }

  export interface Book {
    opened: Promise<any>
    loaded: {
      navigation: Promise<Navigation>
      metadata: Promise<any>
      spine: Promise<BookSpine>
    }
    navigation: Navigation
    spine: BookSpine
    locations: BookLocations
    renderTo(element: HTMLElement | string, options?: Record<string, any>): Rendition
    destroy(): void
  }

  export interface EpubOptions {
    requestHeaders?: Record<string, string>
    [key: string]: any
  }

  function ePub(url?: string | ArrayBuffer, options?: EpubOptions): Book

  export default ePub
}
