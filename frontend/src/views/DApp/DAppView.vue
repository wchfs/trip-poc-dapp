<template>
  <BaseContainer
    v-if="walletStore.onboard?.connectedWallet?.value"
  >
    <div class="flex flex-col gap-5">
      <DAppViewSidebar/>
    </div>
    <Box additionalClass="col-span-2">
      <Map/>
    </Box>
  </BaseContainer>
  <BaseContainer v-else>
    <Box>
      <div class="space-y-4 md:space-y-6 sm:p-5">
        <div class="text-xl text-center font-bold leading-tight tracking-tight text-gray-900 md:text-2xl">
          Connect your wallet
        </div>
        <button
          type="submit"
          class="
              w-full
              text-white
              bg-green-600
              hover:bg-green-700
              font-semibold
              rounded-lg
              text-sm
              px-5
              py-2.5
              text-center
          "
          @click="connectWallet()"
        >
          Connect
        </button>
        <form class="space-y-4 md:space-y-6" action="#">

          <p class="text-sm font-light text-gray-500 text-center">
            Dapp powered by <a
              href="https://cartesi.io"
              target="_blank"
              class="font-medium text-green-600 hover:underline"
          >Cartesi</a>
          </p>
        </form>
      </div>
    </Box>
  </BaseContainer>
</template>

<script setup lang="ts">
import BaseContainer from '@/components/Containers/BaseContainer.vue';
import Box from '@/components/Box/Box.vue';
import { useWalletStore } from '@/stores/wallet';
import Map from '@/components/Map/Map.vue';
import DAppViewSidebar from '@/views/DApp/DAppViewSidebar.vue';

const walletStore = useWalletStore();

async function connectWallet() {
  await walletStore.connectWallet();
}
</script>
