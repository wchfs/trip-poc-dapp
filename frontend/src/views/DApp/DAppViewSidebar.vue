<template>
  <InfoBox
    topText="Marker localization"
    :featuredText="markerPositionWithSpaceSeparator ? markerPositionWithSpaceSeparator : `Loading...`"
    bottomText="Drag marker on map to change"
    icon="fa-solid fa-location-dot"
    textColor="text-red-500"
    bgColor="bg-red-500"
  />
  <InfoBox
    v-if="markerPositionWithSpaceSeparator !== ''"
    topText="You should pay"
    :featuredText="selectedZoneId === 0 ? `Outside of any parking zone` : `${ selectedZone?.price} ETH`"
    :bottomText="selectedZoneId === 0 ? `Move marker somewhere else to check again` : `For zone: ${ selectedZone?.name}`"
    icon="fa-solid fa-hand-holding-dollar"
    textColor="text-green-500"
    bgColor="bg-green-500"
    :showButton="checkPrice"
    buttonText="Get price"
    @buttonClick="check()"
  />
  <Box>
    <DAppViewSidebarBuyTicket
      :checkPrice="checkPrice"
    />
  </Box>
  <InfoBox
    v-for="zone of zones"
    :topText="zone.name"
    :featuredText="`${ zone.price.toString() } ETH`"
    :bottomText="`Identifier: ${ zone.id }`"
    icon="fa-solid fa-square-parking"
    textColor="text-blue-500"
    bgColor="bg-blue-500"
    @mouseenter="parkingZoneStore.setShowOnlyZoneId(zone.id)"
    @mouseleave="parkingZoneStore.setShowOnlyZoneId(null)"
  />
</template>

<script setup lang="ts">
import InfoBox from '@/components/Box/InfoBox.vue';
import { useLocationStore } from '@/stores/location';
import { storeToRefs } from 'pinia';
import { useParkingZoneStore } from '@/stores/parking-zone';
import { ref, watch } from 'vue';
import type { Error, InspectCheckPointInZonesReport } from '@/interfaces/inspect-api';
import { useRollupStore } from '@/stores/rollup';
import Box from '@/components/Box/Box.vue';
import DAppViewSidebarBuyTicket from '@/views/DApp/DAppViewSidebarBuyTicket.vue';

const locationStore = useLocationStore();
const {
  markerPositionWithSpaceSeparator,
  markerPosition,
} = storeToRefs(locationStore);

const parkingZoneStore = useParkingZoneStore();
const {
  zones,
  selectedZone,
  selectedZoneId,
} = storeToRefs(parkingZoneStore);

const rollupStore = useRollupStore();

const checkPrice = ref<boolean>(true);

watch(markerPositionWithSpaceSeparator, function (value) {
  checkPrice.value = true;
});

function check() {
  rollupStore.inspectState<InspectCheckPointInZonesReport>({
    endpoint: "check_point_in_zones",
    payload: [
      markerPosition.value?.lng,
      markerPosition.value?.lat,
    ].join(','),
  }).then((reports) => {
    reports.forEach(report => {
      parkingZoneStore.setSelectedZoneId(report);
      checkPrice.value = false;
    });
  }).catch((error: Error) => {
    console.log(error);
  });
}
</script>
