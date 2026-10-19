// firebase.ts
import { memoize } from 'lodash'
import { initializeApp, type FirebaseApp } from 'firebase/app'
import { getAuth, GoogleAuthProvider, signInWithPopup, onAuthStateChanged } from 'firebase/auth'
import { getMeta } from '../utils/meta'

export const initFirebase = memoize(() => {
  const metaTags = getMeta()

  // Try to parse the Firebase config
  let firebaseConfig = null
  try {
    firebaseConfig = JSON.parse(metaTags['inkheart:firebase-config'] || 'null')
  } catch (e) {
    console.error('Error parsing Firebase config:', e)
  }

  if (!firebaseConfig) {
    console.error('Firebase config not found or invalid')
    return { app: null, auth: null }
  }

  const app = initializeApp(firebaseConfig)
  const auth = getAuth(app)
  return { app, auth, onAuthStateChanged, GoogleAuthProvider, signInWithPopup }
}) as () => {
  app: ReturnType<typeof initializeApp>
  auth: ReturnType<typeof getAuth>
  onAuthStateChanged: typeof onAuthStateChanged
  GoogleAuthProvider: typeof GoogleAuthProvider
  signInWithPopup: typeof signInWithPopup
}
