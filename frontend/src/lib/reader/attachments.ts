import type { PDFDocumentProxy } from 'pdfjs-dist'

export interface AttachmentData {
  filename: string
  content: Uint8Array
}

export const loadAttachments = async (pdfDocument: PDFDocumentProxy): Promise<AttachmentData[]> => {
  try {
    const rawAttachments = await pdfDocument.getAttachments()

    if (!rawAttachments) {
      return []
    }

    return Object.values(rawAttachments).map((a: any) => ({ filename: a.filename, content: a.content }))
  } catch (e) {
    // No attachments, or they could not be read.
    return []
  }
}
