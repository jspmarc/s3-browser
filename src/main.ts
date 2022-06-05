import { createApp } from 'vue'
import { createStore } from 'vuex'
import App from './App.vue'
import './index.css'

const store = createStore({
  state() {
    return {
      baseUrl: '',
      keys: [''],
      isFolder: true,
    }
  },

  mutations: {
    addKey(state, key) {
      state.keys.push(key)
    },
    popKey(state) {
      state.keys.pop()
    },
    openFolder(state) {
      state.isFolder = true
    },
    openFile(state) {
      state.isFolder = false
    },
    updateBaseUrl(state, url) {
      state.baseUrl = url
    },
    reset(state) {
      state.baseUrl = ''
      state.keys = ['']
      state.isFolder = true
    },
  },
})

createApp(App).use(store).mount('#app')
