<template>
  <div class="sm:col-span-3">
    <label for="parking-zone" class="block text-sm font-medium text-gray-700">Select parking zone</label>
    <div class="mt-1 flex rounded-md shadow-sm">
      <select
        v-model="formData.parking_zone_id"
        id="parking-zone"
        name="parking-zone"
        autocomplete="parking-zone-name"
        class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
      >
        <option value="" selected>-</option>
        <option
          v-for="zone in zones"
          :value="zone.id"
        >
          {{ zone.name }} ({{ gwei2eth(zone.price.toString()) }} ETH/h)
        </option>
      </select>
    </div>
  </div>

  <template
    v-if="selectedZone !== null"
  >
    <div class="sm:col-span-3">
      <label for="price" class="block text-sm font-medium text-gray-700">Propose change</label>
      <div class="mt-1 flex rounded-md shadow-sm">
      <span
        class="inline-flex items-center rounded-l-md border border-r-0 border-gray-300 bg-gray-50 px-3 text-gray-500 sm:text-sm"
      >
        {{ gwei2eth(selectedZone.price.toString()) }} ETH to
      </span>
        <input
          v-model="formData.price"
          type="number"
          name="price"
          id="price"
          autocomplete="price"
          class="block w-full min-w-0 flex-1 rounded-none border-gray-300 focus:border-indigo-500 focus:ring-indigo-500"
        />
        <span
          class="inline-flex items-center rounded-r-md border border-l-0 border-gray-300 bg-gray-50 px-3 text-gray-500 sm:text-sm"
        >
        ETH
      </span>
      </div>
    </div>

    <div class="sm:col-span-6">
      <label for="description" class="block text-sm font-medium text-gray-700">
        Description
      </label>
      <div class="mt-1">
      <textarea
        v-model="formData.description"
        id="description"
        name="description"
        rows="3"
        class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
      />
      </div>
      <p class="mt-2 text-sm text-gray-500">
        Write a few sentences to encourage people to vote for your proposal.
      </p>
    </div>
  </template>
</template>

<script setup lang="ts">
import { useParkingZoneStore } from '@/stores/parking-zone';
import { storeToRefs } from 'pinia';
import type { ParkingZone } from '@/interfaces/parking-zone';
import { ref, watch } from 'vue';
import { eth2gwei, gwei2eth } from '@/helpers/helpers';
import { useProposalStore } from '@/stores/proposal';
import { BigNumber } from 'ethers';

const proposalStore = useProposalStore();

const parkingZoneStore = useParkingZoneStore();
parkingZoneStore.fetchZones();

const {
  zones,
} = storeToRefs(parkingZoneStore);

const formData = ref({
  parking_zone_id: '',
  price: '',
  description: '',
});

const selectedZone = ref<ParkingZone | null>(null);

watch(formData, (fd) => {
  setSelectedZone(fd.parking_zone_id);
  calculateCost(fd.price);
}, {
  deep: true,
});

function setSelectedZone(parking_zone_id: string) {
  if (parking_zone_id === '') {
    selectedZone.value = null;

    return;
  }

  const selectedZoneId = parseInt(parking_zone_id);

  if (selectedZone.value?.id === selectedZoneId) {
    return;
  }

  selectedZone.value = parkingZoneStore.getZone(selectedZoneId);
}

function calculateCost(newPrice: string) {
  if (newPrice === '' || parseInt(newPrice) < 0) {
    newPrice = '0';
  }

  const selectedZoneValue = selectedZone.value;
  let newPriceBigNumber = BigNumber.from(0);

  try {
    newPriceBigNumber = newPriceBigNumber.add(BigNumber.from(eth2gwei(newPrice.toString())));
  } catch (e) {
    proposalStore.calculateCostOfNewProposal('0', BigNumber.from(0));

    return;
  }

  if (selectedZoneValue === null || newPriceBigNumber.isZero()) {
    proposalStore.calculateCostOfNewProposal('0', BigNumber.from(0));

    return;
  }

  proposalStore.calculateCostOfNewProposal(
    selectedZoneValue.price.toString(),
    newPriceBigNumber,
  );
}
</script>
