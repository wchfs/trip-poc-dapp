<template>
  <span
    :class="`
      inline-block
      flex-shrink-0
      rounded-full
      px-2
      py-0.5
      text-xs
      font-medium
      ${ props.color === 'green' ? 'bg-green-100 text-green-800' : '' }
      ${ props.color === 'indigo' ? 'bg-indigo-100 text-indigo-800' : '' }
      ${ props.color === 'red' ? 'bg-red-100 text-red-800' : '' }
      ${ props.animation === 'pulse' ? 'animate-pulse' : '' }
    `"
  >
    <slot/>
  </span>
</template>

<script setup lang="ts">

const props = withDefaults(defineProps<{
  color?: 'white' | 'green' | 'indigo' | 'red';
  animation?: 'pulse' | 'none';
}>(), {
  color: 'indigo',
});
</script>
