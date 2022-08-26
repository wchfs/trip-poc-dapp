<template>
  <div class="flex flex-col space-y-8">
    <HomeViewSummary/>
    <HomeViewCommunication
      v-if="walletStore.onboard?.connectedWallet?.value"
      :onboard="walletStore.onboard"
    />
  </div>
</template>

<script setup lang="ts">
import HomeViewCommunication from '@/views/Home/HomeViewCommunication.vue';
import HomeViewSummary from '@/views/Home/HomeViewSummary.vue';
import { useWalletStore } from '@/stores/wallet';

const walletStore = useWalletStore();
</script>
