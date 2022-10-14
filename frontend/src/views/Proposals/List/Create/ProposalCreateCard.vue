<template>
  <div class="space-y-8">
    <div>
      <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
        <div class="sm:col-span-6">
          <h3 class="text-lg font-medium leading-6 text-gray-900">Proposal type</h3>
          <p class="mt-1 text-sm text-gray-500">
            First of all choose a proposal type:<br>
            <strong>Change zone price</strong> when you want to change price per hour in some zone.<br>
            <strong>Create new zone</strong> when you want to create new parking zone.<br>
          </p>
        </div>

        <div class="sm:col-span-4">
          <label for="proposal-type" class="block text-sm font-medium text-gray-700">Type</label>
          <div class="mt-1 flex rounded-md shadow-sm max-w-lg sm:max-w-xs">
            <select
              v-model="formType"
              id="proposal-type"
              name="proposal-type"
              autocomplete="proposal-type-name"
              class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
            >
              <option value="">-</option>
              <option value="zone_price">Change zone price</option>
              <option value="new_zone">Create new zone (todo)</option>
            </select>
          </div>
        </div>

        <form class="sm:col-span-6 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
          <ProposalChangeZonePriceForm
            v-if="formType === 'zone_price'"
          />
          <ProposalAddNewZoneForm
            v-if="formType === 'new_zone'"
          />
        </form>
      </div>
    </div>

    <div class="pt-5">
      <div class="flex justify-end">
        <button
          @click="this.emit('cancel')"
          type="button"
          class="rounded-md border border-gray-300 bg-white py-2 px-4 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
        >
          Cancel
        </button>
        <button
          :disabled="costOfProposal <= 0"
          type="submit"
          class="`
            ml-3
            inline-flex
            justify-center
            rounded-md
            border
            border-transparent
            bg-indigo-600
            py-2
            px-4
            text-sm
            font-medium
            text-white
            shadow-sm
            hover:bg-indigo-700
            disabled:opacity-50
            disabled:cursor-not-allowed
          `"
        >
          Create {{ costOfProposal ? `for ${ gwei2eth(costOfProposal.toString()) } ETH` : '' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import ProposalChangeZonePriceForm from '@/views/Proposals/List/Create/Forms/ProposalChangeZonePriceForm.vue';
import { ref } from 'vue';
import ProposalAddNewZoneForm from '@/views/Proposals/List/Create/Forms/ProposalAddNewZoneForm.vue';
import { useProposalStore } from '@/stores/proposal';
import { storeToRefs } from 'pinia';
import { gwei2eth } from '@/helpers/helpers';

const emit = defineEmits<{
  (e: 'cancel'): void;
}>();

const formType = ref('');

const proposalStore = useProposalStore();
const {
  costOfProposal,
} = storeToRefs(proposalStore);
</script>
