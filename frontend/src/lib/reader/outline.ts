import type { PDFDocumentProxy } from 'pdfjs-dist'

export interface OutlineNode {
  title: string
  pageNumber: number | null
  dest: unknown
  items: OutlineNode[]
}

export const resolveDestPage = async (pdfDocument: PDFDocumentProxy, dest: unknown): Promise<number | null> => {
  let explicit = dest

  if (typeof dest === 'string') {
    explicit = await pdfDocument.getDestination(dest)
  }

  if (Array.isArray(explicit) && explicit[0]) {
    const index = await pdfDocument.getPageIndex(explicit[0])

    return index + 1
  }

  return null
}

// Returns the last outline entry whose page is at or before currentPage.
export const findCurrentNode = (nodes: OutlineNode[], currentPage: number): OutlineNode | null => {
  let activePage: number | null = null

  const walkPage = (items: OutlineNode[]) => {
    for (const node of items) {
      if (
        node.pageNumber !== null &&
        node.pageNumber <= currentPage &&
        (activePage === null || node.pageNumber > activePage)
      ) {
        activePage = node.pageNumber
      }

      walkPage(node.items)
    }
  }

  walkPage(nodes)

  if (activePage === null) {
    return null
  }

  let match: OutlineNode | null = null

  const walkNode = (items: OutlineNode[]) => {
    for (const node of items) {
      if (node.pageNumber === activePage) {
        match = node
      }

      walkNode(node.items)
    }
  }

  walkNode(nodes)

  return match
}

export const enrichOutline = async (pdfDocument: PDFDocumentProxy, items: any[]): Promise<OutlineNode[]> => {
  const result: OutlineNode[] = []

  for (const item of items) {
    let pageNumber: number | null = null

    try {
      pageNumber = await resolveDestPage(pdfDocument, item.dest)
    } catch (e) {
      pageNumber = null
    }

    result.push({
      title: item.title,
      dest: item.dest,
      pageNumber,
      items: await enrichOutline(pdfDocument, item.items || []),
    })
  }

  return result
}
