<template>
  <div :class="clsx('mx-auto max-w-7xl px-4 sm:px-6 lg:px-8', className)">
    <slot />
  </div>
</template>

<script setup lang="ts">
import clsx from "clsx";

const props = defineProps<{ className?: string }>();
</script>
