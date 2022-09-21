import { defineStore } from 'pinia';
import type { InspectGetZoneReport, InspectGetZonesReport } from '@/interfaces/inspect-api';
import type { GeoJSON } from 'geojson';

export const useParkingZoneStore = defineStore('parking-zone', {
  state: () => ({
    zones: [] as InspectGetZonesReport,
    selectedZoneId: null as number|null,
  }),
  getters: {
    selectedZone: (state) => {
      const zone = state.zones.find((zone: InspectGetZoneReport) => {
        return zone.id === state.selectedZoneId;
      });

      return zone ? zone : null;
    },
  },
  actions: {
    clearZones() {
      this.zones = [];
    },
    addZone(zone: InspectGetZoneReport) {
      zone.geo_json = JSON.parse(zone.geo_json.toString()) as GeoJSON;

      this.zones.push(zone);
    },
    setSelectedZoneId(zoneId: number) {
      this.selectedZoneId = zoneId;
    },
  },
});
