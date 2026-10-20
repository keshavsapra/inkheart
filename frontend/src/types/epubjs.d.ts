declare module 'epubjs' {
  export interface NavItem {
    id?: string
    href: string
    label: string
    subitems?: NavItem[]
  }

  export interface Navigation {
    toc: NavItem[]
    landmarks?: any[]
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

  export interface RenditionHooks {
    content: {
      register(fn: (contents: any) => void): void
      trigger?(...args: any[]): Promise<any>
    }
    render?: {
      register(fn: (...args: any[]) => void): void
    }
    [key: string]: any
  }

  export interface Rendition {
    display(target?: string | number): Promise<void>
    next(): Promise<void>
    prev(): Promise<void>
    flow(flowName: string): void
    hooks: RenditionHooks
    on(event: string, listener: (...args: any[]) => void): void
    off(event: string, listener: (...args: any[]) => void): void
    themes: Themes
    destroy(): void
    location: RenditionLocation
    currentLocation(): RenditionLocation
    resize(width?: number | string, height?: number | string): void
  }

  export interface SpineSection {
    idref: string
    href: string
    index: number
    url?: string
    canonical?: string
    cfiBase?: string
    document?: Document
    load(request?: any): Promise<any>
    cfiFromElement(el: Element): string
    cfiFromRange(range: Range): string
    [key: string]: any
  }

  export interface BookSpine {
    get(target?: any): SpineSection | null
    spineItems: SpineSection[]
    length: number
    [key: string]: any
  }

  export interface BookLocations {
    generate(chars?: number): Promise<string[]>
    percentageFromCfi(cfi: string): number
    cfiFromPercentage(pct: number): string
    locationFromCfi(cfi: string): number
    total: number
  }

  export interface Book {
    opened: Promise<any>
    ready: Promise<any>
    loaded: {
      navigation: Promise<Navigation>
      metadata: Promise<any>
      spine: Promise<BookSpine>
    }
    navigation: Navigation
    spine: BookSpine
    locations: BookLocations
    path?: any
    canonical?(path: string): string
    resolve?(path: string, absolute?: boolean): string
    load?(target: any, type?: string): Promise<any>
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
