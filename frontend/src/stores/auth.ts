import type { User } from 'firebase/auth'
import { writable } from 'svelte/store'

const auth = writable<User | null>(null)
const usingAuth = writable<boolean>(false)

export { auth, usingAuth }
