import type { Store } from 'vuex'
import type TFileNode from '../types/TFileNode'

export const currentKey = (store: Store<any>) => {
  const { keys }: { keys: TFileNode[]  } = store.state

  let currentKey = keys[0].name
  keys.forEach((key, i) => {
    if (i == 0)
      return

    currentKey += key.name + '/'
  })


  return currentKey
}
