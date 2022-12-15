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
        shadow-sm
      ">
      <div class="
          relative
          flex
          flex-grow
        ">
        <Listbox
          as="div"
          class="grow"
          v-model="modelValue"
          :disabled="props.disabled"
        >
          <div class="relative">
            <ListboxButton :class="`
              relative
              w-full
              cursor-default
              rounded-md
              border
              ${props.error ? 'border-red-500' : 'border-gray-300'}
              bg-white
              py-2
              pl-3
              pr-10
              text-left
              shadow-sm
              focus:border-sky-500
              focus:outline-none
              focus:ring-1
              focus:ring-sky-500
              disabled:border-gray-200
              disabled:bg-gray-50
              disabled:text-gray-500
            `">
              <span :class="[
                `block truncate`,
                props.size === 'sm' ? 'text-sm' : '',
                props.size === 'md' ? 'text-base' : '',
                props.size === 'lg' ? 'text-lg' : '',
              ]">
                {{ modelValue ? items.find((v) => v.value === modelValue)?.label : props.placeholder }}
              </span>
              <span class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2">
                <ChevronUpDownIcon
                  :class="`h-5 w-5 ${props.error ? 'text-red-500' : 'text-gray-400'}`"
                  aria-hidden="true"
                />
              </span>
            </ListboxButton>

            <transition
              leave-active-class="transition ease-in duration-100"
              leave-from-class="opacity-100"
              leave-to-class="opacity-0"
            >
              <ListboxOptions
                class="absolute z-10 mt-1 max-h-60 w-full overflow-auto rounded-md bg-white py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm"
              >
                <ListboxOption
                  as="template"
                  v-for="item of items"
                  :key="item.value"
                  :value="item.value"
                  v-slot="{ active, selected }"
                >
                  <li
                    :class="[active ? 'text-white bg-sky-600' : 'text-gray-900', 'relative cursor-default select-none py-2 pl-3 pr-9']"
                  >
                    <span :class="[
                      selected ? 'font-semibold' : 'font-normal',
                      'block truncate',
                      props.size === 'sm' ? 'text-sm' : '',
                      props.size === 'md' ? 'text-base' : '',
                      props.size === 'lg' ? 'text-lg' : '',
                    ]">
                      {{ item.label }}
                    </span>

                    <span
                      v-if="selected"
                      :class="[
                        active ? 'text-white' : 'text-sky-600',
                        'absolute inset-y-0 right-0 flex items-center pr-4',
                      ]"
                    >
                      <CheckIcon
                        class="h-5 w-5"
                        aria-hidden="true"
                      />
                    </span>
                  </li>
                </ListboxOption>
              </ListboxOptions>
            </transition>
          </div>
        </Listbox>
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
export interface SelectOption {
  value: string;
  label: string;
}

import { Listbox, ListboxButton, ListboxOption, ListboxOptions } from '@headlessui/vue';
import { CheckIcon, ChevronUpDownIcon } from '@heroicons/vue/20/solid';
import { computed, ref } from 'vue';

const emits = defineEmits<{
  (e: 'update:modelValue', v: string | undefined): void;
}>();

const controlName = Math.random().toString(36).substring(2, 10);
const showAllErrors = ref(false);

const modelValue = computed<string | undefined>({
  get() {
    return props.modelValue === null ? undefined : props.modelValue;
  },
  set(newValue) {
    if (newValue === null) {
      newValue = undefined;
    }

    emits('update:modelValue', newValue);
  },
});

const props = withDefaults(defineProps<{
  label?: string;
  modelValue?: string | null;
  placeholder?: string;
  disabled?: boolean;
  required?: boolean;
  error?: string[] | string | boolean;
  items: SelectOption[];
  size?: 'sm' | 'md' | 'lg';
}>(), {
  placeholder: 'Select an option',
  disabled: false,
  required: false,
  error: false,
  size: 'md',
});
</script>