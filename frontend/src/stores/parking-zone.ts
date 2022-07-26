import type { ParkingZone } from "@/interfaces/parking-zone";
import type { RollupResponseDecodedPayload } from "@/interfaces/rollup-api";
import { RollupService } from "@/services/rollup-service";
import type { GeoJSON } from "geojson";
import { defineStore } from "pinia";

export const useParkingZoneStore = defineStore("parking-zone", {
  state: () => ({
    zones: [] as ParkingZone[],
    selectedZoneId: null as number | null,
    showOnlyZoneId: null as number | null,
    waitingForNewZone: false as boolean,
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
            }).then((reports) => {
              reports.forEach((report) => {
                z.balance = report.data;
              });
            });

            return z;
          });
      };
    },
  },
  actions: {
    clearZones() {
      this.zones = [];
    },
    async fetchZones(force: boolean = false): Promise<boolean> {
      if (!force && this.zones.length > 1) {
        return Promise.resolve(true);
      }

      this.clearZones();

      let result = null as
        | RollupResponseDecodedPayload<ParkingZone[]>[]
        | null;

      try {
        result = await RollupService.inspect<ParkingZone[]>({
          endpoint: "get_zones",
          payload: {
            Zone: {
              Get: {
                zone_id: null,
                owner_address: null,
              },
            },
          },
        });
      } catch (e) {
        console.error(e);
      }

      if (result === null) {
        return Promise.reject(false);
      }

      result.forEach((reports) => {
        reports.data.forEach((zoneReport) => {
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
      zone_owner_address: string,
      price: string,
      geoJson: GeoJSON
    ): Promise<void> {
      this.waitingForNewZone = true;

      const transactionResponse = await RollupService.addInput<ParkingZone>({
        endpoint: "seed_zone",
        payload: {
          Seed: {
            Zone: {
              name,
              zone_owner_address,
              price,
              geo_json: JSON.stringify(geoJson),
            },
          },
        },
      });

      transactionResponse.response.then((zone) => {
        this.addZone(zone.data);
      }).catch((error) => {
        console.error(error);
      }).finally(() => {
        this.waitingForNewZone = false;
      });
    },
    async deleteZone(zoneId: number): Promise<void> {
      const transactionResponse = await RollupService.addInput<{
        errors: string[];
      }>({
        endpoint: "remove_zone",
        payload: {
          Remove: {
            id: zoneId,
          },
        },
      });

      transactionResponse.response.then(() => {
        // this.fetchZones(true);
      }).catch((error) => {
        console.error(error);
      }).finally(() => {
        this.fetchZones(true);
      });
    },
    setSelectedZoneId(zoneId: number | null) {
      this.selectedZoneId = zoneId;
    },
    setShowOnlyZoneId(zoneId: number | null) {
      this.showOnlyZoneId = zoneId;
    },
  },
});
