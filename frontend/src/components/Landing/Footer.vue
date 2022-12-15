<template>
  <footer class="bg-slate-50">
    <Container>
      <div class="py-16">
        <Logo className="mx-auto h-10 w-auto" />
      </div>
      <div class="flex flex-col items-center border-t border-slate-400/10 py-10 sm:flex-row-reverse sm:justify-between">
        <div class="flex gap-x-6 text-smPrzy text-slate-500">
          <p class="mt-6 sm:mt-0">
            Cartesi:
          </p>
          <a href="https://discord.gg/kfwB7sssn8" class="group" aria-label="Cartesi on Discord">
            Discord
          </a>
          <a href="https://www.twitter.com/cartesiproject" class="group" aria-label="Cartesi on Twitter">
            Twitter
          </a>
          <a href="https://www.github.com/cartesi?_gl=1*1r9yo5r*_ga*MTQ3MjgzMTgwOS4xNjY2NTMyNjk1*_ga_HM92STPNFJ*MTY3MTEyNjgzNy4yLjEuMTY3MTEyODQ5MS41OS4wLjA." class="group" aria-label="Cartesi on GitHub">
            Github
          </a>
          <a href="https://t.me/cartesiannouncements" class="group" aria-label="Cartesi on Telegram">
            Telegram
          </a>
          <a href="https://www.medium.com/cartesi" class="group" aria-label="Cartesi on Medium">
            Medium
          </a>
          <a href="https://www.youtube.com/c/Cartesiproject/videos" class="group" aria-label="Cartesi on YouTube">
            YouTube
          </a>
          <a href="https://www.reddit.com/r/cartesi/" class="group" aria-label="Cartesi on Reddit">
            Reddit
          </a>
          <span>|</span>
          <p class="mt-6 text-slate-500 sm:mt-0">
            Webchefs:
          </p>
          <a href="https://www.facebook.com/wchefs/" class="group" aria-label="Webchefs on Facebook">
            Facebook
          </a>
          <a href="https://www.instagram.com/webchefspl/" class="group" aria-label="Webchefs on Instagram">
            Instagram
          </a>
          <a href="https://www.webchefs.tech/" class="group" aria-label="Webchefs on Web">
            Instagram
          </a>
        </div>
        <p class="mt-6 text-sm text-slate-500 sm:mt-0">
          Copyright &copy;{{ new Date().getFullYear() }} Cartesi. All rights reserved.
        </p>
      </div>
    </Container>
  </footer>
</template>

<script setup lang="ts">
import Container from "@/components/Landing/Container.vue";
import Logo from "@/components/Landing/Logo.vue";
</script>
