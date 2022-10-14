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
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router';
import NavMenu from '@/components/Nav/NavMenu.vue';
import { useWalletStore } from '@/stores/wallet';
import { watch } from 'vue';

const walletStore = useWalletStore();

watch(walletStore, async (state) => {
  walletStore.setLastConnectedWallet(state.connectedWallet);
});
</script>
