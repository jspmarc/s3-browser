import { invoke } from '@tauri-apps/api'
import TObjectsList from '../types/TObjectsList'
import TObjectHead from '../types/TObjectHead'

export const list = async (prefix: string): Promise<TObjectsList> => {
  try {
    const result: TObjectsList = await invoke('list_objects', { prefix })
    return result
  } catch (error) {
    // TODO: handle error
    return { files: [], folders: [] }
  }
}

export const head = async (key: string) => {
  try {
    const result: TObjectHead = await invoke('head_object', { key })
    return result
  } catch (error) {
    // TODO: handle error
    return {
      content_type: 'application/octet-stream',
      size: 0,
    }
  }
}

export const rm = async (key: string) => {
  try {
    await invoke('delete_object', { key })
  } catch (error) {
    return
  }
}
