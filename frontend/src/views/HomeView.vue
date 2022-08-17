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
            <div class="input-group mb-3">
                <span class="input-group-text" id="input-lat">Lat</span>
                <input
                    type="number"
                    v-model="inputLat"
                    class="form-control"
                    aria-describedby="input-lat"
                    :disabled="sendingInput"
                >
            </div>
            <div class="input-group mb-3">
                <span class="input-group-text" id="input-lng">Lng</span>
                <input
                    type="number"
                    v-model="inputLng"
                    class="form-control"
                    aria-describedby="input-lng"
                    :disabled="sendingInput"
                >
            </div>
        </div>
        <div class="col">
            <div class="row">
                <button
                    type="button"
                    class="btn btn-success mb-3"
                    @click="chainAddInput()"
                    :disabled="sendingInput"
                >
                    Send Input
                </button>
            </div>

            <div class="row">
                <button
                    type="button"
                    class="btn btn-primary mb-3"
                    @click="patchCoords()"
                >
                    Patch coords
                </button>
            </div>
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
                <div class="d-flex" v-if="input.rawOutput === undefined">
                    <div class="spinner-grow" role="status">
                        <span class="visually-hidden">Loading...</span>
                    </div>
                </div>
                <pre class="m-1" v-else>{{ input.rawOutput }}</pre>
            </td>
        </tr>
    </table>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useWalletStore } from '@/stores/wallet';
import { useRollupStore } from '@/stores/rollup';
import { useLocationStore } from '@/stores/location';
import Map from '@/components/Map.vue';

interface InputTableRow {
    rawOutput?: string,
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
let inputLat = ref('');
let inputLng = ref('');

function patchCoords() {
    const coordsArray = locationStore.coordsArray;

    if (!coordsArray || coordsArray.length === 0) {
        return;
    }

    inputLat.value = coordsArray[0].toString();
    inputLng.value = coordsArray[1].toString();
}

async function chainAddInput() {
    sendingInput.value = true;

    const inputString = [
        inputLng.value,
        inputLat.value,
    ].join(',');

    const inputResult = await rollupStore.addInput(inputString);

    const newLength = inputs.value.push({
        rawInput: inputString,
        rawOutput: undefined,
    });

    clearInput();

    inputs.value[newLength - 1].rawOutput = await inputResult.response;
}

function clearInput() {
    sendingInput.value = false;
    inputLat.value = '';
    inputLng.value = '';
}

</script>
