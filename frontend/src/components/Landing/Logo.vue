<template>
  <img
      src="/images/logos/webchefs-logo-blue.svg"
      height="128"
      width="128"
      alt="Webchefs"
      aria-hidden="true"
      :class="clsx(className)"
      unoptimized
    />
</template>

<script setup lang="ts">
import clsx from "clsx";

const props = defineProps<{ className?: string }>();
</script>
