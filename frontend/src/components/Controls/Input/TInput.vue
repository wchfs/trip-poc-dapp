<template>
  <div>
    <label
      v-if="props.label || $slots.label"
      :for="controlName"
      class="block text-sm font-medium text-gray-700 mb-1"
    >
      <slot name="label">
        {{ props.label }}
      </slot>
    </label>
    <div
      class="
      flex
      sm:text-sm
      shadow-sm
    "
    >
      <div
        class="
          relative
          flex
          flex-grow
          items-stretch
        "
      >
        <slot
          name="left"
        />
        <input
          :type="props.type"
          :name="controlName"
          :id="`input-${controlName}`"
          :placeholder="props.placeholder"
          :value="props.modelValue"
          @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
          :class="`
            block
            w-full
            rounded-md
            ${ slots.right ? 'rounded-r-none' : '' }
            ${ slots.left ? 'rounded-l-none' : '' }
            border-gray-300
            focus:border-indigo-500
            focus:ring-indigo-500
            focus-within:z-10
          `"
        />
        <slot
          name="right"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useSlots } from 'vue';

const slots = useSlots();
const controlName = Math.random().toString(36).substring(2,10);

const props = defineProps<{
  label?: string;
  placeholder?: string;
  type: 'text' | 'number' | 'email' | 'password' | 'textarea';
  modelValue: string;
}>();

const emits = defineEmits<{
  (e: 'update:modelValue'): void;
}>();
</script>
