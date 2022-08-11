<template>
  <div>
    Connected wallet: {{ walletStore.onboard.connectedWallet.value?.label }}
    <br>
    Connected chain: ID: {{ walletStore.onboard.connectedChain.value?.id }} NAMESPACE: {{ walletStore.onboard.connectedChain.value?.namespace }}
  </div>
  <input type="text" v-model="input">
  <br>
  <button @click="chainAddInput()">Add Input</button>
</template>

<script setup lang="ts">
import { useWalletStore } from '@/stores/wallet';
import { ref } from 'vue';
import { useRollupStore } from '@/stores/rollup';

const rollupStore = useRollupStore();
const walletStore = useWalletStore();

walletStore.setup();
walletStore.connect();

let input = ref('');

function chainAddInput() {
  rollupStore.setup();

  console.log(input.value);
}
</script>
