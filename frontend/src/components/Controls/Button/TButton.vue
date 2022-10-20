<template>
  <button
    :disabled="props.disabled"
    @click="emit('click')"
    :type="props.type"
    :class="`
      relative
      inline-flex
      items-center
      rounded-md
      border
      border-transparent
      px-4
      py-2
      text-sm
      font-medium
      shadow-sm
      disabled:opacity-50
      disabled:cursor-not-allowed
      disabled:drop-shadow-none
      hover:disabled:drop-shadow-none
      ${ props.color !== 'white' ? 'text-white' : 'text-gray-700' }
      ${ getColorClasses() }
      drop-shadow-sm
      active:drop-shadow-none
    `"
  >
    <slot/>
  </button>
</template>

<script setup lang="ts">
const props = withDefaults(defineProps<{
  color?: 'indigo' | 'red' | 'green' | 'yellow' | 'white',
  type?: 'button' | 'submit' | 'reset',
  disabled?: boolean,
}>(), {
  color: 'white',
  type: 'button',
  disabled: false,
});

const emit = defineEmits<{
  (e: 'click'): void;
}>();


const getColorClasses = (): string => {
  switch (props.color) {
    case 'indigo':
      return `
        bg-indigo-600
        hover:bg-indigo-700
      `;
    case 'red':
      return `
        bg-red-600
        hover:bg-red-700
      `;
    case 'green':
      return `
        bg-green-600
        hover:bg-green-700
      `;
    case 'yellow':
      return `
        bg-yellow-600
        hover:bg-yellow-700
      `;
    case 'white':
    default:
      return `
        bg-white
        border-gray-300
        hover:bg-gray-50
      `;
  }
}
</script>
