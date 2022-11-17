<template>
  <ConfirmDialog ref="confirmDialogRef" />
  <TButton class="px-3" @click="execute()">
    <span class="pl-1"> execute </span>
  </TButton>
  <TButton
    :disabled="parseInt(zone.balance ?? '0') <= 0"
    class="px-3"
    color="yellow"
    @click="showWithdrawFundsDialog()"
  >
    <BanknotesIcon class="h-4 w-4 text-white" />
    <span class="pl-1"> Withdraw funds </span>
  </TButton>
  <TButton class="px-3" color="red" @click="showDeleteDialog()">
    <TrashIcon class="h-4 w-4 text-white" />
    <span class="pl-1"> Delete zone </span>
  </TButton>
</template>

<script setup lang="ts">
import TButton from '@/components/Controls/Button/TButton.vue';
import { BanknotesIcon } from '@heroicons/vue/20/solid';
import { QuestionMarkCircleIcon, TrashIcon } from '@heroicons/vue/24/outline';
import type { ParkingZone } from '@/interfaces/parking-zone';
import { useParkingZoneStore } from '@/stores/parking-zone';
import ConfirmDialog from '@/components/Dialogs/ConfirmDialog.vue';
import { ref } from 'vue';
import router from '@/router';
import { gwei2eth } from '@/helpers/helpers';
import { RollupService, type PartialVoucher } from '@/services/rollup-service';
import { type VouchersByEpochAndInputQuery, type VoucherQueryVariables, VoucherDocument, type Voucher, type VoucherQuery } from '@/generated/graphql';
import { ApolloService } from '@/services/apollo-service';
import { ethers } from 'ethers';
import type { GraphQLError } from 'graphql';
import type { OutputValidityProofStruct } from '@/generated/rollups/contracts/facets/OutputFacet';

const props = defineProps<{
  zone: ParkingZone;
}>();

const parkingZoneStore = useParkingZoneStore();

const confirmDialogRef = ref<InstanceType<typeof ConfirmDialog> | null>(null);

async function showDeleteDialog() {
  confirmDialogRef.value?.open({
    confirmed: () => {
      deleteZone();
    },
    canceled: () => {
      // ...
    },
  }, {
    title: 'Delete zone',
    message: 'Are you sure you want to delete this zone?',
    confirmButtonText: 'Yes, delete',
  });
}

async function showWithdrawFundsDialog() {
  confirmDialogRef.value?.open({
    confirmed: () => {
      withdrawFunds();
    },
    canceled: () => {
      console.log('canceled 2');
    },
  },{
    title: 'Withdraw funds',
    icon: QuestionMarkCircleIcon,
    color: 'yellow',
    message: 'Are you sure you want to withdraw funds from this zone?',
    confirmButtonText: `Yes, withdraw (${ gwei2eth(props.zone.balance) } ETH)`,
  });
}

async function deleteZone() {
  await parkingZoneStore.deleteZone(props.zone.id);

  return router.push({
    name: 'dapp.zones.my',
  });
}

async function withdrawFunds() {
  await parkingZoneStore.withdrawFunds(props.zone.id, props.zone.balance ?? '0');

  return router.push({
    name: 'dapp.zones.my',
  });
}

async function execute() {
  const variables: VoucherQueryVariables = {
    id: "1",
  };

  ApolloService.getClient().query<VoucherQuery, VoucherQueryVariables>({
    fetchPolicy: 'no-cache',
    query: VoucherDocument,
    variables,
  }).then((response) => {
    if (response?.data?.voucher) {
      const voucher = response
        .data
        .voucher
        // .filter<PartialVoucher>((n: PartialVoucher | null): n is PartialVoucher => n !== null)[0];

      if (!voucher) {
        return;
      }

      const decodedPayload = ethers.utils.toUtf8String(voucher.payload);

      // const payload_object = JSON.parse(decodedPayload);
      // console.log(payload_object.data);
      if (!voucher.proof) {
        throw new Error("no proof");
      }

      const proof: OutputValidityProofStruct = {
        ...voucher.proof,
        epochIndex: voucher.input.epoch.index,
        inputIndex: voucher.input.index,
        outputIndex: voucher.index,
      };
      console.log(voucher.payload);
      RollupService.getContracts().outputContract.executeVoucher(
        voucher.destination,
        voucher.payload,
        proof
      ).then((value) => {
        console.log(value);
      })
    }
  }).catch((error: GraphQLError) => {
    console.log(error.message);
  });
}
</script>
