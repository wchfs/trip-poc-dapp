import { defineStore } from 'pinia';
import 'element-plus/es/components/notification/style/css';

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
