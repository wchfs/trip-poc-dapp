<template>
  <ElForm
    @submit.prevent
    ref="addTicketFormRef"
    :model="addTicketForm"
    :rules="rules"
    size="large"
  >
    <div class="flex flex-col">
      <div class="flex flex-row">
        <ElFormItem
          prop="date"
        >
          <ElDatePicker
            v-model="addTicketForm.date"
            type="datetime"
            placeholder="Start at"
            format="YYYY-MM-DD HH:mm"
          />
        </ElFormItem>
        <ElFormItem
          prop="duration"
        >
          <ElTimeSelect
            v-model="addTicketForm.duration"
            start="00:15"
            end="23:45"
            step="00:15"
            placeholder="Duration"
          />
        </ElFormItem>
      </div>
      <div class="flex flex-col lg:flex-row justify-between">
        <ElFormItem
          prop="plate_number"
        >
          <ElInput
            v-model="addTicketForm.plate_number"
            placeholder="Plate number"
            :clearable="true"
            :formatter="(value) => formatPlateNumber(value)"
            :parser="(value) => formatPlateNumber(value)"
          />
        </ElFormItem>
        <ElFormItem>
          <ElButton
            type="danger"
            @click="submitForm(addTicketFormRef)"
          >
            Buy ticket
          </ElButton>
        </ElFormItem>
      </div>
    </div>
  </ElForm>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue';
import type { FormInstance, FormRules } from 'element-plus';
import { ElMessageBox } from 'element-plus';
import { useLocationStore } from '@/stores/location';
import { useParkingZoneStore } from '@/stores/parking-zone';
import 'element-plus/es/components/message-box/style/css';
import { DateTime } from 'luxon';
import { useParkingTicketStore } from '@/stores/parking-ticket';
import router from '@/router';
import type { ParkingZone } from '@/interfaces/parking-zone';
import { useWalletStore } from '@/stores/wallet';
import { gwei2eth } from '@/helpers/helpers';
import { BigNumber } from 'ethers';

const locationStore = useLocationStore();
const parkingZoneStore = useParkingZoneStore();
const parkingTicketStore = useParkingTicketStore();

const addTicketFormRef = ref<FormInstance>();
const addTicketForm = reactive({
  date: DateTime.now().toJSDate(),
  duration: '01:00',
  plate_number: '',
});

const rules = reactive<FormRules>({
  date: [
    {
      type: 'date',
      required: true,
      message: 'Please pick a date',
      trigger: 'change',
    },
  ],
  duration: [
    {
      required: true,
      message: 'Please pick a duration',
      trigger: 'change',
    },
  ],
  plate_number: [
    {
      required: true,
      message: 'Please enter your plate number',
      trigger: 'change',
    },
    {
      min: 3,
      max: 10,
      message: 'Length should be 3 to 10',
      trigger: 'blur',
    },
  ],
});

const submitForm = async (formEl: FormInstance | undefined) => {
  if (!formEl) return;

  await formEl.validate((valid, fields) => {
    if (!valid) {
      console.log('error submit!', fields);

      return;
    }

    sendToRollup();
  })
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

  ElMessageBox.confirm(
    `You should pay ${ price } ETH. Continue?`,
    'Info', {
      confirmButtonText: 'OK',
      cancelButtonText: 'Cancel',
      type: 'info',
    }
  ).then(() => {
    executeDepositConfirmed(duration, price);
  }).catch(() => {
    executeDepositDeclined();
  });
};

const executeDepositConfirmed = (duration: number, price: string) => {
  const startDate = DateTime.fromJSDate(addTicketForm.date);

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
  const price = BigNumber.from(zone.price).div(60).mul(duration);

  return gwei2eth(price.toString());
}

function getDurationFromTimeString(time: string): number {
  const [
    hours, minutes
  ] = time.split(':').map((e) => parseInt(e));

  return (hours * 60) + minutes;
}

function formatPlateNumber(value: string) {
  return value.toUpperCase().replace(/[\W_]+/g, '');
}
</script>
