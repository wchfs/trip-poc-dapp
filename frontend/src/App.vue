<template>
  <NavMenu/>
  <main class="container mx-w-6xl mx-auto py-4">
    <RouterView/>
  </main>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router';
import NavMenu from '@/components/Nav/NavMenu.vue';
</script>

<style scoped>

</style>
