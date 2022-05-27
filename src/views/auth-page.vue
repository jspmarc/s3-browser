<script
  setup
  lang="ts"
>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const bucketUrl = ref('')
const accessKeyId = ref('')
const secretAccessKey = ref('')
const name = ref('')

const submit = (e: Event) => {
  e.preventDefault()
  invoke('new_s3', {
    bucketUrl: bucketUrl.value,
    accessKeyId: accessKeyId.value,
    secretAccessKey: secretAccessKey.value,
  }).then(() => {
    console.log('done creating S3')
  })
}

// Invoke the command
invoke('hello', { userName: 'world' }).then((n) => {
  if (typeof n === 'string') {
    console.log(n)
    name.value = n
  }
})
</script>
<template>
  name: {{ name }}
  <h1>Auth</h1>
  <form
    class="flex flex-col"
    @submit="submit"
  >
    <label for="bucket-url">Bucket URL</label>
    <input
      id="bucket-url"
      v-model="bucketUrl"
      type="url"
      name="bucket_url"
      placeholder="https://the-best-bucket.s3-ap-southeast-1.amazonaws.com"
      required
    >

    <label for="access-key-id">Access Key ID</label>
    <input
      id="access-key-id"
      v-model="accessKeyId"
      type="password"
      name="access_key_id"
      placeholder="abcdefghijklmnopqrst"
      required
    >

    <label for="secret-access-key">Secret Access Key</label>
    <input
      id="secret-access-key"
      v-model="secretAccessKey"
      type="password"
      name="secret_access_key"
      placeholder="abcdefghijklmnopqrstuvwxyzabcdefghijklmn"
      required
    >

    <button type="submit">
      Submit
    </button>
  </form>
</template>
