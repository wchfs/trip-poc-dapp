<template>
  <HelperContainer :message="helpMessage" />
  <BaseContainer>
    <Box additionalClass="col-span-3 border border-indigo-600 mb-5">
      <div class="
          flex
          justify-center
        ">
        <p
          v-if="voucherStore.waitingForVoucher"
          class="text-indigo-900 text-center animate-pulse"
        >
          Some voucher is already on the way, please wait...
        </p>
        <p
          v-else
          class="text-indigo-900 text-center"
        >
          If you do not see your just-generated voucher, wait a moment and then hit
          <TButton
            @click="voucherStore.fetchVouchers(true)"
            color="green"
          >
            reload vouchers
          </TButton>
        </p>
      </div>
    </Box>
  </BaseContainer>
  <BaseContainer>
    <VoucherListItem
      v-for="voucher of vouchers"
      :key="voucher.id"
      :voucher="voucher"
      :voucherStore="voucherStore"
    />
  </BaseContainer>
</template>

<script setup lang="ts">
import BaseContainer from "@/components/Containers/BaseContainer.vue";
import TButton from "@/components/Controls/Button/TButton.vue";
import { useVoucherStore } from "@/stores/voucher";
import VoucherListItem from "@/views/App/Vouchers/List/VoucherListItem.vue";
import { storeToRefs } from "pinia";
import { onMounted } from "vue";
import Box from "@/components/Box/Box.vue";
import HelperContainer from '@/components/Containers/HelperContainer.vue';

const voucherStore = useVoucherStore();
const { vouchers } = storeToRefs(voucherStore);

onMounted(() => {
  voucherStore.fetchVouchers();
});

const helpMessage = `The vouchers tab is the place where those can be executed.
Pending status informs that voucher is generated successfully.
The process of confirming the ticket can take as long as one week.
Approved status indicated that proofs are generated and vouchers can be safely executed.
After execution, ether should just appear on the wallet.`;
</script>
