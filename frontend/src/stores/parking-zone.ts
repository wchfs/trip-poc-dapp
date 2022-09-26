import { defineStore } from 'pinia';
import type { GeoJSON } from 'geojson';
import type { ParkingZone } from '@/interfaces/parking-zone';

export const useParkingZoneStore = defineStore('parking-zone', {
  state: () => ({
    zones: [] as ParkingZone[],
    selectedZoneId: null as number|null,
    showOnlyZoneId: null as number|null,
  }),
  getters: {
    selectedZone: (state) => {
      const zone = state.zones.find((zone: ParkingZone) => {
        return zone.id === state.selectedZoneId;
      });

      return zone ? zone : null;
    },
  },
  actions: {
    clearZones() {
      this.zones = [];
    },
    addZone(zone: ParkingZone) {
      zone.geo_json = JSON.parse(zone.geo_json.toString()) as GeoJSON;

      this.zones.push(zone);
    },
    setSelectedZoneId(zoneId: number) {
      this.selectedZoneId = zoneId;
    },
    setShowOnlyZoneId(zoneId: number|null) {
      this.showOnlyZoneId = zoneId;
    }
  },
});
