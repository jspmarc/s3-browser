import { invoke } from '@tauri-apps/api'
import Client from '../types/TClient'

export const generateClient = async (): Promise<Client> => {
  const c: {
    access_key_id: string
    bucket_name: string
    secret_access_key: string
    endpoint: string
    region: string
    is_path_style: string
  } = await invoke('get_client_detail')

  return new Client(
    c.access_key_id,
    c.bucket_name,
    c.secret_access_key,
    c.endpoint,
    c.region,
    c.is_path_style === 'true'
  )
}
