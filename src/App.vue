<script
  setup
  lang="ts"
>
import { ref, onMounted, computed } from 'vue'
import Auth from './views/auth-page.vue'
import Browser from './views/browser-page.vue'
import NotFound from './views/not-found-page.vue'

const currentPath = ref(window.location.hash)
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
    currentPath.value = window.location.hash
  })
})
</script>

<template>
  <header class="h-[var(--header-height)]">
    <nav>
      <a href="#/">Home</a>
      <a href="#/auth">Auth</a>
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
