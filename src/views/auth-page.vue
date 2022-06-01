<script
  setup
  lang="ts"
>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const accessKeyId = ref('')
const secretAccessKey = ref('')
const urlStyle = ref('path')
const bucketName = ref('')
const region = ref('')
const endpoint = ref('')

const submit = (e: Event) => {
  e.preventDefault()
  invoke('init_app', {
    name: bucketName.value,
    accessKeyId: accessKeyId.value,
    secretAccessKey: secretAccessKey.value,
    endpoint: endpoint.value,
    region: region.value,
    isPathStyle: urlStyle.value === 'path',
  })
    .then(() => {
      window.location.hash = '/'
    })
    .catch((e) => console.error(JSON.stringify(e, null, 4)))
}
</script>
<template>
  <h1>Auth</h1>
  <form
    class="flex flex-col"
    @submit="submit"
  >
    <label for="bucket_name">Bucket name</label>
    <input
      id="bucket_name"
      v-model="bucketName"
      type="text"
      name="bucket_name"
      placeholder="the-best-bucket"
      required
    >
    <label for="access-key-id">Access Key ID</label>
    <input
      id="access-key-id"
      v-model="accessKeyId"
      type="text"
      name="access_key_id"
      placeholder="abcdefghijklmnopqrst"
      required
    >

    <label for="secret-access-key">Secret Access Key</label>
    <input
      id="secret-access-key"
      v-model="secretAccessKey"
      type="text"
      name="secret_access_key"
      placeholder="abcdefghijklmnopqrstuvwxyzabcdefghijklmn"
      required
    >

    <span>AWS URL style:</span>
    <div>
      <input
        id="path-style"
        v-model="urlStyle"
        name="url_style"
        type="radio"
        value="path"
      >
      <label for="path-style">Path style</label>
    </div>

    <div>
      <input
        id="virtual-hosted-style"
        v-model="urlStyle"
        name="url_style"
        type="radio"
        value="virtual"
      >
      <label for="virtual-hosted-style">Virtual-hosted style</label>
    </div>

    <label for="region">Region</label>
    <input
      id="region"
      v-model="region"
      type="text"
      name="region"
      placeholder="s3-ap-southeast-1"
    >
    <label for="host">Host (leave empty if using AWS)</label>
    <input
      id="host"
      v-model="endpoint"
      type="text"
      name="host"
      placeholder="https://is3.cloudhost.id/"
    >

    <button type="submit">
      Submit
    </button>
  </form>
</template>
