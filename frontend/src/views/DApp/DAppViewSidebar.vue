<template>
  <InfoBox
    topText="Marker localization"
    featuredText="Zone A"
    :bottomText="`Localization: ${ markerPositionWithSpaceSeparator }`"
    icon="fa-solid fa-location-dot"
    textColor="text-red-500"
    bgColor="bg-red-500"
  />
  <InfoBox
    topText="You should pay"
    featuredText="6 ETH"
    bottomText="For zone A"
    icon="fa-solid fa-hand-holding-dollar"
    textColor="text-green-500"
    bgColor="bg-green-500"
  />
  <InfoBox
    topText="Zone A"
    featuredText="6 ETH"
    bottomText="Identifier: 1"
    icon="fa-solid fa-square-parking"
    textColor="text-blue-500"
    bgColor="bg-blue-500"
  />
  <InfoBox
    topText="Zone B"
    featuredText="4 ETH"
    bottomText="Identifier: 2"
    icon="fa-solid fa-square-parking"
    textColor="text-blue-500"
    bgColor="bg-blue-500"
  />
  <InfoBox
    topText="Zone C"
    featuredText="2 ETH"
    bottomText="Identifier: 3"
    icon="fa-solid fa-square-parking"
    textColor="text-blue-500"
    bgColor="bg-blue-500"
  />
</template>

<script setup lang="ts">
import InfoBox from '@/components/Box/InfoBox.vue';
import { useLocationStore } from '@/stores/location';
import { storeToRefs } from 'pinia';

const locationStore = useLocationStore();
const {
  markerPositionWithSpaceSeparator,
} = storeToRefs(locationStore);
</script>
