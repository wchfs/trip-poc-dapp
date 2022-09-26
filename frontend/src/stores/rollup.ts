import { defineStore } from 'pinia';
import 'element-plus/es/components/notification/style/css';

export interface RollupState {
  communicationHistory: RollupCommunicationHistoryItem[],
}

export interface RollupCommunicationHistoryItem {
  rawOutput?: string,
  rawInput: string,
}

export const useRollupStore = defineStore('rollup', {
  state: (): RollupState => {
    return {
      communicationHistory: [], // TODO
    }
  },
  getters: {
    // ...
  },
  actions: {
    // ...
  },
});
