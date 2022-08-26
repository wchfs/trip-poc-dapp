<template>
  <Box>
    <div
      class="
        flex
        justify-between
        items-start
      "
    >
      <div class="flex flex-col overflow-hidden">
        <p class="text-xs text-gray-600 tracking-wide">
          {{ props.topText }}
        </p>
        <h3
          :class="`
            mt-1
            text-lg
            ${props.textColor}
            font-bold
            text-ellipsis
            overflow-hidden
            whitespace-nowrap
          `"
        >
          <el-tooltip
            class="box-item"
            effect="dark"
            :content="props.featuredText"
            placement="bottom"
          >
            {{ props.featuredText }}
          </el-tooltip>
        </h3>
        <span
          class="
            mt-4
            text-xs
            text-gray-500
            text-ellipsis
            overflow-hidden
            whitespace-nowrap
            cursor-pointer
          "
        >
          {{ props.bottomText }}
        </span>
      </div>
      <div :class="`${props.bgColor} p-3 md:p-2 xl:p-3 rounded-md`" v-if="icon">
        <i
          :class="`
            ${ icon }
            w-auto
            h-7
            w-7
            md:h-4
            md:w-4
            xl:h-7
            xl:w-7
            object-cover
            text-white
          `"
        />
      </div>
    </div>
    <div
      v-if="showButton"
      class="
        absolute
        left-0
        top-0
        backdrop-blur-sm
        bg-white/20
        w-full
        h-full
        flex
        justify-center
        items-center
      "
    >
      <el-button
        v-if="props.buttonText"
        :type="props.buttonType"
        size="large"
        round
        @click="buttonClick()"
      >
        {{ props.buttonText }}
      </el-button>
    </div>
  </Box>
</template>

<script setup lang="ts">
import Box from '@/components/Box/Box.vue';

const emits = defineEmits([
  'buttonClick',
]);

const props = defineProps({
  topText: String,
  featuredText: String,
  bottomText: String,
  icon: String,
  textColor: {
    type: String,
    default: 'text-blue-500',
  },
  bgColor: {
    type: String,
    default: 'bg-blue-500',
  },
  showButton: Boolean,
  buttonType: {
    type: String,
    default: 'success',
  },
  buttonText: String,
});

function buttonClick() {
  emits('buttonClick');
}
</script>
