<template>
  <BaseContainer
    class="mb-5 xl:mb-6"
  >
    <Box
      additionalClass="col-span-3 p-0"
    >
      <div class="flex flex-wrap px-4 py-5 sm:px-6 items-center justify-between sm:flex-nowrap">
        <div>
          <h3 class="text-lg font-medium leading-6 text-gray-900">#{{ props.id }} Proposal Details</h3>
          <p class="mt-1 max-w-2xl text-sm text-gray-500">All info about this proposal</p>
        </div>
        <div class="flex-shrink-0 flex gap-2" v-if="proposal?.status === 'pending'">
          <TButton
            class="px-3"
            color="green"
          >
            <ArrowUpIcon
              class="h-4 w-4 text-white"
            />
            <span class="px-1">
              Vote
            </span>
          </TButton>
          <TButton
            class="px-3"
            color="red"
          >
            <ArrowDownIcon
              class="h-4 w-4 text-white"
            />
            <span class="px-1">
              Vote
            </span>
          </TButton>
        </div>
      </div>
      <div
        v-if="proposal"
        class="border-t border-gray-200 px-4 py-5 sm:p-0"
      >
        <dl class="divide-y divide-gray-200">
          <div class="py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:py-5 sm:px-6">
            <dt class="text-sm font-medium text-gray-500">Status</dt>
            <dd class="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">
              <TBadge
                :animation="proposal.status === 'pending' ? 'pulse' : 'none'"
                :color="getBadgeColor(proposal)"
              >
                {{ proposal.status }}
              </TBadge>
            </dd>
          </div>

          <ProposalDetailRow
            label="Identifier"
          >
            {{ proposal.id }}
          </ProposalDetailRow>

          <ProposalDetailRow
            label="Vote cost"
            color="indigo"
          >
            0.001 ETH
          </ProposalDetailRow>

          <ProposalDetailRow
            label="Title"
          >
            {{ proposal.title }}
          </ProposalDetailRow>

          <ProposalDetailRow
            label="Type"
          >
            Change zone price
          </ProposalDetailRow>

          <ProposalDetailRow
            label="Change to"
          >
            0.002 ETH per hour
          </ProposalDetailRow>

          <ProposalDetailRow
            label="Description"
          >
            {{ proposal.description }}
          </ProposalDetailRow>

          <ProposalDetailRow
            label="Due date"
          >
            {{ DateTime.now().toLocaleString(DateTime.DATETIME_MED) }}
          </ProposalDetailRow>

          <ProposalDetailRow
            label="Votes needed to complete"
          >
            10
          </ProposalDetailRow>

          <ProposalDetailRow
            label="Total votes"
          >
            {{ proposal.votes_up + proposal.votes_down }}
          </ProposalDetailRow>

          <ProposalDetailRow
            label="Votes up"
            color="green"
          >
            {{ proposal.votes_up }}
          </ProposalDetailRow>

          <ProposalDetailRow
            label="Votes down"
            color="red"
          >
            {{ proposal.votes_down }}
          </ProposalDetailRow>
        </dl>
      </div>
    </Box>
  </BaseContainer>
  <BaseContainer
    class="mb-5 xl:mb-6"
  >
    <Box
      additionalClass="col-span-3"
    >
      <ul
        class="grid grid-cols-1 gap-5 sm:grid-cols-1 lg:grid-cols-2"
      >
        <li
          v-for="proposalVote of proposalVotes"
          class="col-span-1 flex rounded-md shadow"
        >
          <ProposalVoteItem
            :vote="proposalVote"
          />
        </li>
      </ul>
    </Box>
  </BaseContainer>
</template>

