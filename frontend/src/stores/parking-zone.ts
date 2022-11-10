import { defineStore } from 'pinia';
import type { GeoJSON } from 'geojson';
import type { ParkingZone } from '@/interfaces/parking-zone';
import { RollupService } from '@/services/rollup-service';

export const useParkingZoneStore = defineStore('parking-zone', {
  state: () => ({
    zones: [] as ParkingZone[],
    selectedZoneId: null as number | null,
    showOnlyZoneId: null as number | null,
  }),
  getters: {
    selectedZone: (state) => {
      const zone = state.zones.find((zone: ParkingZone) => {
        return zone.id === state.selectedZoneId;
      });

      return zone ? zone : null;
    },
    getZone: (state) => {
      return (zoneId: number): ParkingZone | null => {
        const zone = state.zones.find((zone: ParkingZone) => {
          return zone.id === zoneId;
        });

        return zone ? zone : null;
      };
    },
    currentUserZones: (state) => {
      return function (currentWalletAddress: string | null): ParkingZone[] {
        if (!currentWalletAddress) {
          return [];
        }

        return state.zones
          .filter((z) => z.owner_address === currentWalletAddress)
          .map((z) => {
            RollupService.inspect<string>({
              endpoint: "get_app_balance",
              payload: {
                Balance: {
                  Get: {
                    zone_id: z.id,
                  },
                },
              },
            }).then(reports => {
              reports.forEach(report => {
                z.balance = report;
              });
            });

            return z;
          });
      }
    },
  },
  actions: {
    clearZones() {
      this.zones = [];
    },
    async fetchZones(
      force: boolean = false,
    ): Promise<boolean> {
      if (!force && this.zones.length > 1) {
        return Promise.resolve(true);
      }

      this.clearZones();

      let result = null;

      try {
        result = await RollupService.inspect<ParkingZone[]>({
          endpoint: "get_zones",
          payload: null,
        });
      } catch (e) {
        console.error(e);
      }

      if (result === null) {
        return Promise.reject(false);
      }

      result.forEach(reports => {
        reports.forEach(zoneReport => {
          this.addZone(zoneReport);
        });
      });

      return Promise.resolve(true);
    },
    addZone(zone: ParkingZone) {
      zone.geo_json = JSON.parse(zone.geo_json.toString()) as GeoJSON;

      this.zones.push(zone);
    },
    async createZone(
      name: string,
      price: string,
      geoJson: GeoJSON,
    ) {
      const result = await RollupService.addInput<ParkingZone>({
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
      });

      result.response.then((zone: ParkingZone) => {
        this.addZone(zone);
      });

      return result;
    },
    async deleteZone(zoneId: number): Promise<boolean> {
      const result = await RollupService.addInput<{
        errors: string[];
      }>({
        endpoint: "remove_zone",
        payload: {
          Remove: {
            id: zoneId,
          },
        },
      });

      this.fetchZones(true);

      console.log(result);

      return true;
    },
    setSelectedZoneId(zoneId: number | null) {
      this.selectedZoneId = zoneId;
    },
    setShowOnlyZoneId(zoneId: number | null) {
      this.showOnlyZoneId = zoneId;
    },
  },
});
