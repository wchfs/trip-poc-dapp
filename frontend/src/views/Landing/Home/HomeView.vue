<template>
      <RouterLink :to="{ name: 'dapp' }">
            <TButton color="green">Go to dapp</TButton>
      </RouterLink>
</template>

<script setup lang="ts">
import TButton from '@/components/Controls/Button/TButton.vue';

</script>