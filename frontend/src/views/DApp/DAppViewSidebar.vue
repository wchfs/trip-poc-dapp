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
    v-if="markerPositionWithSpaceSeparator !== '' && selectedZoneId !== null"
    topText="Price per hour"
    :featuredText="selectedZoneId === 0 ? `Outside of any parking zone` : `${ selectedZone?.price } ETH`"
    :bottomText="selectedZoneId === 0 ? `Move marker somewhere else to check again` : `For zone: ${ selectedZone?.name }`"
    icon="fa-solid fa-hand-holding-dollar"
    textColor="text-green-500"
    bgColor="bg-green-500"
  />
  <Box
    v-if="!!selectedZoneId"
  >
    <DAppViewSidebarBuyTicket/>
  </Box>
  <InfoBox
    v-for="zone of zones"
    additionalClass="col-span-1 hover:bg-gray-100 hover:cursor-pointer"
    :topText="`Hourly rate for ${ zone.name }`"
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
import { watch } from 'vue';
import type { Error } from '@/interfaces/rollup-api';
import Box from '@/components/Box/Box.vue';
import DAppViewSidebarBuyTicket from '@/views/DApp/DAppViewSidebarBuyTicket.vue';
import { RollupService } from '@/services/rollup-service';

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

watch(markerPositionWithSpaceSeparator, function (value) {
  if (!value) {
    return;
  }

  check();
});

function check() {
  parkingZoneStore.setSelectedZoneId(null)

  RollupService.inspect<number>({
    endpoint: "check_point_in_zones",
    payload: {
      Point: {
        latitude: markerPosition.value?.lat as number,
        longitude: markerPosition.value?.lng as number,
      },
    },
  }).then((reports) => {
    reports.forEach(report => {
      parkingZoneStore.setSelectedZoneId(report);
    });
  }).catch((error: Error) => {
    console.log(error); // TODO handle it
  });
}
</script>
