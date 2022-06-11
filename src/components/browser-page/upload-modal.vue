<script
  setup
  lang="ts"
>
import { dialog } from '@tauri-apps/api'
import { ref } from 'vue'
import { useStore } from 'vuex'
import { putMultiple } from '../../controllers/S3Object'
import type TObjectPut from '../../types/TObjectPut'
import { currentKey } from '../../helpers/store'

const store = useStore()

const files = ref<Set<TObjectPut>>(new Set())

const emit = defineEmits<(e: 'close') => void>()

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
          // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
          key: currentKey(store) + f.split('/').at(-1)!,
        })
      )
    })
}

const cancel = async () => {
  if (
    await dialog.confirm(
      'Are you sure you want to cancel?\nThis will remove all selected files from selection',
      {
        title: 'Cancel?',
        type: 'warning',
      }
    )
  ) {
    files.value.clear()
    emit('close')
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

  try {
    await putMultiple(Array.from(files.value))
    await dialog.message('All files have been uploaded', {
      type: 'info',
      title: 'uploading successful',
    })
    emit('close')
  } catch (e: any) {
    dialog.message(e, {
      title: 'some upload failed',
      type: 'error',
    })
  }
}
</script>

<template>
  <div
    class="absolute bg-black bg-opacity-90 flex flex-col h-[100vh] items-center justify-center overflow-hidden top-0 w-[100vw] z-50"
  >
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
          @click="cancel"
        >
          Cancel
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
          class="bg-white grid grid-cols-[9fr_1fr] grid-rows-2 my-2 px-2 py-1 rounded-md w-full"
        >
          <span>{{ f.path }}</span>
          <button
            class="bg-red-400 h-full rounded-md row-span-2 hover:bg-red-600 hover:text-white"
            @click="files.delete(f)"
          >
            d
          </button>
          <div class="flex flex-row gap-4 px-2">
            <label :for="`file-key-${f.path}`">Key</label>
            <input
              :id="`file-key-${f.path}`"
              v-model="f.key"
              class="!w-full"
              type="text"
            >
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>
