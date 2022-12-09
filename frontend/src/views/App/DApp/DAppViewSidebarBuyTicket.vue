<template>
  <ConfirmDialog ref="confirmDialogRef" />
  <form @submit.prevent="submitForm()">
    <div class="flex flex-col">
      <div class="flex flex-col gap-2">
        <div class="flex flex-row gap-2">
          <TInput
            type="text"
            v-model="addTicketForm.date"
            size="sm"
            :disabled="true"
          >
            <template #left>
              <TInputLeftSide
                size="sm"
                :disabled="true"
              >
                Start
              </TInputLeftSide>
            </template>
          </TInput>
          <TSelect
            class="grow"
            v-model="addTicketForm.duration"
            placeholder="Duration"
            :items="durationItems"
            size="sm"
          />
        </div>
        <div class="flex flex-col md:flex-row gap-2">
          <TInput
            class="grow"
            type="text"
            placeholder="Plate number"
            v-model="addTicketForm.plate_number"
            size="sm"
            :required="true"
            :inputCallback="formatPlateNumber"
          />
          <TButton
            class="grow"
            color="indigo"
            type="submit"
          >
            Buy ticket
          </TButton>
        </div>
      </div>
    </div>
  </form>
</template>

<script setup lang="ts">
import TButton from '@/components/Controls/Button/TButton.vue';
import TInputLeftSide from '@/components/Controls/Input/Partials/Sides/Left/TInputLeftSide.vue';
import TInput from '@/components/Controls/Input/TInput.vue';
import TSelect, { SelectOption } from '@/components/Controls/Select/TSelect.vue';
import ConfirmDialog from '@/components/Dialogs/ConfirmDialog.vue';
import { gwei2eth } from '@/helpers/helpers';
import type { ParkingZone } from '@/interfaces/parking-zone';
import router from '@/router';
import { useLocationStore } from '@/stores/location';
import { useParkingTicketStore } from '@/stores/parking-ticket';
import { useParkingZoneStore } from '@/stores/parking-zone';
import { useWalletStore } from '@/stores/wallet';
import { TicketIcon } from '@heroicons/vue/24/outline';
import { BigNumber } from 'ethers';
import { range } from 'lodash';
import { DateTime } from 'luxon';
import { onMounted, reactive, ref } from 'vue';

const locationStore = useLocationStore();
const parkingZoneStore = useParkingZoneStore();
const parkingTicketStore = useParkingTicketStore();

const confirmDialogRef = ref<InstanceType<typeof ConfirmDialog> | null>(null);

const addTicketForm = reactive({
  date: DateTime.now().toFormat('yyyy-MM-dd HH:mm:ss'),
  duration: '01:00',
  plate_number: '',
});

const durationItems = range(1, 23).map<SelectOption>((hour) => {
  const hourString = hour < 10 ? `0${hour}` : hour.toString();

  return {
    value: `${hourString}:00`,
    label: `${hourString}:00`,
  };
});

const dateFormat = 'yyyy-MM-dd HH:mm:ss';

onMounted(() => {
  setInterval(() => {
    addTicketForm.date = DateTime.now().toFormat(dateFormat);
  }, 1000);
});


const submitForm = async () => {
  sendToRollup();
};

const sendToRollup = async () => {
  const selectedZone = parkingZoneStore.selectedZone;

  if (!selectedZone) {
    return null;
  }

  const duration = getDurationFromTimeString(addTicketForm.duration);
  const price = calculatePrice(selectedZone, duration);

  if (!price) {
    return; // TODO throw error
  }

  confirmDialogRef.value?.open({
    confirmed: () => {
      executeDepositConfirmed(duration, price);
    },
    canceled: () => {
      executeDepositDeclined();
    },
  }, {
    color: 'green',
    icon: TicketIcon,
    title: 'Buy ticket',
    message: `You should pay ${price} ETH. Continue?`,
    confirmButtonText: 'Yes, buy ticket',
    cancelButtonText: 'Cancel',
  });
};

const executeDepositConfirmed = (duration: number, price: string) => {
  const startDate = DateTime.fromFormat(addTicketForm.date, dateFormat);

  const selectedZone = parkingZoneStore.selectedZone;
  const walletAddress = useWalletStore().walletAddress;

  if (!selectedZone || walletAddress === null) {
    return;
  }

  parkingTicketStore.buyTicket({
    license: addTicketForm.plate_number,
    latitude: locationStore.markerPosition?.lat as number,
    longitude: locationStore.markerPosition?.lng as number,
    started_at: startDate.toUTC().toISO(),
    duration: duration,
    zone_id: selectedZone.id as number,
    owner_address: walletAddress,
  }, price);

  router.push({
    name: 'dapp.tickets.my',
  });
};

const executeDepositDeclined = () => {
  // TODO
};

function calculatePrice(zone: ParkingZone, duration: number): string {
  const price = BigNumber.from(zone.price).toNumber() * (duration / 60);

  return gwei2eth(price.toString());
}

function getDurationFromTimeString(time: string): number {
  const [
    hours, minutes
  ] = time.split(':').map((e) => parseInt(e));

  return (hours * 60) + minutes;
}

const formatPlateNumber = (value?: string): string => {
  if (!value) {
    return '';
  }

  return value.toUpperCase().replace(/[\W_]+/g, '');
};
</script>
