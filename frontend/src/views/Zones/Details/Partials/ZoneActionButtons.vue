<template>
  <ConfirmDialog
    ref="confirmDialogRef"
  />
  <TButton
    :disabled="parseInt(zone.balance) <= 0"
    class="px-3"
    color="yellow"
    @click="showWithdrawFundsDialog()"
  >
    <BanknotesIcon
      class="h-4 w-4 text-white"
    />
    <span class="pl-1">
      Withdraw funds
    </span>
  </TButton>
  <TButton
    class="px-3"
    color="red"
    @click="showDeleteDialog()"
  >
    <TrashIcon
      class="h-4 w-4 text-white"
    />
    <span class="pl-1">
      Delete zone
    </span>
  </TButton>
</template>

<script setup lang="ts">
import TButton from '@/components/Controls/Button/TButton.vue';
import { BanknotesIcon } from '@heroicons/vue/20/solid';
import { TrashIcon } from '@heroicons/vue/24/outline';
import type { ParkingZone } from '@/interfaces/parking-zone';
import { useParkingZoneStore } from '@/stores/parking-zone';
import ConfirmDialog from '@/components/Dialogs/ConfirmDialog.vue';
import { ref } from 'vue';
import router from '@/router';
import { gwei2eth } from '@/helpers/helpers';

const props = defineProps<{
  zone: ParkingZone;
}>();

const parkingZoneStore = useParkingZoneStore();

const confirmDialogRef = ref<InstanceType<typeof ConfirmDialog> | null>(null);

async function showDeleteDialog() {
  confirmDialogRef.value?.open({
    confirmed: () => {
      deleteZone();
    },
    canceled: () => {
      // ...
    },
  }, {
    title: 'Delete zone',
    message: 'Are you sure you want to delete this zone?',
    confirmButtonText: 'Yes, delete',
  });
}

async function showWithdrawFundsDialog() {
  confirmDialogRef.value?.open({
    confirmed: () => {
      console.log('confirmed 2');
    },
    canceled: () => {
      console.log('canceled 2');
    },
  },{
    title: 'Withdraw funds',
    color: 'yellow',
    message: 'Are you sure you want to withdraw funds from this zone?',
    confirmButtonText: `Yes, withdraw (${ gwei2eth(props.zone.balance) } ETH)`,
  });
}

async function deleteZone() {
  await parkingZoneStore.deleteZone(props.zone.id);

  return router.push({
    name: 'dapp.zones.my',
  });
}


</script>
