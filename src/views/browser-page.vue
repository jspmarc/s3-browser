<script
  setup
  lang="ts"
>
import ObjectsPane from '../components/browser-page/objects-pane.vue'
import { ref, watch } from 'vue'

const folder = ref('')
const showingFile = ref(false)
const baseUrl = ref('')

watch(folder, (newF) => {
  console.log(baseUrl.value + newF)
})
</script>

<template>
  <h1>Browser</h1>
  folder: {{ folder }}
  <hr>
  <objects-pane
    :folder="folder"
    @open-folder="(f) => (folder = f)"
    @open-file="
      (f) => {
        folder = f
        showingFile = true
      }
    "
  />
  <iframe
    v-if="showingFile"
    :src="baseUrl + folder"
    frameborder="0"
  />
</template>
