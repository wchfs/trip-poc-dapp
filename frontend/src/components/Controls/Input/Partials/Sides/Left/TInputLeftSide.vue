<template>
  <span :class="[
    'inline-flex items-center rounded-l-md border border-r-0 border-gray-300 bg-gray-50 px-3 text-gray-500',
    props.size === 'sm' ? 'text-sm' : '',
    props.size === 'md' ? 'text-base' : '',
    props.size === 'lg' ? 'text-lg' : '',
    props.disabled ? 'cursor-not-allowed border-gray-200 bg-gray-50 text-gray-500' : '',
  ]">
    <slot />
  </span>
</template>

<script setup lang="ts">
const props = withDefaults(defineProps<{
  disabled?: boolean;
  size?: 'sm' | 'md' | 'lg';
}>(), {
  disabled: false,
  size: 'md',
});
</script>
