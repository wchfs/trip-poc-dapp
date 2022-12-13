<template>
  <header class="py-10">
    <Container>
      <nav class="relative z-50 flex justify-between">
        <div class="flex items-center md:gap-x-12">
          <RouterLink to="" aria-label="Home">
            <Logo class="h-10 w-auto" />
          </RouterLink>
          <div class="hidden md:flex md:gap-x-6">
            <NavLink to="#features">Features</NavLink>
          </div>
        </div>
        <div class="flex items-center gap-x-5 md:gap-x-8">
          <Button route="dapp" color="blue">Open parking dapp</Button>
          <!-- <div className="-mr-1 md:hidden">
            <MobileNavigation />
          </div> -->
        </div>
      </nav>
    </Container>
  </header>
</template>

<script setup lang="ts">
import Button from "@/components/Landing/Button.vue";
import Container from "@/components/Landing/Container.vue";
import Logo from "@/components/Landing/Logo.vue";
import NavLink from "@/components/Landing/NavLink.vue";
</script>
