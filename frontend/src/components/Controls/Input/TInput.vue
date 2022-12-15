<template>
  <div>
    <label
      v-if="props.label || $slots.label"
      :for="controlName"
      :class="`
        block
        text-sm
        font-medium
        ${props.error ? 'text-red-700' : 'text-gray-700'}
        mb-1
      `"
    >
      <slot name="label">
        {{ props.label }}
        <span
          v-if="props.required"
          class="text-red-500"
        >*</span>
      </slot>
    </label>
    <div class="
      flex
      sm:text-sm
      shadow-sm
      ">
      <div class="
          relative
          flex
          flex-grow
          items-stretch
        ">
        <slot name="left" />
        <input
          v-if="props.type !== 'textarea'"
          :disabled="props.disabled"
          :required="props.required"
          :type="props.type"
          :name="controlName"
          :id="`input-${controlName}`"
          :placeholder="props.placeholder"
          v-model="modelValue"
          :autocomplete="props.autocomplete"
          :class="[
            `block w-full rounded-md`,
            slots.right ? 'rounded-r-none' : '',
            slots.left ? 'rounded-l-none' : '',
            props.error ? 'border-red-500' : 'border-gray-300',
            `focus:border-sky-500 focus:ring-sky-500 focus-within:z-10`,
            `disabled:cursor-not-allowed`,
            `disabled:border-gray-200 disabled:bg-gray-50 disabled:text-gray-500`,
            props.size === 'sm' ? 'text-sm' : '',
            props.size === 'md' ? 'text-base' : '',
            props.size === 'lg' ? 'text-lg' : '',
          ]"
        />
        <textarea
          v-else
          :disabled="props.disabled"
          :required="props.required"
          :name="controlName"
          :id="`input-${controlName}`"
          :placeholder="props.placeholder"
          :rows="props.rows"
          v-model="modelValue"
          :autocomplete="props.autocomplete"
          :class="`
            block
            w-full
            rounded-md
            ${props.error ? 'border-red-500' : 'border-gray-300'}
            focus:border-sky-500
            focus:ring-sky-500
            focus-within:z-10
            disabled:cursor-not-allowed
            disabled:border-gray-200
            disabled:bg-gray-50
            disabled:text-gray-500
          `"
        />
        <slot name="right" />
      </div>
    </div>
    <template v-if="props.error && typeof props.error === 'string'">
      <p class="mt-2 text-sm text-red-600">
        {{ props.error }}
      </p>
    </template>
    <template v-if="props.error && Array.isArray(props.error)">
      <p
        v-if="showAllErrors"
        v-for="error of props.error"
        class="mt-2 text-sm text-red-600"
      >
        {{ error }}
      </p>
      <p
        v-else
        class="mt-2 text-sm text-red-600"
      >
        {{ props.error[0] }}
        <span
          v-if="props.error.length > 1"
          @click="showAllErrors = !showAllErrors"
          class="inline-flex items-center rounded-full bg-red-600 px-2.5 py-0.5 text-xs font-medium text-white"
        >+{{ props.error.length - 1 }}</span>
      </p>
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, useSlots } from 'vue';
const controlName = Math.random().toString(36).substring(2, 10);

const showAllErrors = ref(false);
const slots = useSlots();

const modelValue = computed<string>({
  get() {
    if (props.modelValue) {
      return props.modelValue;
    }

    return '';
  },
  set(newValue) {
    emits('update:modelValue', newValue);
  },
});

const props = withDefaults(defineProps<{
  label?: string;
  placeholder?: string;
  type: 'text' | 'number' | 'email' | 'password' | 'textarea';
  rows?: number;
  modelValue: string | null;
  disabled?: boolean;
  required?: boolean;
  error?: string[] | string | boolean;
  autocomplete?: 'on' | 'off';
  size?: 'sm' | 'md' | 'lg';
  inputCallback?: (value: string) => string;
}>(), {
  disabled: false,
  required: false,
  error: false,
  rows: 5,
  autocomplete: 'off',
  size: 'md',
});

const emits = defineEmits<{
  (e: 'update:modelValue', v: string): void;
}>();
</script>