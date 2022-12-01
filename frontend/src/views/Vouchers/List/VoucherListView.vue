<template>
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
