<template>
  <InfoBox
    additionalClass="col-span-3"
    :topText="`#${props.ticket.id} For ${props.ticket.license} in ${props.zone?.name}`"
    :featuredText="`Expires: ${getExpireAtString(props.ticket)}`"
    :bottomText="`Starts ${getStartAtString(
      props.ticket
    )} and is valid for ${getDurationString(props.ticket.duration)}`"
    :icon="TicketIcon"
    textColor="text-blue-500"
  />
</template>

<script setup lang="ts">
import InfoBox from "@/components/Box/InfoBox.vue";
import type { ParkingTicket } from "@/interfaces/parking-ticket";
import { DateTime } from "luxon";
import type { ParkingZone } from "@/interfaces/parking-zone";
import { TicketIcon } from "@heroicons/vue/24/outline";

const props = defineProps<{
  ticket: ParkingTicket;
  zone?: ParkingZone;
}>();

const getExpireAtString = (ticket: ParkingTicket): string => {
  return DateTime.fromISO(ticket.started_at)
    .plus({
      minutes: ticket.duration,
    })
    .toFormat(`yyyy LLL dd 'at' HH:mm`);
};

const getStartAtString = (ticket: ParkingTicket): string => {
  return DateTime.fromISO(ticket.started_at).toFormat(`yyyy LLL dd 'at' HH:mm`);
};

function getDurationString(duration: number): string {
  const hoursPart = Math.floor(duration / 60);
  const minutesPart = duration % 60;

  let display = "";

  if (hoursPart > 0) {
    display = `${hoursPart}h`;
  }

  if (minutesPart > 0) {
    if (display !== "") {
      display = display.concat(" ");
    }

    display = display.concat(`${minutesPart}m`);
  }

  return display;
}
</script>
