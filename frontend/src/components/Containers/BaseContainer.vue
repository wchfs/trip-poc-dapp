<template>
  <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 px-4 xl:p-0 gap-4 xl:gap-6">
    <div
      v-if="props.caption"
      class="col-span-1 md:col-span-3 lg:col-span-3 flex justify-between"
    >
      <h2
        class="text-xs md:text-sm text-gray-700 font-bold tracking-wide md:tracking-wider"
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
