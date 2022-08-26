<template>
  <LMap
    style="height:70vh"
    v-model:zoom="zoom"
    :center="center"
    :no-blocking-animations="true"
  >
    <LTileLayer
      url="https://{s}.basemaps.cartocdn.com/rastertiles/light_all/{z}/{x}/{y}.png"
      layer-type="base"
      name="OpenStreetMap"
    />
    <LMarker
      v-if="coords"
      :lat-lng="coordsArray"
    />
  </LMap>
</template>

<script setup lang="ts">
import { useLocationStore } from '@/stores/location';
import { storeToRefs } from 'pinia';
import { onMounted, ref, watch } from 'vue';
import "leaflet/dist/leaflet.css";
import LMap from '@vue-leaflet/vue-leaflet/src/components/LMap.vue';
import LMarker from '@vue-leaflet/vue-leaflet/src/components/LMarker.vue';
import LTileLayer from '@vue-leaflet/vue-leaflet/src/components/LTileLayer.vue';

const zoom = ref(4);
const center = ref({
  lat: 50,
  lng: 19,
});

const locationStore = useLocationStore();
const {
  coords,
  coordsArray,
} = storeToRefs(locationStore);

watch(coords, (coords) => {
  if (coords === null) {
    return;
  }

  zoom.value = 14;

  setTimeout(() => {
    /**
     * We need timeout here because leaflet for vue3 has issue with zoom and center in the same time
     */
    center.value = {
      lat: coords.latitude,
      lng: coords.longitude,
    };
  }, 100);
});

onMounted(() => {
  locationStore.setup();
});

</script>
