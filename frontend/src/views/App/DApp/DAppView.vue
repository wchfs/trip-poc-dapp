<template>
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
import BaseContainer from '@/components/Containers/BaseContainer.vue';
import Box from '@/components/Box/Box.vue';
import Map from '@/components/Map/Map.vue';
import DAppViewSidebar from '@/views/App/DApp/DAppViewSidebar.vue';
import { useParkingZoneStore } from '@/stores/parking-zone';
import { storeToRefs } from 'pinia';
import ParkingZoneInfoBox from '@/components/Box/Dedicated/ParkingZoneInfoBox.vue';

const parkingZoneStore = useParkingZoneStore();
const {
  zones,
} = storeToRefs(parkingZoneStore);
</script>