<script setup lang="ts">
import BaseContainer from '@/components/Containers/BaseContainer.vue';
import Box from '@/components/Box/Box.vue';
import { useProposalStore } from '@/stores/proposal';
import { onMounted } from 'vue';
import { storeToRefs } from 'pinia';
import { DateTime } from 'luxon';
import TButton from '@/components/Controls/Button/TButton.vue';
import ProposalDetailRow from '@/views/App/Proposals/Details/Partials/ProposalDetailRow.vue';
import type { Proposal, ProposalVote } from '@/interfaces/proposal';
import ProposalVoteItem from '@/views/App/Proposals/Details/Partials/ProposalVoteItem.vue';
import { ArrowUpIcon, ArrowDownIcon } from '@heroicons/vue/24/outline';

const props = defineProps<{
  id: string;
}>();

const routeId = Number(props.id);

if (isNaN(routeId)) {
  throw new Error('Invalid proposal id');
}

const proposalStore = useProposalStore();
const {
  proposal,
} = storeToRefs(proposalStore);

onMounted(() => {
  proposalStore.fetchProposalById(routeId);
});

function getBadgeColor(proposal: Proposal) {
  switch (proposal.status) {
    case 'approved':
      return 'green';
    case 'rejected':
      return 'red';
    case 'pending':
    default:
      return 'indigo';
  }
}

const proposalVotes: ProposalVote[] = [
  {
    wallet_address: '0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc',
    type: 'up',
    transaction_address: '0xc66458c9c3e9366757bbcec60f83c2c72d10395f7e8370000d62752eeb7cd3ff',
    created_at: '2022-10-20T07:41:26Z',
  },
  {
    wallet_address: '0xd46ba6d942050d489dbd938a2c909a5d5039oyms',
    type: 'down',
    transaction_address: '0x86be892dcd9c2a0b382d9416d3a8b85a6aae2016dc80a6ebf86d7499e2af548a',
    created_at: '2022-10-20T08:41:26Z',
  },
  {
    wallet_address: '0x9259434c865ad0af9df7c64987920ade1e7c56bd',
    type: 'up',
    transaction_address: '0xc66458c9c3e9366757bbcec60f83c2c72d10396f7e8370200d62752eeb7cd544',
    created_at: '2022-10-20T08:42:26Z',
  },
  {
    wallet_address: '0x828ad70d0dafe4f4e4fa4c18b960d2368b14495a',
    type: 'down',
    transaction_address: '0x86be892dcd9c2a0b382d9416d3a8b85a6aee04fd1c80a6ebf87325ee2af51445',
    created_at: '2022-10-20T08:50:26Z',
  },
  {
    wallet_address: '0xff4d855f2500a961525492f0aae5f5d7ffa3bf38',
    type: 'up',
    transaction_address: '0xc66458c9c3e9366757bbcec60f83c2c72d10395f7e8370000d62752eeb7cd3ff',
    created_at: '2022-10-20T08:52:26Z',
  },
  {
    wallet_address: '0xebc12e6dcccf4422a81e80d5384546a94b6f1c6d',
    type: 'up',
    transaction_address: '0x86be892dcd9c2a0b382d9416d3a8b85a6aee04fd1c80a6ebf87325ee2af51445',
    created_at: '2022-10-20T10:42:26Z',
  },
  {
    wallet_address: '0xff4d855f2500a961525492f0aae5f5d7ffa3bf38',
    type: 'up',
    transaction_address: '0x86be892dcd9c2a0b382d9416d3a8b85a6aee04fd1c80a6ebf87325ee2af51445',
    created_at: '2022-10-20T10:46:26Z',
  },
  {
    wallet_address: '0xff4d855f2500a961525492f0aae5f5d7ffa3bf38',
    type: 'down',
    transaction_address: '0x86be892dcd9c2a0b382d9416d3a8b85a6aee04fd1c80a6ebf87325ee2af51445',
    created_at: '2022-10-20T14:26:26Z',
  },
  {
    wallet_address: '0x1f9840a85d5af5bf1d1762f925bdaddc4201f984',
    type: 'down',
    transaction_address: '0x86be892dcd9c2a0b382d9416d3a8b85a6aee04fd1c80a6ebf87325ee2af51445',
    created_at: '2022-10-20T15:26:26Z',
  },
];
</script>
