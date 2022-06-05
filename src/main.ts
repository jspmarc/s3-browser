import { createApp } from 'vue'
import { createStore } from 'vuex'
import App from './App.vue'
import './index.css'

const store = createStore({
  state() {
    return {
      keys: [''],
    }
  },

  mutations: {
    addKey(state, key) {
      state.keys.push(key)
    },
    popKey(state) {
      state.keys.pop()
    },
  },
})

createApp(App).use(store).mount('#app')
