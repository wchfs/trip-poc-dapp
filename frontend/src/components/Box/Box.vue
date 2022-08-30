<template>
  <div
    :class="`
      relative
      overflow-hidden
      bg-white
      p-6
      rounded-xl
      border
      border-gray-50
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
