<template>
      <section
      id="get-started-today"
      class="relative overflow-hidden bg-blue-600 py-32"
    >
      <img
        className="absolute top-1/2 left-1/2 max-w-none -translate-x-1/2 -translate-y-1/2"
        src="/images/background-call-to-action.jpg"
        alt=""
        width=2347
        height=1244
        unoptimized
      />
      <Container className="relative">
        <div class="mx-auto max-w-lg text-center">
          <h2 class="font-display text-3xl tracking-tight text-white sm:text-4xl">
            Start testing today
          </h2>
          <p className="mt-4 text-lg tracking-tight text-white">
            This is demo application using cartesi technology.
            If you are interested in building a truly decentralized application on the blockchain with cartesi tech please contact Webchefs Software House.
            If you are dying to know more about the blockchain OS - explore cartesi.
          </p>
          <ul
            role="list"
            class="flex items-center flex-row gap-x-12 justify-center"
          >
            <li class="flex">
              <a href="https://cartesi.io/" class="mt-10
                bg-white text-slate-900 hover:bg-blue-50 active:bg-blue-200 active:text-slate-600 focus-visible:outline-white
                group inline-flex items-center justify-center rounded-full py-2 px-4 text-sm font-semibold focus:outline-none focus-visible:outline-2 focus-visible:outline-offset-2">
                Explore Cartesi
              </a>
            </li>
            <li>
              <a href="https://webchefs.tech/" class="mt-10
                bg-white text-slate-900 hover:bg-blue-50 active:bg-blue-200 active:text-slate-600 focus-visible:outline-white
                group inline-flex items-center justify-center rounded-full py-2 px-4 text-sm font-semibold focus:outline-none focus-visible:outline-2 focus-visible:outline-offset-2">
                Explore Webchefs
              </a>
            </li>
          </ul>
        </div>
      </Container>
    </section>
</template>

<script setup lang="ts">
import Container from "@/components/Landing/Container.vue";
import Button from "@/components/Landing/Button.vue";
</script>
