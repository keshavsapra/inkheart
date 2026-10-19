import { get } from 'svelte/store'
import { config } from '../config'
import { usingAuth } from '../stores/auth'
import { getToken } from './localstorage'

const createRequest = (
  endpoint: string,
  method: string,
  body?: unknown,
  headers?: { [key: string]: string }
): [string, RequestInit] => {
  const { api_base_url } = config

  const authHeader = get(usingAuth) ? { Authorization: `Bearer ${getToken()}` } : {}
  const headerObject = headers ? { ...headers, ...authHeader } : authHeader

  // Prepare the options object
  const options: RequestInit = {
    method: method.toUpperCase(),
    headers: Object.keys(headerObject).length > 0 ? new Headers(headerObject) : undefined,
  }

  if (body !== undefined) {
    options.body = body as BodyInit
  }

  const slashPrefix = endpoint.startsWith('/') ? '' : '/'
  const resource = `${api_base_url}${slashPrefix}${endpoint}`

  return [resource, options]
}

const createGetRequest = (endpoint: string, headers?: { [key: string]: string }): [string, RequestInit] => {
  return createRequest(endpoint, 'get', headers)
}
const createPostRequest = (
  endpoint: string,
  body?: unknown,
  headers?: { [key: string]: string }
): [string, RequestInit] => {
  return createRequest(endpoint, 'post', body, headers)
}

const createPatchRequest = (
  endpoint: string,
  body?: unknown,
  headers?: { [key: string]: string }
): [string, RequestInit] => {
  return createRequest(endpoint, 'patch', body, headers)
}

const createDeleteRequest = (
  endpoint: string,
  headers?: { [key: string]: string }
): [string, RequestInit] => {
  return createRequest(endpoint, 'delete', undefined, headers)
}

export { createRequest, createGetRequest, createPostRequest, createPatchRequest, createDeleteRequest }
