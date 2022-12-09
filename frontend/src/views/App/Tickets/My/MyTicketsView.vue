<template>
  <BaseContainer>
    <Box additionalClass="col-span-3 border border-indigo-600 mb-5">
      <div class="
          flex
          justify-center
        ">
        <p
          v-if="waitingForNewTicket"
          class="text-indigo-900 text-center animate-pulse"
        >
          Some ticket is already on the way, please wait...
        </p>
        <p
          v-else
          class="text-indigo-900 text-center"
        >
          If you do not see your just-purchased ticket, wait a moment and then hit
          <TButton
            @click="parkingTicketStore.fetchTickets(true)"
            color="green"
          >
            reload tickets
          </TButton>
        </p>
      </div>
    </Box>
  </BaseContainer>
  <BaseContainer>
    <ParkingTicketBox
      v-for="ticket of tickets"
      :ticket="ticket"
      :zone="getZone(ticket.zone_id)"
    />
  </BaseContainer>
</template>

<script setup lang="ts">
import BaseContainer from '@/components/Containers/BaseContainer.vue';
import Box from '@/components/Box/Box.vue';
import { useParkingTicketStore } from '@/stores/parking-ticket';
import { storeToRefs } from 'pinia';
import { useParkingZoneStore } from '@/stores/parking-zone';
import { onMounted } from 'vue';
import ParkingTicketBox from '@/components/ParkingTicket/ParkingTicketBox.vue';
import TButton from '@/components/Controls/Button/TButton.vue';

const parkingTicketStore = useParkingTicketStore();
const parkingZoneStore = useParkingZoneStore();

const {
  waitingForNewTicket,
  tickets,
} = storeToRefs(parkingTicketStore);

onMounted(() => {
  parkingZoneStore.fetchZones();
  parkingTicketStore.fetchTickets();
});

const getZone = parkingZoneStore.getZone;
</script>
