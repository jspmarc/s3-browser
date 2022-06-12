import { InjectionKey } from 'vue'
import { createStore, useStore as baseUseStore, Store } from 'vuex'
import { defaultFolder } from '../types/TFileNode'
import TObjectsList from '../types/TObjectsList'
import TStore from '../types/TStore'

export const key: InjectionKey<Store<TStore>> = Symbol()

export const store = createStore<TStore>({
  state: {
    baseUrl: '',
    keys: [defaultFolder],
    objects: new Map<string, TObjectsList>(),
  },

  mutations: {
    addKey(state, key) {
      state.keys.push(key)
    },
    popKey(state) {
      state.keys.pop()
    },
    updateBaseUrl(state, url) {
      state.baseUrl = url
    },
    reset(state) {
      state.baseUrl = ''
      state.keys = [defaultFolder]
      state.objects.clear()
    },
  },
})

export const useStore = () => baseUseStore(key)

export const currentKey = () => {
  const { keys } = store.state

  let currentKey = keys[0].name
  keys.forEach((key, i) => {
    if (i == 0) return

    currentKey += key.name + '/'
  })

  return currentKey
}
