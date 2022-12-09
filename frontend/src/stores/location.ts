import { defineStore } from 'pinia';

export const useLocationStore = defineStore('location', {
  state: () => ({
    loading: true as boolean,
    error: false as boolean,
    coords: null as GeolocationCoordinates | null,
    markerPosition: {
      lat: 50.0531980546981,
      lng: 19.937084978737403,
    } as {
      lat: number,
      lng: number,
    } | null,
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
    markerPositionWithSpaceSeparator: (state): string => {
      if (state.markerPosition === null) {
        return '';
      }

      return [
        state.markerPosition.lat.toFixed(6),
        state.markerPosition.lng.toFixed(6)
      ].join(' ');
    },
  },
  actions: {
    clear(error: boolean = false) {
      this.loading = false;
      this.error = error;
      this.coords = null;
    },
    setup(): boolean {
      if (!navigator.geolocation) {
        this.clear(true);

        return false;
      }

      if (this.coords !== null && this.error === false && this.loading === false) {
        return true;
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
      );

      return true;
    },
    setMarkerPosition(lat: number, lng: number) {
      this.markerPosition = { lat, lng };
    },
  },
});
