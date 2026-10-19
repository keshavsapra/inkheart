import type { PDFDocumentProxy } from 'pdfjs-dist'

export const renderThumbnail = async (
  pdfDocument: PDFDocumentProxy,
  pageNumber: number,
  container: HTMLElement,
): Promise<void> => {
  const pdfPage = await pdfDocument.getPage(pageNumber)
  const scale = 160 / pdfPage.getViewport({ scale: 1 }).width
  const viewport = pdfPage.getViewport({ scale })

  const canvas = document.createElement('canvas')
  canvas.width = Math.floor(viewport.width)
  canvas.height = Math.floor(viewport.height)

  const ctx = canvas.getContext('2d')

  if (!ctx) {
    return
  }

  container.innerHTML = ''
  container.appendChild(canvas)

  await pdfPage.render({ canvas, canvasContext: ctx, viewport }).promise
}
