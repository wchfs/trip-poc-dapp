<template>
  <Header />
  <main>
    <Hero />
    <PrimaryFeatures />
    <SecondaryFeatures />
    <CallToAction />
    <Faqs />
  </main>
</template>

<script setup lang="ts">
import CallToAction from "@/components/Landing/CallToAction.vue";
import Faqs from "@/components/Landing/Faqs.vue";
import Header from "@/components/Landing/Header.vue";
import Hero from "@/components/Landing/Hero.vue";
import PrimaryFeatures from "@/components/Landing/PrimaryFeatures.vue";
import SecondaryFeatures from "@/components/Landing/SecondaryFeatures.vue";
</script>
