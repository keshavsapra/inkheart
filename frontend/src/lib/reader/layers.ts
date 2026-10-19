export interface LayerNode {
  id?: string
  name: string
  visible?: boolean
  children?: LayerNode[]
}

export const buildLayers = (config: any): LayerNode[] => {
  if (!config) {
    return []
  }

  const build = (items: any[]): LayerNode[] => {
    const nodes: LayerNode[] = []

    for (const item of items) {
      if (typeof item === 'string') {
        const group = config.getGroup(item)
        nodes.push({ id: item, name: group?.name || item, visible: !!group?.visible })
      } else if (Array.isArray(item)) {
        nodes.push(...build(item))
      } else if (item && Array.isArray(item.order)) {
        nodes.push({ name: item.name || 'Additional Layers', children: build(item.order) })
      }
    }

    return nodes
  }

  return build(config.getOrder() || [])
}
