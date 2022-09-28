<template>
  <BaseContainer
    class="mb-5"
  >
    <DAppViewSidebar/>
  </BaseContainer>
  <BaseContainer
    class="mb-5"
  >
    <Box additionalClass="col-span-3">
      <Map/>
    </Box>
  </BaseContainer>
  <BaseContainer>
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
  </BaseContainer>
</template>

<script setup lang="ts">
import BaseContainer from '@/components/Containers/BaseContainer.vue';
import InfoBox from '@/components/Box/InfoBox.vue';
import Box from '@/components/Box/Box.vue';
import Map from '@/components/Map/Map.vue';
import DAppViewSidebar from '@/views/DApp/DAppViewSidebar.vue';
import { useParkingZoneStore } from '@/stores/parking-zone';
import { storeToRefs } from 'pinia';

const parkingZoneStore = useParkingZoneStore();
const {
  zones,
} = storeToRefs(parkingZoneStore);
</script>
