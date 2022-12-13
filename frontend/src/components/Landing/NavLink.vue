<template>
  <RouterLink
    :to="props.to"
    class="inline-block rounded-lg py-1 px-2 text-sm text-slate-700 hover:bg-slate-100 hover:text-slate-900"
  >
    <slot />
  </RouterLink>
</template>

<script setup lang="ts">
const props = defineProps<{ to: string }>();
</script>
