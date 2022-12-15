<template>
  <Header />
  <main>
    <Hero />
    <PrimaryFeatures />
    <SecondaryFeatures />
  </main>
</template>

<script setup lang="ts">
import Header from "@/components/Landing/Header.vue";
import Hero from "@/components/Landing/Hero.vue";
import PrimaryFeatures from "@/components/Landing/PrimaryFeatures.vue";
import SecondaryFeatures from "@/components/Landing/SecondaryFeatures.vue";
</script>
