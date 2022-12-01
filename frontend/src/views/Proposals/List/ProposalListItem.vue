<template>
  <Box additionalClass="p-0">
    <div class="flex w-full items-center justify-between space-x-6 p-6">
      <div class="flex-1 truncate">
        <div class="flex items-center space-x-3">
          <h3 class="truncate text-sm font-medium text-gray-900">
            {{ props.proposal.title }}
          </h3>
          <TBadge
            :animation="props.proposal.status === 'pending' ? 'pulse' : 'none'"
            :color="getBadgeColor(props.proposal)"
          >
            {{ props.proposal.status }}
          </TBadge>
        </div>
        <p class="mt-1 truncate text-sm text-gray-500">
          {{ props.proposal.description }}
        </p>
      </div>
    </div>
    <div>
      <div class="-mb-0 flex divide-x divide-gray-200">
        <div class="flex w-0 flex-1">
          <div
            class="relative inline-flex w-0 flex-1 items-center justify-center rounded-bl-lg border border-transparent py-4 text-sm font-medium text-gray-700"
          >
            <HashtagIcon class="w-4 w-4 text-gray-400" aria-hidden="true" />
            <span class="ml-3">{{ props.proposal.id }}</span>
          </div>
        </div>
        <div class="flex w-0 flex-1">
          <div
            class="relative inline-flex w-0 flex-1 items-center justify-center rounded-bl-lg border border-transparent py-4 text-sm font-medium text-gray-700"
          >
            <ArrowUpIcon class="h-5 w-5 text-green-600" aria-hidden="true" />
            <span class="ml-3">{{ props.proposal.votes_up }}</span>
          </div>
        </div>
        <div class="flex w-0 flex-1">
          <div
            class="relative inline-flex w-0 flex-1 items-center justify-center rounded-br-lg border border-transparent py-4 text-sm font-medium text-gray-700"
          >
            <ArrowDownIcon class="h-5 w-5 text-red-600" aria-hidden="true" />
            <span class="ml-3">{{ props.proposal.votes_down }}</span>
          </div>
        </div>
        <div class="flex w-0 flex-1">
          <router-link
            :to="{
              name: 'dapp.proposals.details',
              params: {
                id: props.proposal.id,
              },
            }"
            class="relative inline-flex w-0 flex-1 items-center justify-center rounded-br-lg border border-transparent py-4 text-sm font-medium text-gray-700"
          >
            <Bars3BottomLeftIcon class="h-5 w-5 text-gray-400" aria-hidden="true" />
            <span class="ml-3">Details</span>
          </router-link>
        </div>
      </div>
    </div>
  </Box>
</template>

<script setup lang="ts">
import Box from "@/components/Box/Box.vue";
import {
  ArrowDownIcon,
  ArrowUpIcon,
  Bars3BottomLeftIcon,
  HashtagIcon,
} from "@heroicons/vue/20/solid";
import type { Proposal } from "@/interfaces/proposal";
import TBadge from "@/components/Common/TBadge/TBadge.vue";

const props = defineProps<{
  proposal: Proposal;
}>();

function getBadgeColor(proposal: Proposal) {
  switch (proposal.status) {
    case "approved":
      return "green";
    case "rejected":
      return "red";
    case "pending":
    default:
      return "indigo";
  }
}
</script>
