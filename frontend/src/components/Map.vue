<template>
  <div v-if="locationStore.loading">
    Getting your location...
  </div>
  <div v-else>
    <div v-if="geoJsonStore.loading">
      Getting map data...
    </div>
    <div v-else>
      <l-map style="height:70vh" ref="map" v-model:zoom="zoom" :center="locationStore.coordsArray">
        <l-tile-layer
            url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
            layer-type="base"
            name="OpenStreetMap"
        ></l-tile-layer>
        <l-marker :lat-lng="locationStore.coordsArray"></l-marker>
        <l-geo-json :geojson="geoJsonStore.map"></l-geo-json>
      </l-map>
    </div>
  </div>
</template>

<script setup lang="ts">

import { useLocationStore } from '@/stores/location';
import { useGeoJsonStore } from '@/stores/geojson';
import 'leaflet/dist/leaflet.css';
import { LMap, LGeoJson, LTileLayer, LMarker } from '@vue-leaflet/vue-leaflet';

const zoom = 14;

const locationStore = useLocationStore();
const geoJsonStore = useGeoJsonStore();

locationStore.setup();
geoJsonStore.setup();

</script>
