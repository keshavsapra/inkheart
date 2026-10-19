interface AppConfig {
  api_base_url: string
}

const config: AppConfig = {
  api_base_url: window.location.origin,
}

export { config }
