import { defineStore } from 'pinia';
import injectedModule from '@web3-onboard/injected-wallets';
import { init, useOnboard } from '@web3-onboard/vue';
import { OnboardComposable } from '@web3-onboard/vue/dist/types';

export const useWalletStore = defineStore('wallet', {
    state: () => ({
        // ...
    }),
    getters: {
        onboard(): OnboardComposable {
            return useOnboard();
        },
    },
    actions: {
        setup() {
            const injected = injectedModule();

            init({
                wallets: [injected],
                chains: [
                    {
                        id: "0x7a69",
                        token: "ETH",
                        label: "localhost",
                        rpcUrl: "http://localhost:8545",
                    },
                    // currently only on localhost
                ],
                appMetadata: {
                    name: "Cartesi Rollups Trip Poc DApp",
                    icon: "<svg><svg/>",
                    description: "Demo app for Cartesi Rollups",
                    recommendedInjectedWallets: [
                        {
                            name: "MetaMask",
                            url: "https://metamask.io",
                        },
                        {
                            name: "Coinbase",
                            url: "https://wallet.coinbase.com/",
                        },
                    ],
                },
            });
        },
        async connect(): Promise<void> {
            if (this.onboard === null) {
                throw new Error('Run setup() first');
            }

            return this.onboard.connectWallet();
        },
    },
});
