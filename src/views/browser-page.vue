<script
  setup
  lang="ts"
>
import { ref, computed } from 'vue'
import ObjectsList from '../components/browser-page/objects-list.vue'
import ObjectPreviewer from '../components/browser-page/object-previewer.vue'
import UploadModal from '../components/browser-page/upload-modal.vue'
import { head } from '../controllers/S3Object'
import { useStore } from '../helpers/store'
import type TFileNode from '../types/TFileNode'
import type TObjectHead from '../types/TObjectHead'

const store = useStore()

const fileMetadata = ref<TObjectHead | null>(null)
const uploading = ref(false)

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const currentObj = computed<TFileNode>(() => store.state.keys.at(-1)!)

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

  <objects-list
    v-if="!fileMetadata"
    :folder="currentObj.s3_key"
    @open="(f) => open(f)"
  />
  <object-previewer
    v-else
    :file-metadata="fileMetadata"
    :current-obj="currentObj"
  />

  <upload-modal
    v-if="uploading"
    @close="() => (uploading = false)"
  />
</template>

<style scoped>
button[disabled] {
  @apply bg-gray-500 cursor-not-allowed text-white hover:bg-gray-500;
}
</style>
