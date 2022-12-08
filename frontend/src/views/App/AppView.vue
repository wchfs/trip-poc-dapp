<template>
  <div class="flex flex-col h-screen justify-between">
    <NavMenu />
    <MainContainer class="mb-auto">
      <RouterView />
    </MainContainer>
    <FooterContainer />
  </div>
  <NotificationContainer />
</template>

<script setup lang="ts">
import NavMenu from "@/components/Nav/NavMenu.vue";
import MainContainer from "@/components/Containers/MainContainer.vue";
import FooterContainer from "@/components/Containers/FooterContainer.vue";
import NotificationContainer from "@/components/Containers/NotificationContainer.vue";
</script>
