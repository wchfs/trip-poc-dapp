<template>
  <NavMenu/>
  <main class="container mx-w-6xl mx-auto py-4">
    <RouterView/>
  </main>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router';
import NavMenu from '@/components/Nav/NavMenu.vue';
import { useWalletStore } from '@/stores/wallet';
import { watch } from 'vue';
import router from '@/router';

const walletStore = useWalletStore();

watch(walletStore, async (state) => {
  if (state.walletAddress !== null) {
    return;
  }

  await router.push({
    name: 'root',
  });
});
</script>
