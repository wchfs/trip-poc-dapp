<template>
  <Box :additionalClass="props.additionalClass">
    <div class="
        flex
        flex-row
        justify-between
        items-start
        my-auto
      ">
      <div class="flex flex-col overflow-hidden">
        <p class="text-xs text-gray-600 tracking-wide">
          {{ props.topText }}
        </p>
        <h3 :class="`
          mt-1
          text-lg
          ${props.textColor}
          text-ellipsis
          overflow-hidden
          whitespace-nowrap
        `">
          {{ props.featuredText }}
        </h3>
        <span class="
            mt-4
            text-xs
            text-gray-500
            text-ellipsis
            overflow-hidden
            whitespace-nowrap
            cursor-pointer
          ">
          {{ props.bottomText }}
        </span>
      </div>
      <div
        v-if="icon"
        :class="`
          bg-opacity-100
          border
          border-solid
          border-gray-200
          p-1
          md:p-2
          xl:p-3
          rounded-md
        `"
      >
        <i
          v-if="typeof icon === 'string' || icon instanceof String"
          :class="`
            ${icon}
            w-auto
            h-7
            w-7
            md:h-6
            md:w-6
            xl:h-8
            xl:w-8
            object-cover
            ${props.textColor}
          `"
        />
        <component
          v-else
          :is="icon"
          :class="`
            w-auto
            h-7
            w-7
            md:h-6
            md:w-6
            xl:h-8
            xl:w-8
            object-cover
            ${props.textColor}
          `"
        ></component>
      </div>
    </div>
    <div
      v-if="props.showButton"
      class="
        absolute
        left-0
        top-0
        backdrop-blur-sm
        bg-white/20
        w-full
        h-full
        flex
        justify-center
        items-center
      "
    >
      <TButton
        v-if="props.buttonText"
        @click="emit('buttonClick')"
        :color="props.buttonColor"
      >
        {{ props.buttonText }}
      </TButton>
    </div>
  </Box>
</template>

<script setup lang="ts">
import Box from '@/components/Box/Box.vue';
import TButton from '@/components/Controls/Button/TButton.vue';

const emit = defineEmits<{
  (e: 'buttonClick'): void;
}>();

const props = withDefaults(defineProps<{
  topText: string;
  featuredText: string;
  bottomText: string;
  icon: any;
  textColor?: string;
  showButton?: boolean;
  buttonType?: string;
  buttonText?: string;
  buttonColor?: 'indigo' | 'red' | 'green' | 'yellow' | 'white';
  additionalClass?: string;
}>(), {
  textColor: 'text-blue-500',
  showButton: false,
  buttonType: 'success',
  additionalClass: 'col-span-1',
  buttonText: 'Click',
  buttonColor: 'white',
});
</script>
