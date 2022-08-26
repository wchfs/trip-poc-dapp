<template>
  <BaseContainer
    v-if="true/*walletStore.onboard?.connectedWallet?.value*/"
    caption="Parking DApp"
  >
    <Box
      additionalClass="col-span-3"
    >
      <Map/>
    </Box>
  </BaseContainer>
  <BaseContainer
    v-else
  >
    <Box
      additionalClass="col-span-3"
    >
      <el-alert
        title="You have to connect with your wallet first"
        type="warning"
        center
        :closable="false"
        show-icon
      />
    </Box>
  </BaseContainer>
</template>

<script setup lang="ts">
import BaseContainer from '@/components/Containers/BaseContainer.vue';
import Box from '@/components/Box/Box.vue';
import { useWalletStore } from '@/stores/wallet';
import Map from '@/components/Map/Map.vue';

const walletStore = useWalletStore();
</script>
