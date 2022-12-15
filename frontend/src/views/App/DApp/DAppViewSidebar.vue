<template>
  <InfoBox
    :featuredText="
      markerPositionWithSpaceSeparator ? markerPositionWithSpaceSeparator : `Loading...`
    "
    bottomText="Drag vehicle marker on map to change"
    :icon="MapPinIcon"
    textColor="text-red-500"
  >
    <template #topText>
      <div class="text-xs text-gray-600 tracking-wide">
        Your location <strong
          @click="takeToTestLocation()"
          class="hover:text-red-500 cursor-pointer"
        >(take me to test location)</strong>
      </div>
    </template>
  </InfoBox>
  <InfoBox
    v-if="markerPositionWithSpaceSeparator !== '' && selectedZoneId !== null"
    topText="Price per hour"
    :featuredText="
      selectedZoneId === 0
        ? `Outside of any parking zone`
        : `${gwei2eth(selectedZone?.price.toString())} ETH`
    "
    :bottomText="
      selectedZoneId === 0
        ? `Move marker somewhere else to check again`
        : `For zone: ${selectedZone?.name}`
    "
    :icon="BanknotesIcon"
    textColor="text-green-500"
  />
  <Box
    v-if="!!selectedZoneId"
    additionalClass="col-span-1 md:col-span-2 lg:col-span-1 overflow-visible z-50"
  >
    <DAppViewSidebarBuyTicket />
  </Box>
</template>

<script setup lang="ts">
import InfoBox from "@/components/Box/InfoBox.vue";
import { useLocationStore } from "@/stores/location";
import { storeToRefs } from "pinia";
import { useParkingZoneStore } from "@/stores/parking-zone";
import { watch } from "vue";
import type { Error } from "@/interfaces/rollup-api";
import Box from "@/components/Box/Box.vue";
import DAppViewSidebarBuyTicket from "@/views/App/DApp/DAppViewSidebarBuyTicket.vue";
import { RollupService } from "@/services/rollup-service";
import { gwei2eth } from "@/helpers/helpers";
import { MapPinIcon, BanknotesIcon } from "@heroicons/vue/24/outline";
import type { ParkingZone } from "@/interfaces/parking-zone";

const locationStore = useLocationStore();
const { markerPositionWithSpaceSeparator, markerPosition } = storeToRefs(locationStore);

const parkingZoneStore = useParkingZoneStore();
const { selectedZone, selectedZoneId } = storeToRefs(parkingZoneStore);

watch(markerPositionWithSpaceSeparator, function (value) {
  if (!value) {
    return;
  }

  check();
});

function check() {
  parkingZoneStore.setSelectedZoneId(null);

  RollupService.inspect<ParkingZone>({
    endpoint: "check_point_in_zones",
    payload: {
      Point: {
        latitude: markerPosition.value?.lat as number,
        longitude: markerPosition.value?.lng as number,
      },
    },
  })
    .then((reports) => {
      reports.forEach((report) => {
        parkingZoneStore.setSelectedZoneId(report.data?.id ?? 0);
      });
    })
    .catch((error: Error) => {
      console.log(error); // TODO handle it
    });
}

function takeToTestLocation() {
  locationStore.setMarkerPosition(
    50.0531980546981,
    19.937084978737403,
  );
}
</script>
