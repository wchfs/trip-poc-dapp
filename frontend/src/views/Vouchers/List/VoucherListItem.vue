<template>
  <Box additionalClass="p-0">
    <div class="flex w-full items-center justify-between space-x-6 p-6">
      <div class="flex-1 truncate">
        <div class="flex items-center space-x-3">
          <h3 class="truncate text-sm font-medium text-gray-900">
            Voucher #{{ voucher.id }}
          </h3>
          <TBadge :color="getBadgeColor(props.voucher)">
            {{ props }}
          </TBadge>
        </div>
      </div>
    </div>
  </Box>
</template>

<script setup lang="ts">
import Box from "@/components/Box/Box.vue";
import TBadge from "@/components/Common/TBadge/TBadge.vue";
import type { Voucher } from "@/interfaces/voucher";

const props = defineProps<{
  voucher: Voucher;
}>();

function getBadgeColor(voucher: Voucher) {
  switch (voucher) {
    case voucher:
      return "green";
    default:
      return "indigo";
  }
}
</script>
