<template>
  <HelperContainer :message="helpMessage" />
  <BaseContainer>
    <Box additionalClass="col-span-3 border border-orange-500 mb-5">
      <div class="
          flex
          justify-center
        ">
        <p
          v-if="parkingZoneStore.waitingForNewZone"
          class="text-orange-700 text-center animate-pulse"
        >
          Your new parking zone is already on the way, please wait...
        </p>
        <p
          v-else
          class="text-indigo-900 text-center"
        >
          If you do not see your just-created parking zone, wait a moment and then hit
          <TButton
            @click="parkingZoneStore.fetchZones(true)"
            color="green"
          >
            reload zones
          </TButton>
        </p>
      </div>
    </Box>
  </BaseContainer>
  <BaseContainer>
    <Box additionalClass="col-span-3 p-0 mb-6">
      <div :class="`
        ${showForm ? 'border-b border-gray-200' : ''}
        bg-white
        px-4
        py-5
        sm:px-6
      `">
        <div class="-ml-4 -mt-4 flex flex-wrap items-center justify-between sm:flex-nowrap">
          <div class="ml-4 mt-4">
            <h3 class="text-lg font-medium leading-6 text-gray-900">Create new parking zone</h3>
            <p class="mt-1 text-sm text-gray-500">
              You can create your own parking zones
            </p>
          </div>
          <div class="ml-4 mt-4 flex-shrink-0">
            <TButton
              v-if="!showForm"
              type="button"
              color="indigo"
              @click="showForm = !showForm"
            >
              Create new zone
            </TButton>
          </div>
        </div>
      </div>
      <div
        v-if="showForm"
        class="py-6 px-6 md:py-10 md:px-10"
      >
        <ZoneCreateCard @cancel="showForm = false" />
      </div>
    </Box>
  </BaseContainer>
  <BaseContainer>
    <ParkingZoneInfoBox
      v-for="zone of parkingZoneStore.currentUserZones(walletStore.walletAddress)"
      :zone="zone"
      :showManageButtons="true"
      @buttonClick="showDetails(zone)"
      :showButton="true"
      showButtonMode="hover"
    />
  </BaseContainer>
</template>

<script setup lang="ts">
import BaseContainer from '@/components/Containers/BaseContainer.vue';
import Box from '@/components/Box/Box.vue';
import { ref } from 'vue';
import TButton from '@/components/Controls/Button/TButton.vue';
import ZoneCreateCard from '@/views/App/Zones/My/Create/ZoneCreateCard.vue';
import { useParkingZoneStore } from '@/stores/parking-zone';
import ParkingZoneInfoBox from '@/components/Box/Dedicated/ParkingZoneInfoBox.vue';
import { useWalletStore } from '@/stores/wallet';
import type { ParkingZone } from '@/interfaces/parking-zone';
import router from '@/router';
import HelperContainer from '@/components/Containers/HelperContainer.vue';

const showForm = ref(false);

const parkingZoneStore = useParkingZoneStore();
const walletStore = useWalletStore();

parkingZoneStore.fetchZones();

function showDetails(zone: ParkingZone) {
  return router.push({
    name: 'dapp.zones.my.details',
    params: {
      zoneId: zone.id,
    },
  });
}

const helpMessage = `Creating a new zone is only possible for entitled wallets.
The list is filtered by wallet by default.
The zone is based on the premade GeoJSON file.
Please note that the owner's wallet will get all of the rights to the zone (withdraw funds, delete the zone).`;
</script>
