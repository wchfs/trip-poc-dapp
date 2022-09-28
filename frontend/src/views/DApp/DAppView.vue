<template>
  <BaseContainer>
    <div class="flex flex-col gap-5">
      <DAppViewSidebar/>
    </div>
    <Box additionalClass="col-span-2">
      <Map/>
    </Box>
  </BaseContainer>
</template>

<script setup lang="ts">
import BaseContainer from '@/components/Containers/BaseContainer.vue';
import Box from '@/components/Box/Box.vue';
import Map from '@/components/Map/Map.vue';
import DAppViewSidebar from '@/views/DApp/DAppViewSidebar.vue';
</script>
