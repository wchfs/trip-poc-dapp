<template>
  <BaseContainer
    caption="Raw Rollup Communication"
  >
    <Box
      additionalClass="col-span-2"
    >
      <el-input
        v-model="input"
        :autosize="{ minRows: 2, maxRows: 4 }"
        type="textarea"
        placeholder="Please place input"
      />
    </Box>
    <Box
      additionalClass="col-span-1"
    >
      <div
        class="
          absolute
          left-0
          top-0
          w-full
          h-full
          flex
          justify-center
          items-center
        "
      >
        <el-button
          type="success"
          size="large"
          @click="chainAddInput()"
          :loading="sendingInput"
          :disabled="input.length < 1"
        >
          <span
            class="font-bold"
          >
            SEND INPUT
          </span>
        </el-button>
      </div>
    </Box>
    <Box
      additionalClass="col-span-3"
      v-if="inputs.length > 0"
    >
      <ul class="divide-y-2 divide-gray-100 overflow-x-auto w-full">
        <li class="py-3 flex justify-between text-sm text-gray-500 font-bold">
          <p class="px-4">Input</p>
          <p class="px-4">Output</p>
        </li>
        <li
          v-for="input of inputs"
          class="py-3 flex justify-between text-sm text-gray-500 font-light"
        >
          <pre class="px-4">{{ input.rawInput }}</pre>
          <el-progress
            v-if="input.rawOutput === undefined"
            style="width: 10%"
            :percentage="100"
            status="warning"
            :show-text="false"
            :indeterminate="true"
            :duration="1"
          />
          <pre
            v-else
            class="px-4"
          >{{ input.rawOutput }}</pre>
        </li>
      </ul>
    </Box>
  </BaseContainer>
</template>

<script setup lang="ts">
import BaseContainer from '@/components/Containers/BaseContainer.vue';
import Box from '@/components/Box/Box.vue';
import { ref, h } from 'vue';
import { useRollupStore } from '@/stores/rollup';
import type { OnboardComposable } from '@web3-onboard/vue/dist/types';
import { ElNotification } from 'element-plus';
import 'element-plus/es/components/notification/style/css';

const props = defineProps<{
  onboard: OnboardComposable,
}>();

interface InputTableRow {
  rawOutput?: string,
  rawInput: string,
}

const rollupStore = useRollupStore();
const inputs = ref<InputTableRow[]>([]);
const input = ref('');
const sendingInput = ref(false);

rollupStore.setup(props.onboard);

function chainAddInput() {
  sendingInput.value = true;

  const newLength = inputs.value.push({
    rawInput: input.value,
    rawOutput: undefined,
  });

  const inputToSend = input.value;
  clearInput();

  ElNotification({
    title: 'Sending Input',
    message: 'Your input is placed to send',
    type: 'info',
    position: 'bottom-left',
  });

  rollupStore
    .addInput(inputToSend)
    .then((inputResult) => {
      ElNotification({
        title: 'Waiting for output',
        message: 'Your input is processing by rollup',
        type: 'info',
        position: 'bottom-left',
      });

      inputResult.response.then((value) => {
        ElNotification({
          title: 'Done',
          message: h('p', {
            class: 'text-left',
          }, 'Your input was successfully processed - watch for output in proper table row'),
          type: 'success',
          position: 'bottom-left',
        });

        inputs.value[newLength - 1].rawOutput = value;
      });
    });
}

function clearInput() {
  sendingInput.value = false;
  input.value = '';
}
</script>
