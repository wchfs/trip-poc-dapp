import { defineStore } from 'pinia';
import { ContractTransaction, ethers } from 'ethers';
import {
    InputFacet,
    InputFacet__factory,
    OutputFacet,
    OutputFacet__factory,
    RollupsFacet,
    RollupsFacet__factory
} from '@/generated/rollups';
import { useWalletStore } from '@/stores/wallet';

export interface RollupsContracts {
    rollupsContract: RollupsFacet;
    inputContract: InputFacet;
    outputContract: OutputFacet;
}

export interface RollupState {
    rollupsAddress: Record<string, any>,
    contracts: RollupsContracts | null,
    transactions: ContractTransaction[],
}

export const useRollupStore = defineStore('rollup', {
    state: (): RollupState => {
        return {
            rollupsAddress: {
                "0x539": process.env.VUE_APP_DAPP_ADDRESS, // local hardhat
            },
            contracts: null,
            transactions: [],
        }
    },
    getters: {
        walletStore() {
            return useWalletStore();
        },
    },
    actions: {
        setup() {
            const connectedWallet = this.walletStore.onboard.connectedWallet.value;

            if (connectedWallet === null) {
                throw new Error('Please connect any wallet');
            }

            const provider = new ethers.providers.Web3Provider(
                connectedWallet.provider
            );

            const connectedChain = this.walletStore.onboard.connectedChain.value;

            if (connectedChain === null) {
                throw new Error('Please connect any chain');
            }

            const address = this.rollupsAddress[connectedChain.id];

            // rollups contract
            const rollupsContract = RollupsFacet__factory.connect(
                address,
                provider.getSigner()
            );

            // input contract
            const inputContract = InputFacet__factory.connect(
                address,
                provider.getSigner()
            );

            // output contract
            const outputContract = OutputFacet__factory.connect(
                address,
                provider.getSigner()
            );

            this.contracts = {
                rollupsContract,
                inputContract,
                outputContract,
            };
        },
        addInput(input: string) {
            if (this.contracts === null) {
                throw new Error('Please run rollups setup first');
            }

            this.contracts
                .inputContract
                .addInput(input)
                .then((transaction: ContractTransaction) => {
                    console.log(transaction);

                    this.transactions.push(transaction);
                });
        }
    },
});
