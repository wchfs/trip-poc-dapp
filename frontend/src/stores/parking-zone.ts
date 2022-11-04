import { defineStore } from 'pinia';
import type { GeoJSON } from 'geojson';
import type { ParkingZone } from '@/interfaces/parking-zone';
import { RollupService } from '@/services/rollup-service';
import type { Error } from '@/interfaces/rollup-api';

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
    getZone: (state) => {
      return (zoneId: number): ParkingZone|null => {
        const zone = state.zones.find((zone: ParkingZone) => {
          return zone.id === zoneId;
        });

        return zone ? zone : null;
      };
    },
  },
  actions: {
    clearZones() {
      this.zones = [];
    },
    fetchZones(force: boolean = false) {
      if (!force && this.zones.length > 1) {
        return;
      }

      this.clearZones();

      RollupService.inspect<ParkingZone[]>({
        endpoint: "get_zones",
        payload: null,
      }).then((result) => {
        result.forEach(reports => {
          reports.forEach(zoneReport => {
            this.addZone(zoneReport);
          });
        });
      }).catch((error: Error) => {
        throw error;
      });
    },
    addZone(zone: ParkingZone) {
      zone.geo_json = JSON.parse(zone.geo_json.toString()) as GeoJSON;

      this.zones.push(zone);
    },
    createZone(
      name: string,
      price: number,
      geoJson: GeoJSON,
    ) {
      RollupService.addInput<ParkingZone>({
        endpoint: "seed_zone",
        payload: {
          Seed: {
            Zone: {
              name,
              price,
              geo_json: JSON.stringify(geoJson),
            },
          },
        },
      }).then((result) => {
        console.log(result);
        //this.addZone(result);
      }).catch((error: Error) => {
        throw error;
      });
    },
    setSelectedZoneId(zoneId: number|null) {
      this.selectedZoneId = zoneId;
    },
    setShowOnlyZoneId(zoneId: number|null) {
      this.showOnlyZoneId = zoneId;
    }
  },
});
