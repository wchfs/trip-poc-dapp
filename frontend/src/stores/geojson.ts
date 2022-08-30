import { defineStore } from 'pinia';
import type { GeoJSON } from 'geojson';
import axios from 'axios';

export const useGeoJsonStore = defineStore('geojson', {
    state: () => ({
        loading: true as boolean,
        map: null as GeoJSON|null,
    }),
    getters: {
        // ...
    },
    actions: {
        async setup() {
            const response = await axios.get('/geojson/map.geojson');

            this.map = response.data as GeoJSON;
            this.loading = false;
        },
    },
});
