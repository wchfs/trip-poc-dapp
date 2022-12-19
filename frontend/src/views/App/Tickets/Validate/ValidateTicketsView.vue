<template>
  <HelperContainer :message="helpMessage" />
  <BaseContainer>
    <Box additionalClass="col-span-1 sm:col-span-2 lg:col-span-1 lg:col-start-2 mb-5">
      <form @submit.prevent="submitForm">
        <div class="flex flex-col">
          <div class="flex flex-col gap-2">
            <div class="flex flex-col gap-2">
              <TInput
                class="grow"
                type="text"
                placeholder="Plate number"
                v-model="validateTicketForm.plate_number"
                size="sm"
                :required="true"
                :inputCallback="formatPlateNumber"
              />
              <TButton
                class="grow"
                color="indigo"
                type="submit"
              >
                Check ticket
              </TButton>
            </div>
          </div>
        </div>
      </form>
    </Box>
  </BaseContainer>
  <BaseContainer v-if="result !== null">
    <ParkingTicketBox
      v-if="(result.error === null && result.data !== null)"
      :ticket="result.data"
      :zone="result.data.zone"
    />
  </BaseContainer>
</template>

<script setup lang="ts">
import Box from "@/components/Box/Box.vue";
import BaseContainer from "@/components/Containers/BaseContainer.vue";
import ParkingTicketBox from "@/components/ParkingTicket/ParkingTicketBox.vue";
import type { ParkingTicket } from "@/interfaces/parking-ticket";
import type { Error, RollupResponseDecodedPayload } from "@/interfaces/rollup-api";
import { RollupService } from "@/services/rollup-service";
import { useRollupStore } from "@/stores/rollup";
import { DateTime } from "luxon";
import type { Ref } from "vue";
import { reactive, ref } from "vue";
import TButton from "@/components/Controls/Button/TButton.vue";
import TInput from "@/components/Controls/Input/TInput.vue";
import HelperContainer from '@/components/Containers/HelperContainer.vue';

const validateTicketForm = reactive({
  plate_number: "",
});

const rollupStore = useRollupStore();

const formatPlateNumber = (value?: string): string => {
  if (!value) {
    return '';
  }

  return value.toUpperCase().replace(/[\W_]+/g, '');
};

const submitForm = async () => {
  sendInspect();
};

const result: Ref<RollupResponseDecodedPayload<ParkingTicket> | null> = ref(null);

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
        if (report.error) {
          rollupStore.addError(report.error);
        }
        
        result.value = report;
      });
    })
    .catch((error: Error) => {
      console.log(error); // TODO handle it
    });
}

const helpMessage = `Based on the plate number this validator can return information.
If "There is no valid ticket available" or just display the ticket.`;
</script>
