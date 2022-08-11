<template>
  <div class="row mt-4">
    <div class="col">
      <div class="alert alert-primary text-center" role="alert">
        <h4 class="alert-heading">
          Connected wallet
        </h4>
        <hr>
        <span class="badge bg-dark">
          {{ walletStore.onboard.connectedWallet.value?.label }}
        </span>
      </div>
    </div>

    <div class="col">
      <div class="alert alert-primary text-center" role="alert">
        <h4 class="alert-heading">
          Connected chain
        </h4>
        <hr>
        <span class="badge bg-dark">
          {{ walletStore.onboard.connectedChain.value?.id }}
        </span>
      </div>
    </div>

    <div class="col">
      <div class="alert alert-primary text-center" role="alert">
        <h4 class="alert-heading">
          Browser localization
        </h4>
        <hr>
        <span class="badge bg-dark">
          {{ locationStore.error ? 'Denied' : 'Allowed' }}
        </span>
      </div>
    </div>
  </div>

  <div class="row mb-3" v-if="walletConnected !== null && chainConnected !== null">
    <Map></Map>
  </div>

  <div class="row">
    <div class="col">
      <input class="form-control" type="text" v-model="input" style="width: 100%" :disabled="sendingInput">
    </div>
    <div class="col">
      <button type="button" class="btn btn-success" @click="chainAddInput()" :disabled="sendingInput">Send Input</button>
    </div>
  </div>

  <br>
  <table class="table table-bordered table-hover">
    <tr class="text-center">
      <th>Input</th>
      <th>Result</th>
    </tr>
    <tr v-for="input of inputs">
      <td>
        <pre class="m-1">{{ input.rawInput }}</pre>
      </td>
      <td>
        <div class="d-flex">
          <div class="spinner-grow" role="status">
            <span class="visually-hidden">Loading...</span>
          </div>
        </div>
      </td>
    </tr>
  </table>
</template>

<script setup lang="ts">
import { useWalletStore } from '@/stores/wallet';
import { ref } from 'vue';
import { useRollupStore } from '@/stores/rollup';
import Map from '@/components/Map.vue';
import { useLocationStore } from '@/stores/location';
import { ContractTransactionResponse } from '@/stores/rollup';

interface InputTableRow {
  transactionResponse: ContractTransactionResponse,
  rawInput: string,
}

const rollupStore = useRollupStore();
const walletStore = useWalletStore();
const locationStore = useLocationStore();

walletStore.setup();
walletStore.connect().then(() => {
  rollupStore.setup();
});

const walletConnected = walletStore.onboard.connectedWallet;
const chainConnected = walletStore.onboard.connectedChain;

const inputs = ref<InputTableRow[]>([]);

let sendingInput = ref(false);
let input = ref('');

async function chainAddInput() {
  sendingInput.value = true;
  const inputResult = await rollupStore.addInput(input.value)
  console.log(inputResult);
  inputs.value.push({
    transactionResponse: inputResult,
    rawInput: input.value,
  });
  sendingInput.value = false;
  input.value = '';
}
</script>
