<template>
  <Box
    additionalClass="p-0"
  >
    <div class="flex w-full items-center justify-between space-x-6 p-6">
      <div class="flex-1 truncate">
        <div class="flex items-center space-x-3">
          <h3 class="truncate text-sm font-medium text-gray-900">
            {{ props.proposal.title }}
          </h3>
          <span
            :class="`
                inline-block
                flex-shrink-0
                rounded-full
                px-2
                py-0.5
                text-xs
                font-medium
                ${ props.proposal.status === 'approved' ? 'bg-green-100 text-green-800' : '' }
                ${ props.proposal.status === 'pending' ? 'bg-indigo-100 text-indigo-800 animate-pulse' : '' }
                ${ props.proposal.status === 'rejected' ? 'bg-red-100 text-red-800' : '' }
              `"
          >{{ props.proposal.status }}</span>
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
            class="relative inline-flex w-0 flex-1 items-center justify-center rounded-bl-lg border border-transparent py-4 text-sm font-medium text-gray-700">
            <HashtagIcon
              class="h-5 w-5 text-gray-400"
              aria-hidden="true"
            />
            <span class="ml-3">{{ props.proposal.id }}</span>
          </div>
        </div>
        <div class="flex w-0 flex-1">
          <div
            class="relative inline-flex w-0 flex-1 items-center justify-center rounded-bl-lg border border-transparent py-4 text-sm font-medium text-gray-700">
            <ArrowUpIcon
              class="h-5 w-5 text-gray-400"
              aria-hidden="true"
            />
            <span class="ml-3">{{ props.proposal.votes_up }}</span>
          </div>
        </div>
        <div class="flex w-0 flex-1">
          <div
            class="relative inline-flex w-0 flex-1 items-center justify-center rounded-br-lg border border-transparent py-4 text-sm font-medium text-gray-700">
            <ArrowDownIcon
              class="h-5 w-5 text-gray-400"
              aria-hidden="true"
            />
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
            <Bars3BottomLeftIcon
              class="h-5 w-5 text-gray-400"
              aria-hidden="true"
            />
            <span class="ml-3">Details</span>
          </router-link>
        </div>
      </div>
    </div>
  </Box>
</template>

<script setup lang="ts">
import Box from '@/components/Box/Box.vue';
import { ArrowDownIcon, ArrowUpIcon, Bars3BottomLeftIcon, HashtagIcon } from '@heroicons/vue/20/solid'
import type { Proposal } from '@/interfaces/proposal';

const props = defineProps<{
  proposal: Proposal;
}>();

</script>
