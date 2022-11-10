<template>
  <div
    @mouseenter="hover = true"
    @mouseleave="hover = false"
  >
    <InfoBox
      :additionalClass="`
        col-span-1
        ${ props.hoverColor ? 'hover:bg-gray-100 hover:cursor-pointer' : '' }
      `"
      :topText="`Hourly rate for ${ props.zone.name }`"
      :featuredText="`${ gwei2eth(props.zone.price.toString()) } ETH`"
      :bottomText="`Identifier: ${ props.zone.id }`"
      :icon="GlobeAltIcon"
      textColor="text-blue-500"
      :showButton="props.showButtonMode === 'always' ? props.showButton : props.showButton && hover"
      buttonText="Details"
      buttonColor="indigo"
      @buttonClick="emit('buttonClick')"
    />
  </div>
</template>

<script setup lang="ts">
import InfoBox from '@/components/Box/InfoBox.vue';
import { gwei2eth } from '@/helpers/helpers';
import { GlobeAltIcon } from '@heroicons/vue/24/outline';
import type { ParkingZone } from '@/interfaces/parking-zone';
import { ref } from 'vue';

const hover = ref(false);

const props = withDefaults(defineProps<{
  zone: ParkingZone,
  hoverColor?: boolean,
  showButton?: boolean,
  showButtonMode?: 'always' | 'hover',
  showManageButtons?: boolean,
}>(), {
  hoverColor: false,
  showButton: false,
  showManageButtons: false,
  showButtonMode: 'always',
});

const emit = defineEmits<{
  (e: 'buttonClick'): void;
}>();

</script>
