<template>
  <NavMenu/>
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
    <RouterView
      v-else
    />
  </main>
  <footer class="bg-gray-800">
    <div class="mx-auto max-w-7xl py-12 px-4 sm:px-6 md:flex md:items-center md:justify-between lg:px-8">
      <div class="mt-8 md:order-1 md:mt-0">
        <p class="text-center text-base text-gray-400">&copy; {{ DateTime.now().year }} Webchefs with Cartesi using the blockchain OS</p>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router';
import { watch } from 'vue';
import { useWalletStore } from '@/stores/wallet';
import { useOnboard } from '@web3-onboard/vue';
import router from '@/router';
import NavMenu from '@/components/Nav/NavMenu.vue';
import { DateTime } from 'luxon';

const walletStore = useWalletStore();

const onboard = useOnboard();

let startWatchingDisconnect = false;

watch(onboard.alreadyConnectedWallets, (newState, oldState) => {
  if (startWatchingDisconnect && newState.length === 0) {
    walletStore.clearLastConnectedWallet();

    router.push({
      name: 'root',
    });

    return;
  }

  if (newState.length > 0 && oldState.length > 0) {
    startWatchingDisconnect = true;
  }

  walletStore.setLastConnectedWallet(newState[0]);
});
</script>
