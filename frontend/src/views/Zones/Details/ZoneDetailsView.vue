<template>
  <div
    v-if="zone"
  >
    <BaseContainer
      class="mb-5 xl:mb-6"
    >
      <Box
        additionalClass="col-span-3 p-0"
      >
        <div class="flex flex-wrap px-4 py-5 sm:px-6 items-center justify-between sm:flex-nowrap">
          <div>
            <h3 class="text-lg font-medium leading-6 text-gray-900">#{{ zone.id }} Parking Zone Details</h3>
            <p class="mt-1 max-w-2xl text-sm text-gray-500">All info about your parking zone</p>
          </div>
          <div class="flex-shrink-0 flex gap-2">
            <ZoneActionButtons
              :zone="zone"
            />
          </div>
        </div>
        <div
          class="border-t border-gray-200 px-4 py-5 sm:p-0"
        >
          <dl class="divide-y divide-gray-200">

            <ZoneDetailRow
              label="ID"
            >
              {{ zone.id }}
            </ZoneDetailRow>

            <ZoneDetailRow
              label="Name"
            >
              {{ zone.name }}
            </ZoneDetailRow>

            <ZoneDetailRow
              label="Price"
            >
              {{ gwei2eth(zone.price) }} ETH / h
            </ZoneDetailRow>

            <ZoneDetailRow
              label="Balance"
              v-if="zone.balance"
            >
              {{ gwei2eth(zone.balance) }} ETH
            </ZoneDetailRow>
          </dl>
        </div>
      </Box>
    </BaseContainer>
    <BaseContainer
      class="mb-5 xl:mb-6"
    >
      <GeoJsonMapPreviewBox
        style="height:60vh"
        :geoJsonString="zone.geo_json"
      />
    </BaseContainer>
  </div>
</template>

<script setup lang="ts">
import BaseContainer from '@/components/Containers/BaseContainer.vue';
import Box from '@/components/Box/Box.vue';
import { ref } from 'vue';
import ZoneDetailRow from '@/views/Zones/Details/Partials/ZoneDetailRow.vue';
import { useParkingZoneStore } from '@/stores/parking-zone';
import type { ParkingZone } from '@/interfaces/parking-zone';
import { useWalletStore } from '@/stores/wallet';
import { gwei2eth } from '@/helpers/helpers';
import "leaflet/dist/leaflet.css";
import type { Map } from 'leaflet';
import ZoneActionButtons from '@/views/Zones/Details/Partials/ZoneActionButtons.vue';
import router from '@/router';
import GeoJsonMapPreviewBox from '@/components/Box/Dedicated/GeoJsonMapPreviewBox.vue';

const props = defineProps<{
  zoneId: string;
}>();

const walletStore = useWalletStore();
const parkingZoneStore = useParkingZoneStore();
const zone = ref<ParkingZone | null>(null);
let map = null as Map | null;

parkingZoneStore.fetchZones(true).then(() => {
  const filteredZone = parkingZoneStore.currentUserZones(walletStore.walletAddress).find(z => {
    return z.id === parseInt(props.zoneId);
  }) || null;

  if (filteredZone === null) {
    return router.push({
      name: 'dapp.zones.my',
    });
  }

  zone.value = filteredZone;
});
</script>
