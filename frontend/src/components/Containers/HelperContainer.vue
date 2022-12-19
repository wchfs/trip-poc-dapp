<template>
  <TDialog ref="dialogRef" />
  <div @click="openHelp()" class="fixed right-16 bottom-16 w-8 h-8 flex place-content-center items-center z-50">
    <div
      class="flex p-8 absulute w-full h-full items-center border-2 border-solid border-gray-100 bg-white place-content-center rounded-full shadow-md cursor-pointer z-20"
    >
      <div>
        <QuestionMarkCircleIcon class="text-gray-600 w-9 h-9" />
      </div>
    </div>
    <div class="p-6 absolute bg-gray-300 rounded-full z-10 animate-ping duration-1000" />
  </div>
</template>

<script setup lang="ts">
import { InformationCircleIcon, QuestionMarkCircleIcon } from '@heroicons/vue/24/outline';
import { ref } from 'vue';
import TDialog from '../Dialogs/TDialog.vue';

const dialogRef = ref<InstanceType<typeof TDialog> | null>(null);

const props = withDefaults(defineProps<{
  title?: string;
  message: string;
}>(), {
  title: 'Help for this page',
});

function openHelp() {
  dialogRef.value?.open({
    color: 'white',
    icon: InformationCircleIcon,
    title: props.title,
    message: props.message,
    buttons: [
      {
        text: 'Close',
        color: 'white',
        async clickCallback(): Promise<boolean> {
          return true;
        },
      },
    ],
  })
}
</script>
