<template>
  <HelperContainer :message="helpMessage" />
  <BaseContainer class="mb-5">
    <DAppViewSidebar />
  </BaseContainer>
  <BaseContainer class="mb-5">
    <Box additionalClass="col-span-3">
      <Map />
    </Box>
  </BaseContainer>
  <BaseContainer>
    <ParkingZoneInfoBox
      v-for="zone of zones"
      :zone="zone"
      @mouseenter="parkingZoneStore.setShowOnlyZoneId(zone.id)"
      @mouseleave="parkingZoneStore.setShowOnlyZoneId(null)"
      :hoverColor="true"
    />
  </BaseContainer>
</template>

<script setup lang="ts">
import Box from "@/components/Box/Box.vue";
import ParkingZoneInfoBox from "@/components/Box/Dedicated/ParkingZoneInfoBox.vue";
import BaseContainer from "@/components/Containers/BaseContainer.vue";
import HelperContainer from "@/components/Containers/HelperContainer.vue";
import Map from "@/components/Map/Map.vue";
import { useParkingZoneStore } from "@/stores/parking-zone";
import DAppViewSidebar from "@/views/App/DApp/DAppViewSidebar.vue";
import { storeToRefs } from "pinia";

const parkingZoneStore = useParkingZoneStore();
const { zones } = storeToRefs(parkingZoneStore);

const helpMessage = `
<ul>
  <li>At the bottom is a list of the existing Zones with the hourly rate.</li><br>
  <li>The car on the map can be moved which is displayed in the location box respectively.</li><br>
  <li>The page automatically checks whether the vehicle is in the zone and shows that with the price per hour.</li><br>
  <li>The start time of the ticket is prefilled and auto-updating, the duration can be safely chosen in the select box, and after filling in the plate number ticket can be securely bought.</li><br>
</ul>
`;
</script>
