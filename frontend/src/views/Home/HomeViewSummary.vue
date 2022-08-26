<template>
  <BaseContainer
    caption="App summary"
  >
    <InfoBox
      topText="Connected wallet"
      :featuredText="walletStore.onboard?.connectedWallet?.value?.label"
      bottomText="Your wallet provider name"
      icon="fa-solid fa-wallet"
      textColor="text-blue-500"
      bgColor="bg-blue-500"
      :showButton="!walletStore.onboard?.connectedWallet?.value"
      buttonText="CONNECT"
      @buttonClick="connectWallet()"
    />
    <InfoBox
      topText="Connected chain"
      :featuredText="walletStore.onboard?.connectedChain?.value?.id"
      bottomText="Hex id of connected chain"
      icon="fa-solid fa-link"
      textColor="text-green-500"
      bgColor="bg-green-500"
      :showButton="!walletStore.onboard?.connectedChain?.value"
    />
    <InfoBox
      topText="DApp address"
      :featuredText="dappAddress"
      bottomText="Your DApp backend address"
      icon="fa-solid fa-location-crosshairs"
      textColor="text-yellow-500"
      bgColor="bg-yellow-500"
    />
  </BaseContainer>
</template>

<script setup lang="ts">
import InfoBox from '@/components/Box/InfoBox.vue';
import BaseContainer from '@/components/Containers/BaseContainer.vue';
import { useWalletStore } from '@/stores/wallet';

const walletStore = useWalletStore();

async function connectWallet() {
  await walletStore.connectWallet();
}

const dappAddress = import.meta.env.VITE_APP_DAPP_ADDRESS as string;
</script>
