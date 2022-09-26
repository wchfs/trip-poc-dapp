import { defineStore } from 'pinia';
import type { ParkingTicket } from '@/interfaces/parking-ticket';

export const useParkingTicketStore = defineStore('parking-ticket', {
  state: () => ({
    tickets: [] as ParkingTicket[],
  }),
  getters: {},
  actions: {},
});
