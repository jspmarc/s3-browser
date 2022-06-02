<script
  setup
  lang="ts"
>
import { invoke } from '@tauri-apps/api'
import { ref, computed, watch } from 'vue'
import type TObjectsList from '../../types/TObjectsList'

const props = defineProps({
  folder: {
    type: String,
    default: '',
  },
})

const files = ref<string[]>([])
const folders = ref<string[]>([])
const folder = ref(props.folder)

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
  console.log({ newValue, folder })
  getObjects()
})

const updateLists = (l: TObjectsList) => {
  files.value = l.files
  folders.value = l.folders
}

const getObjects = () => {
  invoke('list_objects', { folder: folder.value })
    .then((v) => updateLists(v as TObjectsList))
    .catch(console.error)
}

getObjects()
</script>

<template>
  <ul>
    <li
      v-for="f in lists.folders"
      :key="f"
      class="cursor-pointer"
      @click="$emit('openFolder', f)"
    >
      {{ f }}
    </li>
  </ul>
  <hr>
  <ul>
    <li
      v-for="f in lists.files"
      :key="f"
      class="cursor-pointer"
      @click="$emit('openFile', f)"
    >
      {{ f }}
    </li>
  </ul>
</template>
