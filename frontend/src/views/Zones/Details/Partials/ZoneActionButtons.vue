<template>
  <TButton
    class="px-3"
    color="yellow"
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
    @click="deleteZone"
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
import { useWalletStore } from '@/stores/wallet';
import { useParkingZoneStore } from '@/stores/parking-zone';
import router from '@/router';

const props = defineProps<{
  zone: ParkingZone;
}>();

const walletStore = useWalletStore();
const parkingZoneStore = useParkingZoneStore();

async function deleteZone() {
  if (await parkingZoneStore.deleteZone(props.zone.id)) {
    return router.push({
      name: 'dapp.zones.my',
    });
  }
}


</script>
