<script
  setup
  lang="ts"
>
import { ref, computed, watch } from 'vue'
import FileItem from './file-item.vue'
import type TObjectsList from '../../types/TObjectsList'
import type TFileNode from '../../types/TFileNode'
import { list } from '../../controllers/S3Object'

const props = defineProps({
  folder: {
    type: String,
    default: '',
  },
})

const files = ref<TFileNode[]>([])
const folders = ref<TFileNode[]>([])
const currentFolderKey = ref(props.folder)
const searchQuery = ref('')

defineEmits<{
  (e: 'open', obj: TFileNode): void
}>()

const lists = computed(() => {
  return {
    files: files.value,
    folders: folders.value,
  }
})

watch(props, (newValue) => {
  currentFolderKey.value = newValue.folder
  getObjects()
})

const updateLists = (l: TObjectsList) => {
  files.value = l.files ?? []
  folders.value = l.folders ?? []
}

const getObjects = () => {
  list(currentFolderKey.value)
    .then((v) => updateLists(v as TObjectsList))
    .catch(console.error)
}

getObjects()
</script>

<template>
  <div class="flex flex-col w-full">
    <input
      v-model="searchQuery"
      class="!w-full"
      type="text"
      placeholder="Search"
    >
    <ul>
      <file-item
        v-for="f in lists.folders.filter((f) => new RegExp(searchQuery, 'i').test(f.name))"
        :key="f.name"
        :file="f"
        class="cursor-pointer"
        @open="$emit('open', f)"
      />
    </ul>
    <hr>
    <ul>
      <file-item
        v-for="f in lists.files.filter((f) => new RegExp(searchQuery, 'i').test(f.name))"
        :key="f.name"
        :file="f"
        class="cursor-pointer"
        @open="$emit('open', f)"
      />
    </ul>
  </div>
</template>
