<template>
  <BaseContainer>
    <Box
      additionalClass="col-span-3 border border-indigo-600"
    >
      <div
        class="
          flex
          justify-center
        "
      >
        <p v-if="waitingForNewTicket" class="text-indigo-900 text-center">
          Some ticket is already on the way, please wait...
        </p>
        <p v-else class="text-indigo-900 text-center">
          If you do not see your just-purchased ticket, wait a moment and then hit
          <ElButton
            type="success"
          >
            reload tickets
          </ElButton>
        </p>
      </div>
    </Box>
  </BaseContainer>
  <BaseContainer>
    <InfoBox
      v-for="ticket of tickets"
      additionalClass="col-span-3"
      :topText="`For ${ ticket.license } in ${ getZone(ticket.zone_id)?.name }`"
      :featuredText="`Expires: ${ getExpireAtString(ticket) }`"
      :bottomText="`Start: ${ getStartAtString(ticket) } Duration: ${ ticket.duration } min Id: ${ ticket.id }`"
      icon="fa-solid fa-receipt"
      textColor="text-blue-500"
      bgColor="bg-blue-500"
    />
  </BaseContainer>
</template>

<script setup lang="ts">
import 'element-plus/es/components/alert/style/css';
import BaseContainer from '@/components/Containers/BaseContainer.vue';
import InfoBox from '@/components/Box/InfoBox.vue';
import Box from '@/components/Box/Box.vue';
import { DateTime } from 'luxon';
import { useParkingTicketStore } from '@/stores/parking-ticket';
import { storeToRefs } from 'pinia';
import { useParkingZoneStore } from '@/stores/parking-zone';
import { onMounted } from 'vue';
import type { ParkingTicket } from '@/interfaces/parking-ticket';

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

const getExpireAtString = (ticket: ParkingTicket): string => {
  return DateTime.fromISO(ticket.started_at).plus({
    minutes: ticket.duration,
  }).toFormat(`yyyy LLL dd 'at' HH:mm`);
}

const getStartAtString = (ticket: ParkingTicket): string => {
  return DateTime.fromISO(ticket.started_at).toFormat(`yyyy LLL dd HH:mm`);
}
</script>
