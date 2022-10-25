import { defineStore } from 'pinia';
import type { Proposal } from '@/interfaces/proposal';
import { eth2gwei } from '@/helpers/helpers';
import { BigNumber } from 'ethers';

export const useProposalStore = defineStore('proposal', {
  state: () => ({
    proposals: [] as Proposal[],
    proposal: null as Proposal | null,
    costOfProposal: BigNumber.from(0),
  }),
  getters: {},
  actions: {
    calculateCostOfNewProposal(
      currentPrice: string,
      newPrice: BigNumber,
    ) {
      let currentPriceBigNumber: BigNumber;

      try {
        currentPriceBigNumber = BigNumber.from(currentPrice);
      } catch (error) {
        this.costOfProposal = BigNumber.from(0);

        return;
      }

      if (currentPriceBigNumber.lte(0) || newPrice.lte(0)) {
        this.costOfProposal = BigNumber.from(0);

        return;
      }

      this.costOfProposal = BigNumber
        .from(currentPrice)
        .sub(newPrice)
        .abs()
        .mul(1000);
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
