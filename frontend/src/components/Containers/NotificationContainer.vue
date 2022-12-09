<template>
  <!-- Global notification live region, render this permanently at the end of the document -->
  <div
    aria-live="assertive"
    class="pointer-events-none fixed inset-0 flex items-end px-4 py-6 sm:items-start sm:p-6"
  >
    <div class="flex w-full flex-col items-center space-y-4 sm:items-end">
      <!-- Notification panel, dynamically insert this into the live region when it needs to be displayed -->
      <transition
        v-for="error in rollupStore.rollupErrors"
        enter-active-class="transform ease-out duration-300 transition"
        enter-from-class="translate-y-2 opacity-0 sm:translate-y-0 sm:translate-x-2"
        enter-to-class="translate-y-0 opacity-100 sm:translate-x-0"
        leave-active-class="transition ease-in duration-100"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <TNotification
          title="Rollup Error"
          :message="error"
        />
      </transition>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRollupStore } from '@/stores/rollup';
import TNotification from '../Common/TNotification/TNotification.vue';

const rollupStore = useRollupStore();
</script>
