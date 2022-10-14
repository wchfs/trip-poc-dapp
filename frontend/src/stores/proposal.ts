import { defineStore } from 'pinia';
import type { Proposal } from '@/interfaces/proposal';
import { eth2gwei } from '@/helpers/helpers';

export const useProposalStore = defineStore('proposal', {
  state: () => ({
    proposals: [] as Proposal[],
    proposal: null as Proposal | null,
    costOfProposal: 0,
  }),
  getters: {},
  actions: {
    calculateCostOfNewProposal(
      oldPrice: string,
      newPrice: string,
    ) {
      console.log('oldPrice', oldPrice);
      console.log('newPrice', newPrice);
      //this.costOfProposal = (parseInt(oldPrice) - eth2gwei(newPrice.toString())) * 1000000000;
    },
    fetchProposalById(id: number) { // TODO: implement
      this.fetchProposals();

      const proposal = this.proposals.find((proposal) => proposal.id === id);

      this.proposal = proposal ? proposal : null;
    },
    fetchProposals(force: boolean = false) { // TODO: implement
      if (this.proposals.length > 0 && !force) {
        return;
      }

      this.proposals = [
        {
          id: 1,
          title: 'Change price for Zone #1',
          description: 'Set to 0.02 ETH per hour',
          status: 'pending',
          votes_up: 24,
          votes_down: 12,
          created_at: '2021-05-01T00:00:00.000Z',
        },
        {
          id: 2,
          title: 'Change price for Zone #2',
          description: 'Set to 0.03 ETH per hour',
          status: 'approved',
          votes_up: 24,
          votes_down: 12,
          created_at: '2021-05-01T00:00:00.000Z',
        },
        {
          id: 3,
          title: 'Change price for Zone #3',
          description: 'Set to 0.04 ETH per hour',
          status: 'rejected',
          votes_up: 24,
          votes_down: 12,
          created_at: '2021-05-01T00:00:00.000Z',
        },
        {
          id: 4,
          title: 'Change price for Zone #4',
          description: 'Set to 0.05 ETH per hour',
          status: 'pending',
          votes_up: 24,
          votes_down: 12,
          created_at: '2021-05-01T00:00:00.000Z',
        },
        {
          id: 5,
          title: 'Change price for Zone #5',
          description: 'Set to 0.06 ETH per hour',
          status: 'pending',
          votes_up: 24,
          votes_down: 12,
          created_at: '2021-05-01T00:00:00.000Z',
        },
      ];
    },
    addProposal(
      parkingZoneId: number,
      price: number,
      description: string,
    ) {

    },
  },
});
