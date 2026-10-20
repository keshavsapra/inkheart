const getMeta = () => {
  const metas = document.getElementsByTagName('meta')

  const map: { [k: string]: string } = {}
  for (const meta of metas) {
    const name = meta.getAttribute('name')
    if (name) {
      if (name.startsWith('nextchapter:') || name.startsWith('inkheart:')) {
        const val = meta.getAttribute('content') || ''
        map[name] = val
        if (name.startsWith('inkheart:')) {
          map[name.replace('inkheart:', 'nextchapter:')] = val
        } else if (name.startsWith('nextchapter:')) {
          map[name.replace('nextchapter:', 'inkheart:')] = val
        }
      }
    }
  }

  return map
}

export { getMeta }
