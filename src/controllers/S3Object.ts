import { invoke } from '@tauri-apps/api'
import type TObjectsList from '../types/TObjectsList'
import type TObjectHead from '../types/TObjectHead'
import type TObjectPut from '../types/TObjectPut'

export const list = async (prefix: string): Promise<TObjectsList> => {
  const result: TObjectsList = await invoke('list_objects', { key: prefix })
  return result
}

export const head = async (key: string) => {
  const result: TObjectHead = await invoke('head_object', { key })
  return result
}

export const rm = async (key: string) => {
  await invoke('delete_object', { key })
}

export const putMultiple = async (objects: TObjectPut[]) => {
  await invoke('put_multiple_objects', { objects })
}
