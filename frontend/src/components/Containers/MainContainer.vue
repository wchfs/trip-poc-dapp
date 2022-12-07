<template>
  <main class="container mx-w-6xl mx-auto py-4">
    <div
      v-if="walletStore.connectingWallet"
      class="flex items-center justify-center h-40"
    >
      <div class="flex space-x-2 animate-pulse">
        <div class="w-3 h-3 bg-gray-500 rounded-full"></div>
        <div class="w-3 h-3 bg-gray-500 rounded-full"></div>
        <div class="w-3 h-3 bg-gray-500 rounded-full"></div>
      </div>
    </div>
    <slot v-else />
  </main>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router';
import { watch } from 'vue';
import { useWalletStore } from '@/stores/wallet';
import { useOnboard } from '@web3-onboard/vue';
import router from '@/router';

const walletStore = useWalletStore();

const onboard = useOnboard();

let startWatchingDisconnect = false;

watch(onboard.alreadyConnectedWallets, (newState, oldState) => {
  if (startWatchingDisconnect && newState.length === 0) {
    walletStore.clearLastConnectedWallet();

    router.push({
      name: 'dapp.connect',
    });

    return;
  }

  if (newState.length > 0 && oldState.length > 0) {
    startWatchingDisconnect = true;
  }

  walletStore.setLastConnectedWallet(newState[0]);
});
</script>
