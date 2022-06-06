<script
  setup
  lang="ts"
>
import { dialog } from '@tauri-apps/api'
import { ref } from 'vue'
import { putMultiple } from '../../controllers/S3Object'
import type TObjectUpload from '../../types/TObjectUpload'

const files = ref<Set<TObjectUpload>>(new Set())

defineEmits<(e: 'close') => void>()

const addFiles = () => {
  dialog
    .open({
      title: 'Select files to upload',
      multiple: true,
    })
    .then((f) => {
      if (!f) return

      if (typeof f === 'string') f = [f]

      f.forEach((f) =>
        files.value.add({
          path: f,
          key: f,
        })
      )
    })
}

const removeAll = async () => {
  if (
    await dialog.confirm('Are you sure you want to remove all files?', {
      title: 'remove all files?',
      type: 'warning',
    })
  ) {
    files.value.clear()
  }
}

const upload = async () => {
  if (files.value.size === 0) {
    await dialog.message('Please select at least one file', {
      type: 'error',
      title: 'no files? :(',
    })
    return
  }

  const yes = await dialog.confirm('Are you sure you want to upload all files now?', {
    title: 'upload now?',
    type: 'info',
  })

  if (!yes) return

  putMultiple(Array.from(files.value)).catch((e: any) =>
    dialog.message(e, {
      title: 'some upload failed',
      type: 'error',
    })
  )
}
</script>

<template>
  <div
    class="absolute bg-black bg-opacity-90 flex flex-col h-[100vh] items-center justify-center overflow-hidden top-0 w-[100vw] z-50"
  >
    <button
      class="bg-red-400 text-center mb-2 px-2 py-1 rounded-md hover:bg-red-600 hover:text-white"
      @click="$emit('close')"
    >
      X
    </button>

    <div class="bg-slate-400 flex flex-col items-center h-3/4 p-2 rounded-md w-3/4">
      <div class="mb-2">
        <button
          class="bg-blue-400 mr-4 px-2 py-1 rounded-md hover:bg-blue-600 hover:text-white"
          @click="upload"
        >
          Upload
        </button>
        <button
          class="bg-red-400 px-2 py-1 rounded-md hover:bg-red-600 hover:text-white"
          @click="removeAll"
        >
          Remove all
        </button>
      </div>

      <button
        class="bg-slate-200 mb-4 px-2 py-1 rounded-md hover:bg-slate-100"
        @click="addFiles"
      >
        <span v-if="files.size === 0"> Select </span>
        <span v-else> Add </span>
        files
      </button>

      <ul class="overflow-auto w-full">
        <li
          v-for="f in files"
          :key="f.key"
          class="bg-white grid grid-cols-[9fr_1fr] my-2 px-2 py-1 rounded-md w-full"
        >
          <span>{{ f.path }}</span>
          <button
            class="bg-red-400 h-full rounded-md hover:bg-red-600 hover:text-white"
            @click="files.delete(f)"
          >
            d
          </button>
        </li>
      </ul>
    </div>
  </div>
</template>
