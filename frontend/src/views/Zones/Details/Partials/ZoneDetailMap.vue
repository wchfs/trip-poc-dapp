<template>
  <div
    style="height:60vh"
    ref="mapContainer"
  ></div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import type { ParkingZone } from '@/interfaces/parking-zone';
import "leaflet/dist/leaflet.css";
import type { Map } from 'leaflet';
import L from 'leaflet';
import type { GeoJSON } from 'geojson';

const props = defineProps<{
  zone: ParkingZone;
}>();

const mapContainer = ref<HTMLDivElement | null>(null);

let map = null as Map | null;

onMounted(() => {
  if (mapContainer.value !== null) {
    map = setupMap(mapContainer.value);
  }

  if (map) {
    drawZone(map, props.zone);
  }
});

function setupMap(mapContainer: HTMLDivElement) {
  const map = L.map(mapContainer);

  L.tileLayer('https://{s}.basemaps.cartocdn.com/rastertiles/light_all/{z}/{x}/{y}.png', {
    attribution: '©OpenStreetMap, ©CartoDB',
  }).addTo(map);

  map.setView([50, 19], 4);

  return map;
}

function drawZone(map: Map, zone: ParkingZone) {
  map.eachLayer((layer) => {
    if (layer instanceof L.GeoJSON) {
      map.removeLayer(layer);
    }
  });

  const layer = L.geoJSON(zone.geo_json as GeoJSON).addTo(map);

  map.fitBounds(layer.getBounds());
}
</script>
