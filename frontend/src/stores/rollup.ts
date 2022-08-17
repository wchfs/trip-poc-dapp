import { defineStore, getActivePinia } from 'pinia';
import { ContractReceipt, ContractTransaction, ethers } from 'ethers';
import {
    InputFacet,
    InputFacet__factory,
    OutputFacet,
    OutputFacet__factory,
    RollupsFacet,
    RollupsFacet__factory
} from '@/generated/rollups';
import { useWalletStore } from '@/stores/wallet';
import { NoticeKeys } from '@/generated/graphql';
import { InputAddedEvent } from '@/generated/rollups/contracts/interfaces/IInput';
import gql from 'graphql-tag';
import { inject } from 'vue';
import { DefaultApolloClient } from '@vue/apollo-composable';
import { ApolloClient, ApolloQueryResult } from 'apollo-client';


export interface RollupsContracts {
    rollupsContract: RollupsFacet;
    inputContract: InputFacet;
    outputContract: OutputFacet;
}

export interface RollupState {
    rollupsAddress: Record<string, any>,
    contracts: RollupsContracts | null,
}

export interface ContractTransactionResponse {
    transaction: ContractTransaction,
    receipt: ContractReceipt,
    response: Promise<any>,
}

export const useRollupStore = defineStore('rollup', {
    state: (): RollupState => {
        return {
            rollupsAddress: {
                "0x7a69": process.env.VUE_APP_DAPP_ADDRESS, // local hardhat
            },
            contracts: null,
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
        addInput: async function (input: string): Promise<ContractTransactionResponse> {
            if (this.contracts === null) {
                throw new Error('Please run rollups setup first');
            }

            const transaction = await this.contracts.inputContract.addInput(
                ethers.utils.toUtf8Bytes(input)
            );

            const receipt = await transaction.wait(1);

            return new Promise<ContractTransactionResponse>((resolve) => {
                resolve({
                    transaction,
                    receipt,
                    response: new Promise<any>(async (resolve) => {
                        const keys = this.findNoticeKeys(receipt);

                        let result: ApolloQueryResult<any> | null = null;

                        const intervalId = setInterval(async () => {
                            result = await this.apolloClient.query({
                                fetchPolicy: 'no-cache',
                                query: gql`
                                    query getNoticeByInputIndex(
                                        $inputIndex: String,
                                        $epochIndex: String,
                                    ) {
                                        GetNotice(query: {
                                            input_index: $inputIndex,
                                            epoch_index: $epochIndex,
                                        }) {
                                            payload,
                                            session_id,
                                            notice_index,
                                        }
                                    }
                                `,
                                variables: {
                                    inputIndex: keys.input_index,
                                    epochIndex: keys.epoch_index,
                                },
                            });

                            if (result?.data?.GetNotice?.length > 0) {
                                clearInterval(intervalId);
                                resolve(
                                    ethers.utils.toUtf8String(`0x${result.data.GetNotice[0].payload}`)
                                );
                            }
                        }, 1000);
                    }),
                });
            });
        },
        findNoticeKeys: function (receipt: ContractReceipt): NoticeKeys {
            const event = receipt.events?.find((e) => e.event === "InputAdded");

            if (!event) {
                throw new Error(
                    `InputAdded event not found in receipt of transaction ${receipt.transactionHash}`
                );
            }

            const inputAdded = event as InputAddedEvent;

            return {
                epoch_index: inputAdded.args.epochNumber.toString(),
                input_index: inputAdded.args.inputIndex.toString(),
            };
        },
    },
});
