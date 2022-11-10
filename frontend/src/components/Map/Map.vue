<template>
  <div
    style="height:60vh"
    ref="mapContainer"
  ></div>
</template>

<script setup lang="ts">
import { useLocationStore } from '@/stores/location';
import { storeToRefs } from 'pinia';
import { onMounted, ref, watch } from 'vue';
import "leaflet/dist/leaflet.css";
import { useParkingZoneStore } from '@/stores/parking-zone';
import type { Map } from 'leaflet';
import L from 'leaflet';
import type { GeoJSON } from 'geojson';
import type { ParkingZone } from '@/interfaces/parking-zone';

type MarkerPosition = {
  lat: number,
  lng: number,
};

const mapContainer = ref<HTMLDivElement | null>(null);
const markerPositionRef = ref<MarkerPosition | null>(null);
const parkingZoneStore = useParkingZoneStore();
const {
  zones,
} = storeToRefs(parkingZoneStore);
const locationStore = useLocationStore();
const {
  coords,
} = storeToRefs(locationStore);

let map = null as Map | null;

onMounted(() => {
  try {
    parkingZoneStore.fetchZones();
  } catch (e) {
    console.log(e); // TODO handle it
  }

  if (mapContainer.value !== null) {
    map = setupMap(mapContainer.value);
  }

  if (map && zones.value.length > 0) {
    drawZones(map, zones.value);
  }

  if (map && locationStore.setup() && coords.value) {
    addAndWatchMarker(map, {
      lat: coords.value.latitude,
      lng: coords.value.longitude,
    });
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

function addAndWatchMarker(map: Map, markerPosition: MarkerPosition) {
  map.eachLayer((layer) => {
    if (layer instanceof L.Marker) {
      map.removeLayer(layer);
    }
  });

  markerPositionRef.value = markerPosition;

  const marker = L.marker([markerPosition.lat, markerPosition.lng], {
    draggable: true,
  }).addTo(map);

  map.setView([markerPosition.lat, markerPosition.lng], 15);

  marker.on('dragend', (e) => {
    const {lat, lng} = e.target.getLatLng();
    markerPositionRef.value = {lat, lng};
  });
}

function drawZones(map: Map, zones: ParkingZone[]) {
  map.eachLayer((layer) => {
    if (layer instanceof L.GeoJSON) {
      map.removeLayer(layer);
    }
  });

  zones.forEach((zone) => {
    L.geoJSON(zone.geo_json as GeoJSON).addTo(map);
  });
}

watch(coords, (coords) => {
  if (coords === null || map === null || markerPositionRef.value !== null) {
    return;
  }

  addAndWatchMarker(map, {
    lat: coords.latitude,
    lng: coords.longitude,
  });
});

watch(() => parkingZoneStore.zones, (parkingZones) => {
  if (map) {
    drawZones(map, parkingZones);
  }
}, {
  deep: true,
});


watch(markerPositionRef, (position) => {
  if (position === null) {
    return;
  }

  locationStore.setMarkerPosition(position.lat, position.lng);
});

</script>
