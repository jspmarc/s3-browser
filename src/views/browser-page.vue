<script
  setup
  lang="ts"
>
import { ref, computed } from 'vue'
import { useStore } from 'vuex'
import ObjectsPane from '../components/browser-page/objects-pane.vue'
import { head } from '../controllers/S3Object'
import { generateClient } from '../controllers/Client'
import type TObjectHead from '../types/TObjectHead'

const store = useStore()

const fileMetadata = ref<TObjectHead | null>(null)
const baseUrl = ref('')

const currentKey = computed(() =>
  store.state.keys.reduce((combined: string, cur: string, idx: number) => {
    if (idx !== store.state.keys.length - 1) return combined + cur + '/'

    return combined + cur + (store.state.isFolder ? '/' : '')
  })
)

const openFile = (f: string) => {
  store.commit('addKey', f)
  store.commit('openFile')
  head(f)
    .then((r) => (fileMetadata.value = r))
    .catch(console.error)
}

const openFolder = (f: string) => {
  store.commit('addKey', f)
  store.commit('openFolder')
}
generateClient().then((client) => (baseUrl.value = client.generateUrl()))
</script>

<template>
  <div class="bg-gray-300 grid grid-cols-[1fr_9fr] mb-2 rounded-md w-full xl:grid-cols-[1fr_13fr]">
    <button
      class="bg-white mr-2 px-2 py-1 rounded-md hover:bg-slate-100"
      @click="
        () => {
          store.commit('popKey')
          fileMetadata = null
        }
      "
    >
      Back
    </button>
    <div
      class="bg-white px-2 inline-block overflow-ellipsis overflow-hidden whitespace-nowrap rounded-md w-full"
    >
      <span v-if="!fileMetadata">folder: </span>
      <span v-else>file: </span>
      <span class="w-full">{{ currentKey }}</span>
    </div>
  </div>
  <hr>
  <objects-pane
    v-if="!fileMetadata"
    :folder="currentKey"
    @open-folder="(f) => openFolder(f)"
    @open-file="(f) => openFile(f)"
  />
  <template v-else>
    <img
      v-if="fileMetadata.content_type.startsWith('image')"
      class="preview"
      :src="baseUrl + currentKey"
      :alt="currentKey"
    >
    <video
      v-else-if="fileMetadata.content_type.startsWith('video')"
      controls
      class="preview"
      :src="baseUrl + currentKey"
      :alt="currentKey"
      :type="fileMetadata.content_type"
    />
    <audio
      v-else-if="fileMetadata.content_type.startsWith('audio')"
      controls
      class="preview"
      :src="baseUrl + currentKey"
    />
    <iframe
      v-else-if="fileMetadata.content_type.startsWith('text')"
      class="preview no-w"
      :src="baseUrl + currentKey"
    />
    <iframe
      v-else
      class="preview no-w"
      :src="`https://docs.google.com/gview?url=${baseUrl + currentKey}&embedded=true`"
      frameborder="0"
    />
  </template>
</template>

<style scoped>
.preview {
  @apply bg-white max-h-[85vh] min-h-[80vh] w-auto;
}

.preview.no-w {
  @apply w-full;
}
</style>
