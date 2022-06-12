<script
  setup
  lang="ts"
>
import { ref } from 'vue'
import TObjectHead from '../../types/TObjectHead'
import TFileNode from '../../types/TFileNode'
import { useStore } from '../../helpers/store'

const store = useStore()

defineProps<{
  fileMetadata: TObjectHead
  currentObj: TFileNode
}>()

const baseUrl = ref(store.state.baseUrl)
</script>

<template>
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
</style>
