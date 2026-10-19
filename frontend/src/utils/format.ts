const MULTIPLICATION_SIGN = `\u00D7`

const formatPrecision = (num: number, precision: number) =>
  (Math.round(num * Math.pow(10, precision)) / Math.pow(10, precision)).toFixed(precision)

const formatBytes = (bytes: number): string => {
  if (bytes < 1024) {
    return `${bytes} B`
  }

  const kb = bytes / 1024

  if (kb < 1024) {
    return `${kb.toFixed(1)} KB`
  }

  return `${(kb / 1024).toFixed(1)} MB`
}

const formatPageSize = (width: number, height: number): string => {
  const inches = (value: number) => (value / 72).toFixed(2)
  const orientation = width > height ? 'landscape' : 'portrait'

  return `${inches(width)} ${MULTIPLICATION_SIGN} ${inches(height)} in (${orientation})`
}

// PDF dates are stored as `D:YYYYMMDDHHmmSS` with optional trailing timezone.
const formatPdfDate = (raw?: string): string => {
  if (!raw) {
    return ''
  }

  const match = /^D?:?(\d{4})(\d{2})?(\d{2})?(\d{2})?(\d{2})?(\d{2})?/.exec(raw)

  if (!match) {
    return ''
  }

  const [, year, month = '01', day = '01', hour = '00', minute = '00', second = '00'] = match
  const date = new Date(Number(year), Number(month) - 1, Number(day), Number(hour), Number(minute), Number(second))

  if (Number.isNaN(date.getTime())) {
    return ''
  }

  return date.toLocaleString()
}

export { formatPrecision, formatBytes, formatPageSize, formatPdfDate }
