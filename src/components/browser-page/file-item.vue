<script
  setup
  lang="ts"
>
import type TFileNode from '../../types/TFileNode'
import { rm } from '../../controllers/S3Object'

defineEmits<{
  (e: 'open'): void
}>()

const props = defineProps<{
  file: TFileNode
}>()

const editEv = (e: Event) => {
  e.stopPropagation()
  alert(`edit ${props.file.name} is todo`)
}

const rmEv = (e: Event) => {
  e.stopPropagation()
  rm(props.file.s3_key)
    .then(() => alert(`delete ${props.file.name} todo`))
    .catch(console.error)
}
</script>

<template>
  <li
    class="bg-white flex flex-row items-center gap-2 my-2 p-2 rounded-md w-full hover:bg-gray-100"
    @click="$emit('open')"
  >
    <span>✅</span>
    <span class="overflow-ellipsis overflow-hidden whitespace-nowrap w-full">
      {{ file.name }}
    </span>
    <button
      class="bg-blue-400 hover:bg-blue-600 hover:text-white"
      @click="editEv"
    >
      edit
    </button>
    <button
      class="bg-red-400 hover:bg-red-600 hover:text-white"
      @click="rmEv"
    >
      delete
    </button>
  </li>
</template>

<style scoped>
button {
  @apply p-2 rounded-md;
}
</style>
