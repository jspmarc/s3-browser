<script
  setup
  lang="ts"
>
import { dialog, clipboard } from '@tauri-apps/api'
import type TFileNode from '../../types/TFileNode'
import { rm } from '../../controllers/S3Object'
import { useStore } from '../../helpers/store'

const store = useStore()

const emit = defineEmits<{
  (e: 'open'): void
  (e: 'delete'): void
}>()

const props = defineProps<{
  file: TFileNode
}>()

const editEv = (e: Event) => {
  e.stopPropagation()
  alert(`edit ${props.file.name} is todo`)
}

const rmEv = async (e: Event) => {
  e.stopPropagation()
  const conf = await dialog.confirm(`Are you sure you want to delete ${props.file.name}?`, {
    type: 'warning',
    title: 'Delete?',
  })
  if (conf) {
    try {
      await rm(props.file.s3_key)
      await dialog.message(`${props.file.name} is succesfully deleted`, {
        title: 'Success',
      })
      emit('delete')
    } catch (e) {
      console.error(e)
      if (typeof e === 'string') {
        dialog.message(e, {
          title: 'some upload failed',
          type: 'error',
        })
      } else {
        dialog.message(JSON.stringify(e), {
          title: 'some upload failed',
          type: 'error',
        })
      }
    }
  }
}

const copyEv = (e: Event) => {
  e.stopPropagation()
  const url: string = store.state.baseUrl
  clipboard.writeText(url + props.file.s3_key)
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
    <template v-if="!file.is_folder">
      <button
        class="bg-green-400 hover:bg-green-600 hover:text-white"
        @click="copyEv"
      >
        copy
      </button>
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
    </template>
  </li>
</template>

<style scoped>
button {
  @apply p-2 rounded-md;
}
</style>
