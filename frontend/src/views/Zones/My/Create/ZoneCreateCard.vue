<template>
  <div class="space-y-8">
    <div>
      <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
        <form class="sm:col-span-6 grid grid-cols-2 gap-y-6 gap-x-4 sm:grid-cols-12">
          <TInput
            class="col-span-2 sm:col-span-6 sm:col-start-4"
            v-model="newZone.name"
            type="text"
            label="Name"
            placeholder="Your new zone name..."
          />

          <TInput
            class="col-span-2 sm:col-span-6 sm:col-start-4"
            v-model="newZone.price"
            type="number"
            placeholder="Hourly rate..."
          >
            <template #label>
              Price per hour <span class="text-gray-400">(100KB max)</span>
            </template>
            <template #left>
              <TInputLeftSide>ETH</TInputLeftSide>
            </template>
          </TInput>

          <div
            class="col-span-2 sm:col-span-6 sm:col-start-4"
          >
            <label
              for="cover-photo"
              class="block text-sm font-medium text-gray-700 mb-1"
            >
              Zone GeoJSON file <span class="text-gray-400">(100KB max)</span>
            </label>
            <div class="sm:col-span-2">
              <div
                @dragenter.prevent
                @dragover.prevent
                @drop.prevent="onDrop"
                class="flex justify-center rounded-md border-2 border-dashed border-gray-300 px-6 pt-5 pb-6"
              >
                <div
                  class="space-y-1 text-center"
                  v-if="file === null"
                >
                  <GlobeAltIcon
                    class="mx-auto h-12 w-12 text-gray-400"
                  />
                  <div class="flex text-sm text-gray-600">
                    <label
                      for="file-upload"
                      class="
                        relative
                        cursor-pointer
                        rounded-md
                        bg-white
                        font-medium
                        text-indigo-600
                        focus-within:outline-none
                        focus-within:ring-2
                        focus-within:ring-indigo-500
                        focus-within:ring-offset-2
                        hover:text-indigo-500
                      "
                    >
                      <span>Upload a file</span>
                      <input
                        @change="onFileChange"
                        id="file-upload"
                        accept=".geojson"
                        name="file-upload"
                        type="file"
                        class="sr-only"
                      />
                    </label>
                    <p class="pl-1">or drag and drop</p>
                  </div>
                  <p class="text-xs text-gray-500">
                    <strong>GEOJSON</strong> up to <strong>100KB</strong>
                  </p>
                </div>
                <div
                  v-else
                  class="
                    relative
                    w-32
                    space-y-1
                    text-center
                    border
                    border-solid
                    rounded-md
                    px-3
                    py-5
                    shadow-md
                  "
                >
                  <XMarkIcon
                    class="absolute h-5 w-5 text-gray-400 right-2 top-2 cursor-pointer"
                    @click="file = null"
                  />
                  <DocumentTextIcon
                    class="mx-auto h-12 w-12 text-gray-400"
                  />
                  <span
                    class="block text-sm truncate overflow-hidden"
                  >
                    {{ file?.name }}
                  </span>
                  <span
                    :class="`
                      block
                      text-xs
                      ${ Math.round(file?.size / 1024) > 100 ? 'text-red-600 animate-pulse' : '' }
                    `"
                  >
                    {{ Math.round(file?.size / 1024) }} KB
                  </span>
                </div>
              </div>
            </div>
          </div>
        </form>
      </div>
    </div>

    <div class="pt-5">
      <div class="flex justify-end">
        <TButton
          @click="this.emit('cancel')"
          type="button"
          color="white"
        >
          Cancel
        </TButton>
        <TButton
          class="ml-3"
          :disabled="disableSubmit"
          type="submit"
          color="green"
          @click="submit"
        >
          Create
        </TButton>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import TButton from '@/components/Controls/Button/TButton.vue';
import TInput from '@/components/Controls/Input/TInput.vue';
import { computed, reactive, ref, watch } from 'vue';
import type { GeoJSON } from 'geojson';
import TInputLeftSide from '@/components/Controls/Input/Partials/Sides/Left/TInputLeftSide.vue';
import { DocumentTextIcon, GlobeAltIcon, XMarkIcon } from '@heroicons/vue/24/outline';


const emit = defineEmits<{
  (e: 'cancel'): void;
}>();

const file = ref<File | null>(null);

const onFileChange = (e: Event) => {
  const target = e.target as HTMLInputElement;
  const files = target.files;

  if (files) {
    file.value = files[0];
  }
};

const newZone = reactive<{
  name: string;
  price: string;
  geo_json: GeoJSON | null;
}>({
  name: '',
  price: '',
  geo_json: null,
});

const disableSubmit = computed(() => {
  const floatPrice = parseFloat(newZone.price);

  return !(
    newZone.name.length > 3 &&
    floatPrice < 1 &&
    floatPrice > 0 &&
    newZone.geo_json !== null
  );
});

const reader = new FileReader();

reader.onload = (e) => {
  try {
    newZone.geo_json = JSON.parse(e.target?.result as string) as GeoJSON;
  } catch (e) {
    file.value = null;
    alert('Invalid file');
  }
};

watch(file, async (file) => {
  if (!file) {
    newZone.geo_json = null;

    return;
  }

  reader.readAsText(file);
});

function onDrop(e: DragEvent) {
  const files = e.dataTransfer?.files;

  if (files) {
    file.value = files[0];
  }
}

function submit() {
  console.log(newZone);
}
</script>
