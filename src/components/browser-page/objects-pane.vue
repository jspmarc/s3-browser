<script
  setup
  lang="ts"
>
import { ref, computed, watch } from 'vue'
import FileItem from './file-item.vue'
import type TObjectsList from '../../types/TObjectsList'
import { list } from '../../controllers/S3Object'

const props = defineProps({
  folder: {
    type: String,
    default: '',
  },
})

const files = ref<string[]>([])
const folders = ref<string[]>([])
const folder = ref(props.folder)
const searchQuery = ref('')

defineEmits<{
  (e: 'openFile', key: string): void
  (e: 'openFolder', folder: string): void
}>()

const lists = computed(() => {
  return {
    files: files.value,
    folders: folders.value,
  }
})

watch(props, (newValue) => {
  folder.value = newValue.folder
  getObjects()
})

const updateLists = (l: TObjectsList) => {
  files.value = l.files ?? []
  folders.value = l.folders ?? []
}

const getObjects = () => {
  list(folder.value)
    .then((v) => updateLists(v as TObjectsList))
    .catch(console.error)
}

getObjects()
</script>

<template>
  <div class="flex flex-col w-full">
    <input
      v-model="searchQuery"
      class="border-4 border-slate-500 px-2"
      type="text"
      placeholder="Search"
    >
    <ul>
      <file-item
        v-for="f in lists.folders.filter((f) => new RegExp(searchQuery, 'i').test(f))"
        :key="f"
        class="cursor-pointer"
        @click="$emit('openFolder', f)"
      >
        {{ f }}
      </file-item>
    </ul>
    <hr>
    <ul>
      <file-item
        v-for="f in lists.files.filter((f) => new RegExp(searchQuery, 'i').test(f))"
        :key="f"
        class="cursor-pointer"
        @click="$emit('openFile', f)"
      >
        {{ f }}
      </file-item>
    </ul>
  </div>
</template>
