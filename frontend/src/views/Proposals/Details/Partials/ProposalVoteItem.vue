<template>
  <div
    :class="`
      ${ props.vote.type === 'up' ? 'bg-green-600' : 'bg-red-600' }
      flex-shrink-0
      flex
      items-center
      justify-center
      w-16
      text-white
      text-sm
      font-medium
      rounded-l-md
    `"
  >
    <ArrowUpIcon
      v-if="props.vote.type === 'up'"
      class="w-8 h-8"
    />
    <ArrowDownIcon
      v-else
      class="w-8 h-8"
    />
  </div>
  <div
    class="flex flex-1 items-center justify-between truncate rounded-r-md border-t border-r border-b border-gray-200 bg-white"
  >
    <div class="flex-1 truncate px-4 py-2 text-sm">
      <p class="font-medium text-gray-900 hover:text-gray-600 font-mono">{{ props.vote.wallet_address }}</p>
      <p class="text-gray-500">
        Voted {{ DateTime.fromISO(props.vote.created_at).toLocaleString(DateTime.DATETIME_MED_WITH_SECONDS) }}
      </p>
    </div>
  </div>

</template>

<script setup lang="ts">
import { ArrowDownIcon, ArrowUpIcon } from '@heroicons/vue/20/solid'
import type { ProposalVote } from '@/interfaces/proposal';
import { DateTime } from 'luxon';

const props = defineProps<{
  vote: ProposalVote;
}>();
</script>
