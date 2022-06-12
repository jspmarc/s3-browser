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

const files = ref<TObjectPut[]>([])

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
        files.value.push({
          path: f,
          // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
          key: currentKey(store) + f.split('/').at(-1)!,
          acl: 'Private',
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
    emit('close')
  }
}

const upload = async () => {
  if (files.value.length === 0) {
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
      title: 'Uploading successful',
    })
    emit('close')
  } catch (e) {
    const err = (e as { PutObjectsSomeFailed: { [key: string]: string } }).PutObjectsSomeFailed
    if (err) {
      let message = ''
      for (const key in err) {
        message += `${key},\n`
      }
      dialog.message(message, {
        title: 'Some Files Failed to Upload',
        type: 'error',
      })
      const failedFiles: TObjectPut[] = []
      for (const path in err) {
        failedFiles.push({
          path: path,
          key: path,
          acl: 'Private',
        })
      }
      files.value = failedFiles
    }
  }
}

const removeFile = (file: TObjectPut) => {
  const idx = files.value.findIndex((el) => el.path === file.path && el.key === file.key)
  files.value.splice(idx, 1)
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
        <span v-if="files.length === 0"> Select </span>
        <span v-else> Add </span>
        files
      </button>

      <ul class="overflow-auto w-full">
        <li
          v-for="f in files"
          :key="f.key"
          class="bg-white grid grid-cols-[9fr_1fr] grid-rows-3 my-2 px-2 py-1 rounded-md w-full"
        >
          <span>{{ f.path }}</span>
          <button
            class="bg-red-400 h-full rounded-md row-span-3 hover:bg-red-600 hover:text-white"
            @click="removeFile(f)"
          >
            d
          </button>
          <div class="input-container mb-2">
            <label :for="`file-key-${f.path}`">Key</label>
            <input
              :id="`file-key-${f.path}`"
              v-model.lazy="f.key"
              class="!m-0 !w-full"
              type="text"
            >
          </div>
          <div class="input-container">
            <label :for="`file-acl-${f.path}`">ACL</label>
            <select
              :id="`file-acl-${f.path}`"
              v-model="f.acl"
              class="!w-full"
            >
              <option value="AuthenticatedRead">
                Authenticated read
              </option>
              <option value="AwsExecRead">
                AWS exec read
              </option>
              <option value="BucketOwnerFullControl">
                Bucket owner full control
              </option>
              <option value="BucketOwnerRead">
                Bucket owner read
              </option>
              <option value="Private">
                Private
              </option>
              <option value="PublicRead">
                Public read
              </option>
              <option value="PublicReadWrite">
                Public read & write
              </option>
            </select>
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
.input-container {
  @apply flex flex-row items-center gap-4 px-2;
}
</style>
