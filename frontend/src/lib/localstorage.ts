const setToken = (token: string) => {
  localStorage.setItem('google_id', token)
}

const getToken = (): string => {
  return localStorage.getItem('google_id') || ''
}

const setUsingAuth = (flag: boolean) => {
  localStorage.setItem('using_auth', JSON.stringify(flag))
}

const getUsingAuth = (): string => {
  return JSON.parse(localStorage.getItem('google_id')) || false
}

export { setToken, getToken, setUsingAuth, getUsingAuth }
