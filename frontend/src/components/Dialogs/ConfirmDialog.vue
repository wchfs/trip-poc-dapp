<template>
  <TransitionRoot
    as="template"
    :show="isOpen"
  >
    <Dialog
      as="div"
      class="relative z-[1000]"
      @close="close()"
    >
      <TransitionChild
        as="template"
        enter="ease-out duration-300"
        enter-from="opacity-0"
        enter-to="opacity-100"
        leave="ease-in duration-200"
        leave-from="opacity-100"
        leave-to="opacity-0"
      >
        <div
          class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"
        />
      </TransitionChild>

      <div
        class="fixed inset-0 z-auto overflow-y-auto"
      >
        <div
          class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0"
        >
          <TransitionChild
            as="template"
            enter="ease-out duration-300"
            enter-from="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
            enter-to="opacity-100 translate-y-0 sm:scale-100"
            leave="ease-in duration-200"
            leave-from="opacity-100 translate-y-0 sm:scale-100"
            leave-to="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
          >
            <DialogPanel
              class="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 w-full sm:max-w-lg"
            >
              <div
                class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4"
              >
                <div
                  class="sm:flex sm:items-start"
                >
                  <div
                    :class="`
                      mx-auto
                      flex
                      h-12
                      w-12
                      flex-shrink-0
                      items-center
                      justify-center
                      rounded-full
                      ${ getIconBgColorClasses(options.color) }
                      sm:mx-0
                      sm:h-10
                      sm:w-10
                    `"
                  >
                    <component
                      :is="options.icon"
                      :class="`h-6 w-6 ${ getIconColorClasses(options.color) }`"
                      aria-hidden="true"
                    />
                  </div>
                  <div
                    class="mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left"
                  >
                    <DialogTitle
                      as="h3"
                      class="text-lg font-medium leading-6 text-gray-900"
                    >
                      {{ options.title }}
                    </DialogTitle>
                    <div
                      class="mt-2"
                    >
                      <p
                        class="text-sm text-gray-500"
                      >
                        {{ options.message }}
                      </p>
                    </div>
                  </div>
                </div>
              </div>
              <div
                class="bg-gray-50 px-4 py-3 flex flex-col sm:flex-row-reverse sm:px-6 gap-2"
              >
                <TButton
                  @click="confirmClicked()"
                  :color="options.color"
                  class="w-full sm:w-auto"
                >
                  {{ options.confirmButtonText }}
                </TButton>
                <TButton
                  @click="close()"
                  color="white"
                  class="w-full sm:w-auto"
                >
                  {{ options.cancelButtonText }}
                </TButton>
              </div>
            </DialogPanel>
          </TransitionChild>
        </div>
      </div>
    </Dialog>
  </TransitionRoot>
</template>

<script setup lang="ts">
import { Dialog, DialogPanel, DialogTitle, TransitionChild, TransitionRoot } from '@headlessui/vue';
import { ExclamationTriangleIcon } from '@heroicons/vue/24/outline';
import { ref } from 'vue';
import TButton from '@/components/Controls/Button/TButton.vue';

type Events = {
  confirmed: () => void;
  canceled: () => void;
};

type Options = {
  icon: any,
  color: 'indigo' | 'red' | 'green' | 'yellow' | 'white';
  title?: string;
  message?: string;
  cancelButtonText?: string;
  confirmButtonText?: string;
};

const isOpen = ref(false);

const defaultOptions: Options = {
  icon: ExclamationTriangleIcon,
  color: 'red',
  title: 'Are you sure?',
  message: 'Are you confirm that action? This action cannot be undone.',
  confirmButtonText: 'Ok',
  cancelButtonText: 'Cancel',
};

const options: Options = {...defaultOptions};

const events = ref<Events | null>(null);

/**
 * Options can be passed by props or by params
 */
const open = (
  dialogEvents: Events,
  dialogOptions?: Partial<Options>
) => {
  Object.assign(options, defaultOptions, dialogOptions);

  events.value = dialogEvents;

  isOpen.value = true;
};

function confirmClicked() {
  events.value?.confirmed();
  close(false);
}

const close = (fireEvent: boolean = true): void => {
  if (fireEvent) {
    events.value?.canceled();
  }

  isOpen.value = false;
}

defineExpose({
  open,
  close,
});

const getIconBgColorClasses = (color: string): string => {
  switch (color) {
    case 'indigo':
      return `bg-indigo-100`;
    case 'red':
      return `bg-red-100`;
    case 'green':
      return `bg-green-100`;
    case 'yellow':
      return `bg-yellow-100`;
    case 'white':
    default:
      return `bg-gray-100`;
  }
}

const getIconColorClasses = (color: string): string => {
  switch (color) {
    case 'indigo':
      return `text-indigo-700`;
    case 'red':
      return `text-red-700`;
    case 'green':
      return `text-green-700`;
    case 'yellow':
      return `text-yellow-700`;
    case 'white':
    default:
      return `text-gray-700`;
  }
}
</script>
