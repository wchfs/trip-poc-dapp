<template>
  <Box additionalClass="p-0">
    <div class="flex w-full items-center justify-between space-x-6 p-6">
      <div class="flex-1 truncate">
        <div class="flex items-center space-x-3">
          <h3 class="truncate text-sm font-medium text-gray-900">
            Voucher #{{ voucher.id }}
          </h3>
          <TBadge :color="getBadgeColor()">
            {{ props.voucher.status ?? "checking" }}
          </TBadge>
        </div>
        <p class="mt-1 truncate text-sm text-gray-500">
          {{ props.voucher.owner_address }}
        </p>
      </div>
    </div>
    <div>
      <div class="-mb-0 flex divide-x divide-gray-200">
        <div class="flex w-0 flex-1">
          <button
            :disabled="props.voucher.status != 'approved'"
            @click="execute()"
            class="relative inline-flex w-0 flex-1 items-center justify-center rounded-br-lg border border-transparent py-4 text-sm font-medium text-gray-700"
          >
            <Bars3BottomLeftIcon class="h-5 w-5 txt-gray-400" aria-hidden="true" />
            <span class="ml-3">Execute Voucher</span>
          </button>
        </div>
      </div>
    </div>
  </Box>
</template>

<script setup lang="ts">
import Box from "@/components/Box/Box.vue";
import TBadge from "@/components/Common/TBadge/TBadge.vue";
import type { Voucher } from "@/interfaces/voucher";
import { RollupService } from "@/services/rollup-service";
import type { OutputValidityProofStruct } from "@cartesi/rollups/dist/src/types/contracts/interfaces/IOutput";
import { Bars3BottomLeftIcon } from "@heroicons/vue/20/solid";

const props = defineProps<{
  voucher: Voucher;
}>();

function getBadgeColor(): "red" | "green" | "indigo" {
  switch (props.voucher.status) {
    case undefined:
      return "red";
    case "approved":
      return "green";
    default:
      return "indigo";
  }
}

async function execute() {
  // bunch of validation (test purposes)
  if (!props.voucher.generated_voucher) {
    throw new Error("no voucher");
  }

  if (!props.voucher.generated_voucher.proof) {
    throw new Error("no proof");
  }

  if (!props.voucher.generated_voucher.payload) {
    throw new Error("no payload");
  }

  if (!props.voucher.generated_voucher.destination) {
    throw new Error("no destination");
  }

  if (props.voucher.generated_voucher.proof.machineStateHash == "0x") {
    throw new Error("no machine state hash (host mode)");
  }

  const proof: OutputValidityProofStruct = {
    ...props.voucher.generated_voucher.proof,
    epochIndex: props.voucher.epoch_index,
    inputIndex: props.voucher.input_index,
    outputIndex: props.voucher.voucher_index,
  };

  RollupService.getContracts()
    .outputContract.executeVoucher(
      props.voucher.generated_voucher.destination,
      props.voucher.generated_voucher.payload,
      proof
    )
    .then(async (tx) => {
      const receipt = await tx.wait();

      console.log(`voucher executed! (tx="${tx.hash}")`);

      if (receipt.events) {
        console.log(`resulting events: ${JSON.stringify(receipt.events)}`);
      }
    })
    .catch((value) => {
      console.error(value);
    });
}
</script>
