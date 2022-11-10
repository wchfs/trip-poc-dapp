<template>
  <footer class="bg-gray-800 mt-6">
    <div class="mx-auto max-w-7xl py-12 px-4 sm:px-6 md:flex md:items-center md:justify-between lg:px-8">
      <div class="mt-8 md:order-1 md:mt-0">
        <p class="text-center text-base text-gray-400">&copy; {{ DateTime.now().year }} Webchefs with Cartesi using the
          blockchain OS</p>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { DateTime } from 'luxon';</script>
