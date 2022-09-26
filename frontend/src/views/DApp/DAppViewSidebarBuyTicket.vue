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
import { useLocationStore } from '@/stores/location';
import { useParkingZoneStore } from '@/stores/parking-zone';
import { RollupService } from '@/services/rollup-service';
import { ElMessageBox } from 'element-plus';
import type { ParkingTicket } from '@/interfaces/parking-ticket';
import { DateTime } from 'luxon';

const locationStore = useLocationStore();
const parkingZoneStore = useParkingZoneStore();

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
  ElMessageBox.confirm(
    `You should pay ${parkingZoneStore.selectedZone?.price} ETH. Continue?`,
    'Info', {
      confirmButtonText: 'OK',
      cancelButtonText: 'Cancel',
      type: 'info',
    }
  ).then(() => {
    executeDepositConfirmed();
  }).catch(() => {
    executeDepositDeclined();
  });
};

const executeDepositConfirmed = async () => {
  const transactionResponse = await RollupService.addInput<ParkingTicket>({
    endpoint: 'buy_ticket',
    payload: {
      Ticket: {
        Buy: {
          license: addTicketForm.plate_number,
          latitude: locationStore.markerPosition?.lat,
          longitude: locationStore.markerPosition?.lng,
          started_at: "",
          duration: 60,
          zone_id: parkingZoneStore.selectedZone?.id,
        }
      }
    },
  }, parkingZoneStore.selectedZone?.price);

  transactionResponse.response.then((r) => {
    console.log(r);
  });
};

const executeDepositDeclined = () => {

};

function formatPlateNumber(value: string) {
  return value.toUpperCase().replace(/[\W_]+/g, '');
}
</script>
