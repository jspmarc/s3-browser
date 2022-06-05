<script
  setup
  lang="ts"
>
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import Auth from './views/auth-page.vue'
import Browser from './views/browser-page.vue'
import NotFound from './views/not-found-page.vue'

const authRoute = '#/auth'

const currentPath = ref(authRoute)
const routes = {
  '/': Browser,
  '/auth': Auth,
}
const currentView = computed(() => {
  const route = currentPath.value.slice(1) || '/'
  if (route in routes) {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    //@ts-ignore
    return routes[route]
  }
  return NotFound
})
onMounted(() => {
  window.addEventListener('hashchange', () => {
    // if moving from auth to browser, check whether client has been
    // initialized or not. If initialized, redirect; else alert.
    if (window.location.hash !== authRoute) {
      invoke('has_client').then((hasClient) => {
        if (hasClient) {
          currentPath.value = window.location.hash
        } else {
          window.location.hash = authRoute
          // TODO: Find a phrase/word better (more fitting) than "login"
          alert('Please login first')
        }
      })
    } else {
      currentPath.value = window.location.hash
    }
  })
})
</script>

<template>
  <header class="bg-gray-400 grid grid-cols-1 h-[var(--header-height)]">
    <nav class="flex items-center justify-between px-4">
      <a href="#/">Files</a>
      <a :href="authRoute">Change S3</a>
    </nav>
  </header>
  <main
    class="flex flex-col items-center max-w-full min-h-[calc(100vh-var(--outer-content-height))] overflow-auto px-8 pt-1"
  >
    <component :is="currentView" />
  </main>
  <footer class="bg-gray-400 h-[var(--footer-height)] text-center w-screen">
    Created by
    <a
      href="https://jspmarc.dev"
      target="_blank"
    >
      Josep Marcello </a>. © Josep Marcello 2022.
  </footer>
</template>

<style scoped>
a {
  @apply bg-gray-400 h-full px-4 hover:bg-slate-200;
}
</style>

<style>
input[type='text'],
input[type='url'],
input[type='password'] {
  @apply border-2 border-gray-500 mb-2 px-2 rounded-sm w-[25rem];
}
</style>
