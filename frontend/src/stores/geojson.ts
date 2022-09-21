import { defineStore } from 'pinia';
import type { GeoJSON } from 'geojson';

export const useGeoJsonStore = defineStore('geojson', {
  state: () => ({
    geoJsons: [] as GeoJSON[],
  }),
  getters: {
    // ...
  },
  actions: {
    async addGeoJson(geoJson: GeoJSON) {
      this.geoJsons.push(geoJson);
    },
  },
});
