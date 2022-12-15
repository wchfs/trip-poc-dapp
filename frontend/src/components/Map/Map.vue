<template>
  <div
    style="height:60vh"
    ref="mapContainer"
  ></div>
</template>

<script setup lang="ts">
import type { ParkingZone } from '@/interfaces/parking-zone';
import { MarkerPosition, useLocationStore } from '@/stores/location';
import { useParkingZoneStore } from '@/stores/parking-zone';
import type { GeoJSON } from 'geojson';
import type { Map } from 'leaflet';
import L from 'leaflet';
import "leaflet/dist/leaflet.css";
import { storeToRefs } from 'pinia';
import { computed, onMounted, ref, watch } from 'vue';

const mapContainer = ref<HTMLDivElement | null>(null);
const parkingZoneStore = useParkingZoneStore();
const {
  zones,
} = storeToRefs(parkingZoneStore);
const locationStore = useLocationStore();
const {
  coords,
} = storeToRefs(locationStore);

let map = null as Map | null;

const markerIcon = L.divIcon({
  html: `
      <svg class="w-9 h-9 drop-shadow" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" fill="currentColor">
        <path xmlns="http://www.w3.org/2000/svg" d="M441.5179,226.8645l-26.3-140.2688a17.11,17.11,0,0,0-16.8106-13.9438H113.5928A17.11,17.11,0,0,0,96.7822,86.5957l-26.3,140.2688a55.1171,55.1171,0,0,0-40.58,52.9783V355.75A17.0965,17.0965,0,0,0,47,372.8481H58.4019V389a50.3481,50.3481,0,0,0,100.6962,0V372.8481H352.9019V389a50.3481,50.3481,0,0,0,100.6962,0V372.8481H465A17.0965,17.0965,0,0,0,482.0981,355.75V279.8428A55.1171,55.1171,0,0,0,441.5179,226.8645ZM122.9988,327.25a28.5,28.5,0,1,1,28.5-28.5A28.4993,28.4993,0,0,1,122.9988,327.25Zm-17.301-102.5981,22.0893-117.8038H384.2129l22.0893,117.8038ZM388.9988,327.25a28.5,28.5,0,1,1,28.5-28.5A28.4993,28.4993,0,0,1,388.9988,327.25Z"/>
      </svg>
    `,
  className: 'border-0 bg-transparent text-red-700',
  iconAnchor: [12, 24],
});

let marker: L.Marker<any> | null = null;

const markerPositionRef = computed({
  get() {
    if (locationStore.markerPosition === null) {
      return null;
    }

    return {
      lat: locationStore.markerPosition.lat,
      lng: locationStore.markerPosition.lng,
    };
  },
  set(newValue: MarkerPosition | null) {
    if (newValue === null) {
      locationStore.clearMarkerPosition();
    } else {
      locationStore.setMarkerPosition(newValue.lat, newValue.lng);
    }
  },
});

watch(markerPositionRef, (newValue) => {
  if (map && newValue) {
    if (marker) {
      marker.setLatLng([newValue.lat, newValue.lng]);
      map.setView([newValue.lat, newValue.lng], 15);
    } else {
      addAndWatchMarker(map, newValue);
    }
  }
});

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

  if (marker === null) {
    marker = L.marker([markerPosition.lat, markerPosition.lng], {
      draggable: true,
      icon: markerIcon,
    }).addTo(map);
  }

  map.setView([markerPosition.lat, markerPosition.lng], 15);

  marker.on('dragend', (e) => {
    const { lat, lng } = e.target.getLatLng();
    markerPositionRef.value = { lat, lng };
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
</script>
