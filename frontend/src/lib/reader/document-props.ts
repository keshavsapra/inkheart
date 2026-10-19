import type { PDFDocumentProxy } from 'pdfjs-dist'
import { formatBytes, formatPageSize, formatPdfDate } from '../../utils/format'

export interface DocProps {
  fileName: string
  fileSize: string
  title: string
  author: string
  subject: string
  keywords: string
  creationDate: string
  modificationDate: string
  creator: string
  producer: string
  pdfVersion: string
  pages: string
  pageSize: string
  fastWebView: string
}

export const emptyDocProps = (): DocProps => ({
  fileName: '',
  fileSize: '',
  title: '',
  author: '',
  subject: '',
  keywords: '',
  creationDate: '',
  modificationDate: '',
  creator: '',
  producer: '',
  pdfVersion: '',
  pages: '',
  pageSize: '',
  fastWebView: '',
})

export const loadFileSize = (pdfDocument: PDFDocumentProxy): Promise<string> =>
  pdfDocument
    .getDownloadInfo()
    .then(({ length }: { length: number }) => `${formatBytes(length)} (${length.toLocaleString()} bytes)`)

export const loadDocumentProps = async (pdfDocument: PDFDocumentProxy): Promise<DocProps> => {
  const meta = await pdfDocument.getMetadata()
  const info = (meta?.info || {}) as {
    Title?: string
    Author?: string
    Subject?: string
    Keywords?: string
    Creator?: string
    Producer?: string
    CreationDate?: string
    ModDate?: string
    PDFFormatVersion?: string
    IsLinearized?: boolean
  }

  const props: DocProps = {
    fileName: (meta as unknown as { contentDispositionFilename?: string }).contentDispositionFilename || '',
    fileSize: '',
    title: info.Title || '',
    author: info.Author || '',
    subject: info.Subject || '',
    keywords: info.Keywords || '',
    creationDate: formatPdfDate(info.CreationDate),
    modificationDate: formatPdfDate(info.ModDate),
    creator: info.Creator || '',
    producer: info.Producer || '',
    pdfVersion: info.PDFFormatVersion || '',
    pages: String(pdfDocument.numPages),
    pageSize: '',
    fastWebView: info.IsLinearized ? 'Yes' : 'No',
  }

  try {
    const firstPage = await pdfDocument.getPage(1)
    const viewport = firstPage.getViewport({ scale: 1 })
    props.pageSize = formatPageSize(viewport.width, viewport.height)
  } catch (e) {
    // Page dimensions unavailable.
  }

  return props
}
