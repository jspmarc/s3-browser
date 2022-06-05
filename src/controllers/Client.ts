import { invoke } from '@tauri-apps/api'

export const removeClient = async () => {
  await invoke('remove_client')
}

export const initApp = async (
  bucketName: string,
  accessKeyId: string,
  secretAccessKey: string,
  endpoint: string,
  region: string,
  isPathStyle: boolean
) => {
  await invoke('init_app', {
    name: bucketName,
    accessKeyId,
    secretAccessKey,
    endpoint,
    region,
    isPathStyle,
  })
}
