<template>
  <div class="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-3 px-4 xl:p-0 gap-4 xl:gap-6">
    <div class="col-span-1 md:col-span-2 lg:col-span-4 flex justify-between">
      <h2
        class="text-xs md:text-sm text-gray-700 font-bold tracking-wide md:tracking-wider"
        v-if="props.caption"
      >
        {{ props.caption }}
      </h2>
    </div>

    <slot/>
  </div>
</template>

<script setup lang="ts">
const props = defineProps({
  caption: String,
});
</script>
