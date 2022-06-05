<script
  setup
  lang="ts"
>
import ObjectsPane from '../components/browser-page/objects-pane.vue'
import { head } from '../controllers/S3Object'
import { generateClient } from '../controllers/Client'
import { ref } from 'vue'
import type TObjectHead from '../types/TObjectHead'

const visited = ref<string[]>([''])
const fileMetadata = ref<TObjectHead | null>(null)
const baseUrl = ref('')

const openFile = (f: string) => {
  visited.value.push(f)
  head(f)
    .then((r) => (fileMetadata.value = r))
    .catch(console.error)
}
generateClient().then((client) => (baseUrl.value = client.generateUrl()))
</script>

<template>
  folder: {{ visited.at(-1) }}
  <button
    @click="
      () => {
        visited.pop()
        fileMetadata = null
      }
    "
  >
    Back
  </button>
  <hr>
  <objects-pane
    v-if="!fileMetadata"
    :folder="visited.at(-1)"
    @open-folder="(f) => visited.push(f)"
    @open-file="(f) => openFile(f)"
  />
  <template v-if="fileMetadata">
    <img
      v-if="fileMetadata.content_type.startsWith('image')"
      :src="baseUrl + visited.at(-1)"
      :alt="visited.at(-1)"
    >
    <video
      v-else-if="fileMetadata.content_type.startsWith('video')"
      controls
      :src="baseUrl + visited.at(-1)"
      :alt="visited.at(-1)"
      :type="fileMetadata.content_type"
    />
    <audio
      v-else-if="fileMetadata.content_type.startsWith('audio')"
      controls
      :src="baseUrl + visited.at(-1)"
    />
    <iframe
      v-else-if="fileMetadata.content_type.startsWith('text')"
      :src="baseUrl + visited.at(-1)"
    />
    <iframe
      v-else
      :src="`https://docs.google.com/gview?url=${baseUrl + visited.at(-1)}&embedded=true`"
      style="width: 600px; height: 500px"
      frameborder="0"
    />
  </template>
</template>
