import { defineStore } from 'pinia';
import type { ContractReceipt, ContractTransaction } from 'ethers';
import { ethers } from 'ethers';
import type { OnboardComposable } from '@web3-onboard/vue/dist/types';
import type { InputAddedEvent } from "@cartesi/rollups/dist/src/types/contracts/interfaces/IInput";
import type { ERC20PortalFacet, InputFacet, OutputFacet, } from "@cartesi/rollups";
import { ERC20PortalFacet__factory, InputFacet__factory, OutputFacet__factory, } from "@cartesi/rollups";

import type {
  Input,
  Notice,
  NoticesByEpochAndInputQuery,
  NoticesByEpochAndInputQueryVariables
} from '@/generated/graphql';
import { NoticesByEpochAndInputDocument } from '@/generated/graphql';
import type { GraphQLError } from 'graphql';
import { ElNotification } from 'element-plus';
import 'element-plus/es/components/notification/style/css';

export type PartialEpoch = Pick<Input, "index">;
export type PartialInput = PartialEpoch & { epoch: PartialEpoch };
export type PartialNotice = Pick<Notice, "__typename" | "id" | "index" | "payload"> & {
  input: PartialInput;
};


export interface RollupsContracts {
  inputContract: InputFacet;
  outputContract: OutputFacet;
  rollupsContract: ERC20PortalFacet;
}

export interface RollupState {
  rollupsAddress: Record<string, any>,
  contracts: RollupsContracts | null,
  communicationHistory: RollupCommunicationHistoryItem[],
}

export interface ContractTransactionResponse {
  transaction: ContractTransaction,
  receipt: ContractReceipt,
  response: Promise<any>,
}

export interface RollupCommunicationHistoryItem {
  rawOutput?: string,
  rawInput: string,
}

export const useRollupStore = defineStore('rollup', {
  state: (): RollupState => {
    return {
      rollupsAddress: {
        "0x7a69": import.meta.env.VITE_APP_DAPP_ADDRESS, // local hardhat
      },
      contracts: null,
      communicationHistory: [],
    }
  },
  getters: {
    // ...
  },
  actions: {
    setup(onboard: OnboardComposable) {
      if (this.contracts !== null) {
        return;
      }

      const connectedChain = onboard.connectedChain.value;

      if (connectedChain === null) {
        throw new Error('Please connect any chain');
      }

      const connectedWallet = onboard.connectedWallet.value;

      if (connectedWallet === null) {
        throw new Error('Please connect any wallet');
      }

      const provider = new ethers.providers.Web3Provider(
        connectedWallet.provider
      );

      const address = this.rollupsAddress[connectedChain.id];

      this.contracts = {
        inputContract: InputFacet__factory.connect(address, provider.getSigner()),
        outputContract: OutputFacet__factory.connect(address, provider.getSigner()),
        rollupsContract: ERC20PortalFacet__factory.connect(address, provider.getSigner()),
      };
    },
    addInput: async function (input: string): Promise<ContractTransactionResponse> {
      if (this.contracts === null) {
        throw new Error('Please run rollups setup first');
      }

      const transaction = await this.contracts.inputContract.addInput(
        ethers.utils.toUtf8Bytes(input)
      );

      ElNotification({
        title: 'Waiting for receipt',
        message: 'Transaction is waiting for a receipt',
        type: 'info',
        position: 'bottom-left',
        duration: 8000,
      });

      const receipt = await transaction.wait(1);

      return new Promise<ContractTransactionResponse>((resolve) => {
        resolve({
          transaction,
          receipt,
          response: new Promise<any>(async (resolve) => {
            const keys = this.getInputKeys(receipt);

            const intervalId = setInterval(async () => {
              const variables: NoticesByEpochAndInputQueryVariables = {
                input_index: keys.input_index,
                epoch_index: keys.epoch_index,
              };

              this.apolloClient.query<NoticesByEpochAndInputQuery, NoticesByEpochAndInputQueryVariables>({
                fetchPolicy: 'no-cache',
                query: NoticesByEpochAndInputDocument,
                variables,
              }).then((response) => {
                if (response === null) {
                  return;
                }

                if (response?.data?.epoch?.input?.notices) {
                  const notice = response
                    .data
                    .epoch
                    .input
                    .notices
                    .nodes
                    .filter<PartialNotice>((n: PartialNotice | null): n is PartialNotice => n !== null)[0];

                  clearInterval(intervalId);
                  resolve(ethers.utils.toUtf8String(notice.payload));
                }
              }).catch((error: GraphQLError) => {
                console.log(error.message);
              });
            }, 3000);
          }),
        });
      });
    },
    getInputKeys: function (receipt: ContractReceipt): {
      epoch_index: number;
      input_index: number;
    } {
      const event = receipt.events?.find((event) => event.event === "InputAdded");

      if (!event) {
        throw new Error(
          `InputAdded event not found in receipt of transaction ${receipt.transactionHash}`
        );
      }

      const inputAdded = event as InputAddedEvent;

      const inputKeys = {
        epoch_index: inputAdded.args.epochNumber.toNumber(),
        input_index: inputAdded.args.inputIndex.toNumber(),
      };

      console.log(inputKeys);

      return inputKeys;
    },
  },
});
