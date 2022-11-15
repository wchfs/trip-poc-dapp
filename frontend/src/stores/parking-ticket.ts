import { defineStore } from 'pinia';
import type { ParkingTicket } from '@/interfaces/parking-ticket';
import { RollupService } from '@/services/rollup-service';
import type { Error } from '@/interfaces/rollup-api';
import { useWalletStore } from '@/stores/wallet';

export type BuyTicketPayload = {
  license: string,
  owner_address: string,
  longitude: number,
  latitude: number,
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

      const walletAddress = useWalletStore().walletAddress;

      if (walletAddress === null) {
        return; // TODO throw error
      }

      this.clearTickets();

      RollupService.inspect<ParkingTicket[]>({
        endpoint: "get_tickets",
        payload: {
          Ticket: {
            Get: {
              owner_address: walletAddress,
            },
          },
        },
      }).then((result) => {
        result.forEach(tickets => {
          tickets.data.forEach(ticket => {
            this.addTicket(ticket);
          });
        });
      }).catch((error: Error) => {
        throw error;
      });
    },
    async buyTicket(payload: BuyTicketPayload, price: string) {
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
      this.tickets = this.tickets.sort((a: ParkingTicket, b: ParkingTicket) => {
        return a.id > b.id ? -1 : 1;
      });
    },
  },
});
