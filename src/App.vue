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
  <header class="h-[var(--header-height)]">
    <nav>
      <a href="#/">Home</a>
      <a :href="authRoute">Auth</a>
    </nav>
  </header>
  <main class="min-h-[calc(100vh-var(--outer-content-height))]">
    <component :is="currentView" />
  </main>
  <footer class="h-[var(--footer-height)]">
    Created by
    <a
      href="https://jspmarc.dev"
      target="_blank"
    >
      Josep Marcello </a>. © Josep Marcello 2022.
  </footer>
</template>
