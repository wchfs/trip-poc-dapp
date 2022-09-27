<template>
  <div class="flex flex-col space-y-8">
    <SummaryViewSummary/>
    <SummaryViewCommunication
      :onboard="walletStore.onboard"
    />
  </div>
</template>

<script setup lang="ts">
import SummaryViewCommunication from '@/views/Summary/SummaryViewCommunication.vue';
import SummaryViewSummary from '@/views/Summary/SummaryViewSummary.vue';
import { useWalletStore } from '@/stores/wallet';

const walletStore = useWalletStore();
</script>
