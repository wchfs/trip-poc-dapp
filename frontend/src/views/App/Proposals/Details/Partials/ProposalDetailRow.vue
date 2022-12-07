<template>
  <div
    :class="`
      py-4
      sm:grid
      sm:grid-cols-3
      sm:gap-4
      sm:py-5
      sm:px-6
      ${ props.color === 'white' ? 'bg-white' : '' }
      ${ props.color === 'green' ? 'bg-green-50' : '' }
      ${ props.color === 'red' ? 'bg-red-50' : '' }
      ${ props.color === 'indigo' ? 'bg-indigo-50' : '' }
    `"
  >
    <dt class="text-sm font-medium text-gray-500">
      {{ props.label}}
    </dt>
    <dd class="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">
      <slot/>
    </dd>
  </div>
</template>

<script setup lang="ts">
const props = withDefaults(defineProps<{
  label: string;
  color?: 'white' | 'red' | 'green' | 'indigo';
}>(), {
  color: 'white',
});
</script>
