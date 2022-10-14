import type { ContractReceipt, ContractTransaction } from 'ethers';
import { ethers } from 'ethers';
import type { InputAddedEvent } from '@cartesi/rollups/dist/src/types/contracts/interfaces/IInput';
import type { ERC20PortalFacet, EtherPortalFacet, InputFacet, OutputFacet, } from "@cartesi/rollups";
import {
  ERC20PortalFacet__factory,
  EtherPortalFacet__factory,
  InputFacet__factory,
  OutputFacet__factory
} from '@cartesi/rollups';
import type {
  Input,
  Notice,
  NoticesByEpochAndInputQuery,
  NoticesByEpochAndInputQueryVariables
} from '@/generated/graphql';
import { NoticesByEpochAndInputDocument } from '@/generated/graphql';
import type { GraphQLError } from 'graphql';
import type { AdvanceRequest } from '@/interfaces/rollup-api';
import { ApolloService } from '@/services/apollo-service';
import { useWalletStore } from '@/stores/wallet';
import fetch from 'cross-fetch';
import type { Error as InspectError, InspectRequest, InspectResponse, Report } from '@/interfaces/rollup-api';
import { hex2str } from '@/helpers/helpers';

export interface InputKeys {
  epoch_index: number;
  input_index: number;
}

export interface RollupsContracts {
  inputContract: InputFacet;
  outputContract: OutputFacet;
  rollupsContract: ERC20PortalFacet;
  etherContract: EtherPortalFacet;
}

export interface ContractTransactionResponse<T> {
  transaction: ContractTransaction,
  receipt: ContractReceipt,
  response: Promise<T>,
}

export type PartialEpoch = Pick<Input, "index">;
export type PartialInput = PartialEpoch & { epoch: PartialEpoch };
export type PartialNotice = Pick<Notice, "__typename" | "id" | "index" | "payload"> & {
  input: PartialInput;
};

export abstract class RollupService {
  private static contracts?: RollupsContracts;

  public static setContracts(contracts: RollupsContracts) {
    this.contracts = contracts;
  }

  public static getContracts(): RollupsContracts {
    if (!this.contracts) {
      this.setup();
    }

    if (!this.contracts) {
      throw new Error('Cannot setup rollup contracts');
    }

    return this.contracts;
  }

  public static hasContracts(): boolean {
    return !!this.contracts;
  }

  public static getRollupAddress(key: string): string {
    const addresses: {
      [key: string]: string,
    } = {
      "0x7a69": import.meta.env.VITE_APP_DAPP_ADDRESS as string, // local hardhat
    };

    return addresses[key];
  }

  public static setup() {
    if (this.hasContracts()) {
      return;
    }

    const onboard = useWalletStore().onboard;

    if (onboard === null) {
      throw new Error('Run web3 onboard first');
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

    const address = this.getRollupAddress(connectedChain.id);
    const signer = provider.getSigner();

    RollupService.setContracts({
      inputContract: InputFacet__factory.connect(address, signer),
      outputContract: OutputFacet__factory.connect(address, signer),
      rollupsContract: ERC20PortalFacet__factory.connect(address, signer),
      etherContract: EtherPortalFacet__factory.connect(address, signer),
    });
  }

  public static async addInput<T>(payload: AdvanceRequest, deposit: string | null = null): Promise<ContractTransactionResponse<T>> {
    const payloadBytes = ethers.utils.isBytesLike(payload)
      ? payload
      : ethers.utils.toUtf8Bytes(JSON.stringify(payload));

    const transaction = deposit === null
      ? await RollupService.getContracts().inputContract.addInput(payloadBytes)
      : await RollupService.getContracts().etherContract.etherDeposit(
        payloadBytes, {
          value: ethers.utils.parseUnits("1000000", "gwei")
        }
      );

    const receipt = await transaction.wait(1);

    return new Promise((resolve) => {
      resolve({
        transaction,
        receipt,
        response: new Promise<T>(async (resolve) => {
          const keys = deposit === null
            ? RollupService.getInputKeysFromAdvanceReceipt(receipt)
            : RollupService.getInputKeysFromDepositReceipt(receipt);

          const intervalId = setInterval(async () => {
            const variables: NoticesByEpochAndInputQueryVariables = {
              input_index: keys.input_index,
              epoch_index: keys.epoch_index,
            };

            ApolloService.getClient().query<NoticesByEpochAndInputQuery, NoticesByEpochAndInputQueryVariables>({
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
                const decodedNoticePayload = ethers.utils.toUtf8String(notice.payload);
                resolve(JSON.parse(decodedNoticePayload) as T);
              }
            }).catch((error: GraphQLError) => {
              console.log(error.message);
            });
          }, 3000);
        }),
      });
    });
  }

  public static async inspect<T>(params: InspectRequest): Promise<T[]> {
    const url = import.meta.env.VITE_APP_INSPECT_ENDPOINT;
    const response = await fetch(`${url}/${JSON.stringify(params)}`);

    return new Promise((resolve, reject) => {
      if (response.status !== 200) {
        response.json().then((r: InspectError) => {
          reject(r);
          return;
        });
      }

      response.json().then((r: InspectResponse) => {
        const decodedReports = r.reports.map((report: Report) => {
          return JSON.parse(hex2str(report.payload)) as T;
        });

        resolve(decodedReports);
      });
    });
  }

  public static getInputKeysFromDepositReceipt(
    receipt: ContractReceipt,
  ): InputKeys {
    const errorMessage = `InputAdded event not found in receipt of transaction ${receipt.transactionHash}`;

    if (!receipt.events || receipt.events.length < 1) {
      throw new Error(errorMessage);
    }

    for (const event of receipt.events) {
      try {
        const parsedLog = this.getContracts().inputContract.interface.parseLog(event);

        if (parsedLog.name == "InputAdded") {
          return {
            epoch_index: parsedLog.args.epochNumber.toNumber(),
            input_index: parsedLog.args.inputIndex.toNumber(),
          };
        }
      } catch (e) {
        // do nothing, just skip to try parsing the next event
      }
    }

    throw new Error(errorMessage);
  }

  public static getInputKeysFromAdvanceReceipt(
    receipt: ContractReceipt,
  ): InputKeys {
    const event = receipt.events?.find((event) => event.event === "InputAdded");

    if (!event) {
      throw new Error(
        `InputAdded event not found in receipt of transaction ${receipt.transactionHash}`
      );
    }

    const inputAdded = event as InputAddedEvent;

    return {
      epoch_index: inputAdded.args.epochNumber.toNumber(),
      input_index: inputAdded.args.inputIndex.toNumber(),
    };
  }
}
