<template>
  <BaseContainer>
    <Box additionalClass="col-span-1 md:col-start-2 mb-5">
      <ElForm
        @submit.prevent
        ref="validateTicketFormRef"
        :model="validateTicketForm"
        :rules="rules"
        size="large"
      >
        <div class="flex flex-col">
          <div class="flex flex-col lg:flex-row justify-between">
            <ElFormItem prop="plate_number">
              <ElInput
                v-model="validateTicketForm.plate_number"
                placeholder="Plate number"
                :clearable="true"
                :formatter="(value) => formatPlateNumber(value)"
                :parser="(value) => formatPlateNumber(value)"
              />
            </ElFormItem>
            <ElFormItem>
              <ElButton type="danger" @click="submitForm(validateTicketFormRef)">
                Get ticket
              </ElButton>
            </ElFormItem>
          </div>
        </div>
      </ElForm>
    </Box>
  </BaseContainer>
  <BaseContainer v-if="result !== null">
    <Box v-if="result.hasOwnProperty('error')" additionalClass="col-span-3">
      {{ result.error }}
    </Box>
    <ParkingTicketBox v-else :ticket="result" :zone="result.zone" />
  </BaseContainer>
</template>

<script setup lang="ts">
import BaseContainer from "@/components/Containers/BaseContainer.vue";
import Box from "@/components/Box/Box.vue";
import type { Ref } from "vue";
import { reactive, ref } from "vue";
import type { FormInstance, FormRules } from "element-plus";
import { RollupService } from "@/services/rollup-service";
import type { Error } from "@/interfaces/rollup-api";
import type { ParkingTicket } from "@/interfaces/parking-ticket";
import ParkingTicketBox from "@/components/ParkingTicket/ParkingTicketBox.vue";
import { DateTime } from "luxon";

const validateTicketFormRef = ref<FormInstance>();
const validateTicketForm = reactive({
  plate_number: "",
});

const rules = reactive<FormRules>({
  plate_number: [
    {
      required: true,
      message: "Please enter plate number",
      trigger: "change",
    },
    {
      min: 3,
      max: 10,
      message: "Length should be 3 to 10",
      trigger: "blur",
    },
  ],
});

function formatPlateNumber(value: string) {
  return value.toUpperCase().replace(/[\W_]+/g, "");
}

const submitForm = async (formEl: FormInstance | undefined) => {
  if (!formEl) return;

  await formEl.validate((valid, fields) => {
    if (!valid) {
      console.log("error submit!", fields);

      return;
    }

    sendInspect();
  });
};

const result: Ref<ParkingTicket | null> = ref(null);

function sendInspect() {
  RollupService.inspect<ParkingTicket>({
    endpoint: "validate_ticket",
    payload: {
      Ticket: {
        Validate: {
          license: validateTicketForm.plate_number,
          date: DateTime.now().toJSDate().toISOString(),
        },
      },
    },
  })
    .then((reports) => {
      reports.forEach((report) => {
        result.value = report.data;
      });
    })
    .catch((error: Error) => {
      console.log(error); // TODO handle it
    });
}
</script>
