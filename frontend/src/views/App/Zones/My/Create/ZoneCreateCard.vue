<template>
  <div class="space-y-8">
    <div>
      <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
        <form class="sm:col-span-6 grid grid-cols-2 gap-y-6 gap-x-4 sm:grid-cols-12">
          <TInput
            class="col-span-2 sm:col-span-6 sm:col-start-4"
            v-model="newZone.name"
            type="text"
            placeholder="Your new zone name..."
          >
            <template #label>
              Name <span class="text-gray-400">(from 3 to 100 characters)</span>
            </template>
          </TInput>

          <TInput
            class="col-span-2 sm:col-span-6 sm:col-start-4"
            v-model="newZone.owner_address"
            type="text"
            placeholder="Your wallet address..."
            error="You must be the owner of the zone."
          >
            <template #label> Zone owner wallet address </template>
          </TInput>

          <TInput
            class="col-span-2 sm:col-span-6 sm:col-start-4"
            v-model="newZone.price"
            type="number"
            placeholder="Hourly rate..."
          >
            <template #label>
              Price per hour <span class="text-gray-400">(0.0001 ETH to 1 ETH)</span>
            </template>
            <template #left>
              <TInputLeftSide>ETH</TInputLeftSide>
            </template>
          </TInput>

          <div class="col-span-2 sm:col-span-6 sm:col-start-4">
            <TInputDropZone class="mb-3" @fileChanged="fileChanged" />
            <GeoJsonMapPreviewBox
              v-if="newZone.geo_json"
              class="border border-gray-300 rounded-md shadow-sm"
              style="height: 20vh"
              :unwrapContainer="true"
              :geoJsonString="newZone.geo_json"
            />
          </div>
        </form>
      </div>
    </div>

    <div class="pt-5">
      <div class="flex justify-end">
        <TButton @click="$emit('cancel')" type="button" color="white">
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
import TButton from "@/components/Controls/Button/TButton.vue";
import TInput from "@/components/Controls/Input/TInput.vue";
import { computed, reactive, ref, watch } from "vue";
import type { GeoJSON } from "geojson";
import TInputLeftSide from "@/components/Controls/Input/Partials/Sides/Left/TInputLeftSide.vue";
import TInputDropZone from "@/components/Controls/Input/TInputDropZone.vue";
import { useParkingZoneStore } from "@/stores/parking-zone";
import { eth2gwei } from "@/helpers/helpers";
import GeoJsonMapPreviewBox from "@/components/Box/Dedicated/GeoJsonMapPreviewBox.vue";
import { useWalletStore } from "@/stores/wallet";

const parkingZoneStore = useParkingZoneStore();
const walletStore = useWalletStore();

const emit = defineEmits<{
  (e: "cancel"): void;
}>();

const file = ref<File | null>(null);

const reader = new FileReader();

reader.onload = (e) => {
  try {
    newZone.geo_json = JSON.parse(e.target?.result as string) as GeoJSON;
  } catch (e) {
    file.value = null;
    alert("Invalid file");
  }
};

const newZone = reactive<{
  name: string;
  owner_address: string;
  price: string;
  geo_json: GeoJSON | null;
}>({
  name: "",
  owner_address: walletStore.walletAddress ?? "",
  price: "",
  geo_json: null,
});

const disableSubmit = computed(() => {
  let bigNumberPrice = eth2gwei("0");

  try {
    bigNumberPrice = eth2gwei(newZone.price);
  } catch (e) {
    return true;
  }

  return !(
    newZone.name.length >= 3 &&
    newZone.name.length <= 100 &&
    bigNumberPrice.lte("1000000000") &&
    bigNumberPrice.gte("100000") &&
    newZone.geo_json !== null
  );
});

watch(file, async (file) => {
  if (!file) {
    newZone.geo_json = null;

    return;
  }

  reader.readAsText(file);
});

function fileChanged(e: File | null) {
  file.value = e;
}

function submit() {
  parkingZoneStore.createZone(
    newZone.name,
    newZone.owner_address,
    eth2gwei(newZone.price).toString(),
    newZone.geo_json as GeoJSON
  );

  emit("cancel");
}
</script>
