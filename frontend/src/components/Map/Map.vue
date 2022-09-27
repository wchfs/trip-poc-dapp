<template>
  <LMap
    style="height:70vh"
    v-model:zoom="zoom"
    :center="center"
    :no-blocking-animations="true"
    :zoom-animation="true"
  >
    <LTileLayer
      url="https://{s}.basemaps.cartocdn.com/rastertiles/light_all/{z}/{x}/{y}.png"
      layer-type="base"
      name="OpenStreetMap"
    />
    <LMarker
      v-if="markerPosition"
      v-model:lat-lng="markerPosition"
      draggable
    />
    <LGeoJson
      v-for="zone of zones"
      :geojson="zone.geo_json"
      :visible="showOnlyZoneId === null || showOnlyZoneId === zone.id"
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
import LGeoJson from '@vue-leaflet/vue-leaflet/src/components/LGeoJson.vue';
import LTileLayer from '@vue-leaflet/vue-leaflet/src/components/LTileLayer.vue';
import type { Error } from '@/interfaces/rollup-api';
import { useParkingZoneStore } from '@/stores/parking-zone';
import { RollupService } from '@/services/rollup-service';
import type { ParkingZone } from '@/interfaces/parking-zone';

const zoom = ref(4);
const center = ref({
  lat: 50,
  lng: 19,
});

const markerPosition = ref<{
  lat: number,
  lng: number,
}|null>(null);

const parkingZoneStore = useParkingZoneStore();
const {
  zones,
  showOnlyZoneId,
} = storeToRefs(parkingZoneStore);
const locationStore = useLocationStore();
const {
  coords,
} = storeToRefs(locationStore);

watch(coords, (coords) => {
  if (coords === null) {
    return;
  }

  markerPosition.value = {
    lat: coords.latitude,
    lng: coords.longitude,
  };

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

watch(markerPosition, (a) => {
  if (a === null) {
    return;
  }

  locationStore.setMarkerPosition(a.lat, a.lng);
});

onMounted(() => {
  locationStore.setup();

  try {
    parkingZoneStore.fetchZones();
  } catch (e) {
    console.log(e); // TODO handle it
  }
});

</script>
