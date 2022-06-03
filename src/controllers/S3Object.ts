import { invoke } from '@tauri-apps/api'
import TObjectsList from '../types/TObjectsList'
import TObjectHead from '../types/TObjectHead'

export const list = async (prefix: string) => {
  try {
    const result: TObjectsList = await invoke('list_objects', { folder: prefix })
    return result
  } catch (error) {
    // TODO: handle error
    return { files: [], folders: [] }
  }
}

export const head = async (key: string) => {
  try {
    const result: TObjectHead = await invoke('head_object', { file: key })
    return result
  } catch (error) {
    // TODO: handle error
    return {
      content_type: 'application/octet-stream',
      size: 0,
    }
  }
}
