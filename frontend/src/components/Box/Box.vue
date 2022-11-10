<template>
  <div
    :class="`
      relative
      flex
      flex-col
      overflow-hidden
      items-stretch
      bg-white
      p-6
      rounded-xl
      border
      border-gray-50
      drop-shadow-md
      ${props.additionalClass}
     `"
  >
    <slot/>
  </div>
</template>

<script setup lang="ts">
const props = defineProps({
  additionalClass: {
    type: String,
    default: "col-span-1",
  },
});
</script>
