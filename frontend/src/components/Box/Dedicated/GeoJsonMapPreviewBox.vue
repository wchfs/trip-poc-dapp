<template>
  <Box
    v-if="!unwrapContainer"
    :additionalClass="props.additionalClass"
  >
    <div
      class="h-full"
      ref="mapContainer"
    />
  </Box>
  <div
    v-else
    class="h-full"
    ref="mapContainer"
  />
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import "leaflet/dist/leaflet.css";
import type { Map } from 'leaflet';
import L from 'leaflet';
import type { GeoJSON } from 'geojson';
import Box from '@/components/Box/Box.vue';

const props = withDefaults(defineProps<{
  geoJsonString: string | object | GeoJSON;
  additionalClass?: string;
  unwrapContainer?: boolean;
}>(), {
  additionalClass: 'col-span-3',
  unwrapContainer: false,
});

const mapContainer = ref<HTMLDivElement | null>(null);

let map = null as Map | null;

onMounted(() => {
  if (mapContainer.value !== null) {
    map = setupMap(mapContainer.value);
  }

  if (map) {
    drawGeoJson(map, props.geoJsonString);
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

function drawGeoJson(map: Map, geoJson: string | object | GeoJSON) {
  map.eachLayer((layer) => {
    if (layer instanceof L.GeoJSON) {
      map.removeLayer(layer);
    }
  });

  if (typeof geoJson === 'string') {
    geoJson = JSON.parse(geoJson);
  }

  const layer = L.geoJSON(geoJson as GeoJSON);

  layer.addTo(map);

  map.fitBounds(layer.getBounds());
}
</script>
