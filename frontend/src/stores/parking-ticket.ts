import { defineStore } from 'pinia';
import type { ParkingTicket } from '@/interfaces/parking-ticket';
import { RollupService } from '@/services/rollup-service';
import type { Error } from '@/interfaces/rollup-api';

export type BuyTicketPayload = {
  license: string,
  latitude: number,
  longitude: number,
  started_at: string,
  duration: number,
  zone_id: number,
};

export const useParkingTicketStore = defineStore('parking-ticket', {
  state: () => ({
    waitingForNewTicket: false as boolean,
    tickets: [] as ParkingTicket[],
  }),
  getters: {},
  actions: {
    clearTickets() {
      this.tickets = [];
    },
    fetchTickets(force: boolean = false) {
      if (!force && this.tickets.length > 1) {
        return;
      }

      this.clearTickets();

      RollupService.inspect<ParkingTicket[]>({
        endpoint: "my_tickets",
        payload: null,
      }).then((result) => {
        console.log(result);
        result.forEach(tickets => {
          tickets.forEach(ticket => {
            this.addTicket(ticket);
          });
        });
      }).catch((error: Error) => {
        throw error;
      });
    },
    async buyTicket(payload: BuyTicketPayload, price: number) {
      this.waitingForNewTicket = true;

      const transactionResponse = await RollupService.addInput<ParkingTicket>({
        endpoint: 'buy_ticket',
        payload: {
          Ticket: {
            Buy: payload
          }
        },
      }, price);

      transactionResponse.response.then((r: ParkingTicket) => {
        this.waitingForNewTicket = false;
        this.addTicket(r);
      });
    },
    addTicket(ticket: ParkingTicket) {
      this.tickets = this.tickets.filter((t) => t.id !== ticket.id);
      this.tickets.push(ticket);
    },
  },
});
