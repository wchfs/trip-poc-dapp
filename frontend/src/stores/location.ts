import { defineStore } from 'pinia';

export const useLocationStore = defineStore('location', {
    state: () => ({
        loading: true as boolean,
        error: false as boolean,
        coords: null as GeolocationCoordinates|null,
    }),
    getters: {
        coordsArray: (state) => {
            if (state.coords === null) {
                return null;
            }

            return [
                state.coords.latitude,
                state.coords.longitude,
            ];
        },
    },
    actions: {
        clear(error: boolean = false) {
            this.loading = false;
            this.error = error;
            this.coords = null;
        },
        setup() {
            if (! navigator.geolocation) {
                this.clear(true);

                return;
            }

            navigator.geolocation.getCurrentPosition(
                (position: GeolocationPosition) => {
                    this.coords = position.coords;
                    this.error = false;
                    this.loading = false;
                },
                () => {
                    this.clear(true);
                },
            )
        },
    },
});
