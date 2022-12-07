<template>
  <BaseContainer>
    <Box
      additionalClass="col-span-3 border border-indigo-600 mb-5"
    >
      <div
        class="
          flex
          justify-center
        "
      >
        <p v-if="voucherStore.waitingForVoucher" class="text-indigo-900 text-center animate-pulse">
          Some voucher is already on the way, please wait...
        </p>
        <p v-else class="text-indigo-900 text-center">
          If you do not see your just-generated voucher, wait a moment and then hit
          <ElButton
            type="success"
            @click="voucherStore.fetchVouchers(true)"
          >
            reload vouchers
          </ElButton>
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
import VoucherListItem from "@/views/Vouchers/List/VoucherListItem.vue";
import { useVoucherStore } from "@/stores/voucher";
import { storeToRefs } from "pinia";
import { onMounted } from "vue";

const voucherStore = useVoucherStore();
const { vouchers } = storeToRefs(voucherStore);

onMounted(() => {
  voucherStore.fetchVouchers();
});
</script>
