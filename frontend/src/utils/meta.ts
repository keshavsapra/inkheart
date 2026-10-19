const getMeta = () => {
  const metas = document.getElementsByTagName('meta')

  const map: { [k: string]: string } = {}
  for (const meta of metas) {
    if (meta.getAttribute('name') && meta.getAttribute('name').startsWith('inkheart:')) {
      map[meta.getAttribute('name')] = meta.getAttribute('content')
    }
  }

  return map
}

export { getMeta }
