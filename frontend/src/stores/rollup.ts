import { defineStore } from 'pinia';

export const useRollupStore = defineStore('rollup', {
  state: () => ({
    rollupErrors: [] as string[],
  }),
  getters: {
    // ...
  },
  actions: {
    addError(error: string) {
      if (error.length < 1) return;

      this.rollupErrors.push(error);
    },
  },
});
