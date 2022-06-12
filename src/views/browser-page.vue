<script
  setup
  lang="ts"
>
import { ref, computed } from 'vue'
import { useStore } from 'vuex'
import ObjectsPane from '../components/browser-page/objects-pane.vue'
import UploadModal from '../components/browser-page/upload-modal.vue'
import { head } from '../controllers/S3Object'
import type TFileNode from '../types/TFileNode'
import type TObjectHead from '../types/TObjectHead'

const store = useStore()

const fileMetadata = ref<TObjectHead | null>(null)
const baseUrl = ref(store.state.baseUrl)
const uploading = ref(false)

const currentObj = computed<TFileNode>(() => store.state.keys.at(-1))

const open = (f: TFileNode) => {
  if (!f.is_folder) {
    head(f.s3_key)
      .then((r) => (fileMetadata.value = r))
      .catch(console.error)
  }
  store.commit('addKey', f)
}

const upload = () => {
  uploading.value = true
}
</script>

<template>
  <div
    class="bg-gray-300 grid grid-cols-[1fr_8fr_1fr] mb-2 rounded-md w-full xl:grid-cols-[1fr_18fr_1fr]"
  >
    <button
      class="bg-white px-2 py-1 rounded-md hover:bg-slate-100"
      :disabled="store.state.keys.length === 1"
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
      class="bg-white inline-block mx-2 overflow-ellipsis overflow-hidden px-2 whitespace-nowrap rounded-md"
    >
      <span v-if="!fileMetadata">folder: </span>
      <span v-else>file: </span>
      <span class="w-full">{{ currentObj.name ?? 'undefined' }}</span>
    </div>

    <button
      class="bg-white px-2 py-1 rounded-md hover:bg-slate-100"
      :disabled="!currentObj.is_folder"
      @click="upload"
    >
      Upload
    </button>
  </div>
  <hr>
  <objects-pane
    v-if="!fileMetadata"
    :folder="currentObj.s3_key"
    @open="(f) => open(f)"
  />
  <template v-else>
    <img
      v-if="fileMetadata.content_type.startsWith('image')"
      class="preview media"
      :src="baseUrl + currentObj.s3_key"
      :alt="`${currentObj.name} failed to load.`"
    >
    <video
      v-else-if="fileMetadata.content_type.startsWith('video')"
      controls
      class="preview media"
      :src="baseUrl + currentObj.s3_key"
      :alt="currentObj"
      :type="fileMetadata.content_type"
    />
    <audio
      v-else-if="fileMetadata.content_type.startsWith('audio')"
      controls
      class="preview media"
      :src="baseUrl + currentObj.s3_key"
    />
    <iframe
      v-else-if="fileMetadata.content_type.startsWith('text')"
      class="preview no-w"
      :src="baseUrl + currentObj.s3_key"
    />
    <iframe
      v-else
      class="preview no-w"
      :src="`https://docs.google.com/gview?url=${baseUrl + currentObj.s3_key}&embedded=true`"
      frameborder="0"
    />
  </template>

  <upload-modal
    v-if="uploading"
    @close="() => (uploading = false)"
  />
</template>

<style scoped>
.preview {
  @apply bg-white max-h-[85vh] min-h-[80vh] rounded-md w-auto;
}

.preview.media {
  @apply bg-transparent object-contain;
}

.preview.no-w {
  @apply w-full;
}

button[disabled] {
  @apply bg-gray-500 cursor-not-allowed text-white hover:bg-gray-500;
}
</style>
