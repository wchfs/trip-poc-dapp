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
  //   ApolloService.getClient()
  //     .query<VoucherQuery, VoucherQueryVariables>({
  //       fetchPolicy: "no-cache",
  //       query: VoucherDocument,
  //       variables,
  //     })
  //     .then((response) => {
  //       if (response?.data?.voucher) {
  //         const voucher = response.data.voucher;
  //         // .filter<PartialVoucher>((n: PartialVoucher | null): n is PartialVoucher => n !== null)[0];
  //         if (!voucher) {
  //           return;
  //         }
  //         const decodedPayload = ethers.utils.toUtf8String(voucher.payload);
  //         // const payload_object = JSON.parse(decodedPayload);
  //         // console.log(payload_object.data);
  //         if (!voucher.proof) {
  //           throw new Error("no proof");
  //         }
  //         const proof: OutputValidityProofStruct = {
  //           ...voucher.proof,
  //           epochIndex: voucher.input.epoch.index,
  //           inputIndex: voucher.input.index,
  //           outputIndex: voucher.index,
  //         };
  //         console.log(voucher.payload);
  //         RollupService.getContracts()
  //           .outputContract.executeVoucher(voucher.destination, voucher.payload, proof)
  //           .then((value) => {
  //             console.log(value);
  //           });
  //       }
  //     })
  //     .catch((error: GraphQLError) => {
  //       console.log(error.message);
  //     });
}
</script>
