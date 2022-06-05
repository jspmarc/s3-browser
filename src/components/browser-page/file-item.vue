<script
  setup
  lang="ts"
></script>

<template>
  <li
    class="bg-white flex flex-row items-center gap-2 my-2 p-2 rounded-md w-full hover:bg-gray-100"
  >
    <span>✅</span>
    <span class="overflow-ellipsis overflow-hidden whitespace-nowrap w-full">
      <slot />
    </span>
    <button class="bg-blue-400">
      edit
    </button>
    <button class="bg-red-400">
      delete
    </button>
  </li>
</template>

<style scoped>
button {
  @apply p-2 rounded-md;
}
</style>
