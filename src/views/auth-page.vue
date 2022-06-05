<script
  setup
  lang="ts"
>
import { ref } from 'vue'
import { useStore } from 'vuex'
import { removeClient, initApp } from '../controllers/Client'

removeClient()

const store = useStore()

const accessKeyId = ref('')
const secretAccessKey = ref('')
const urlStyle = ref('')
const bucketName = ref('')
const region = ref('')
const endpoint = ref('')

const submit = (e: Event) => {
  e.preventDefault()
  const ep = endpoint.value ? endpoint.value : 'amazonaws.com'
  initApp(
    bucketName.value,
    accessKeyId.value,
    secretAccessKey.value,
    ep,
    region.value,
    urlStyle.value === 'path'
  )
    .then(() => {
      store.commit('reset')
      store.commit(
        'updateBaseUrl',
        (() => {
          if (urlStyle.value === 'path') return `${endpoint.value}/${bucketName.value}/`
          else if (endpoint.value.indexOf('aws') !== -1)
            return `https://${bucketName.value}.s3-${region.value}.${endpoint.value}/`
          else return `https://${bucketName.value}.${region.value}.${endpoint.value}/`
        })()
      )
      window.location.hash = '/'
    })
    .catch(console.error)
}
</script>
<template>
  <form
    class="bg-gray-300 flex flex-col p-4 rounded-xl"
    @submit="submit"
  >
    <label
      class="label-class"
      for="bucket_name"
    >
      Bucket name
    </label>
    <input
      id="bucket_name"
      v-model="bucketName"
      class="input-class"
      type="text"
      name="bucket_name"
      placeholder="the-best-bucket"
      required
    >

    <label
      class="label-class"
      for="access-key-id"
    >
      Access Key ID
    </label>
    <input
      id="access-key-id"
      v-model="accessKeyId"
      class="input-class"
      type="text"
      name="access_key_id"
      placeholder="abcdefghijklmnopqrst"
      required
    >

    <label
      class="label-class"
      for="secret-access-key"
    >
      Secret Access Key
    </label>
    <input
      id="secret-access-key"
      v-model="secretAccessKey"
      class="input-class"
      type="text"
      name="secret_access_key"
      placeholder="abcdefghijklmnopqrstuvwxyzabcdefghijklmn"
      required
    >

    <div class="mb-2">
      <span class="label-class">URL style:</span>
      <div>
        <input
          id="path-style"
          v-model="urlStyle"
          name="url_style"
          type="radio"
          value="path"
        >
        <label
          class="ml-2"
          for="path-style"
        >
          Path style
        </label>
      </div>
      <div>
        <input
          id="virtual-hosted-style"
          v-model="urlStyle"
          name="url_style"
          type="radio"
          value="virtual"
        >
        <label
          class="ml-2"
          for="virtual-hosted-style"
        >
          Virtual-hosted style
        </label>
      </div>
    </div>

    <label
      class="label-class"
      for="region"
    >
      Region
    </label>
    <input
      id="region"
      v-model="region"
      class="input-class"
      type="text"
      name="region"
      placeholder="s3-ap-southeast-1"
    >
    <label
      class="label-class"
      for="host"
    >
      Host (leave empty if using AWS)
    </label>
    <input
      id="host"
      v-model="endpoint"
      class="input-class"
      type="text"
      name="host"
      placeholder="digitaloceanspaces.com"
    >

    <button
      class="bg-slate-200 border-2 py-2 rounded-md hover:bg-gray-100"
      type="submit"
    >
      Login
    </button>
  </form>
</template>

<style scoped>
.label-class {
  @apply font-normal;
}
</style>
