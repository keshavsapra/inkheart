import { writable } from 'svelte/store'

const createModalStack = () => {
  const { subscribe, update } = writable<symbol[]>([])

  return {
    subscribe,
    add: (id: symbol) => update((stack) => (stack.includes(id) ? stack : [...stack, id])),
    remove: (id: symbol) => update((stack) => stack.filter((entry) => entry !== id)),
  }
}

export const modalStack = createModalStack()
