import { createApp } from 'vue'
import { createStore } from 'vuex'
import App from './App.vue'
import { defaultFolder } from './types/TFileNode'
import './index.css'

const store = createStore({
  state() {
    return {
      baseUrl: '',
      keys: [defaultFolder],
    }
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
    },
  },
})

createApp(App).use(store).mount('#app')
