import type {
  VoucherByEpochInputAndVoucherIndexQuery,
  VoucherByEpochInputAndVoucherIndexQueryVariables
} from "@/generated/graphql";
import { VoucherByEpochInputAndVoucherIndexDocument } from "@/generated/graphql";
import type { RollupResponseDecodedPayload } from "@/interfaces/rollup-api";
import type { Voucher } from "@/interfaces/voucher";
import { ApolloService } from "@/services/apollo-service";
import { RollupService } from "@/services/rollup-service";
import { useWalletStore } from "@/stores/wallet";
import type { GraphQLError } from "graphql";
import { defineStore } from "pinia";

const walletStore = useWalletStore();

export const useVoucherStore = defineStore("voucher", {
  state: () => ({
    waitingForVoucher: false as boolean,
    vouchers: [] as Voucher[],
  }),
  getters: {},
  actions: {
    clearVouchers() {
      this.vouchers = [];
    },
    async fetchVouchers(force: boolean = false): Promise<boolean> {
      if (this.vouchers.length > 0 && !force) {
        return Promise.resolve(true);
      }

      this.clearVouchers();

      let result = null as RollupResponseDecodedPayload<Voucher[]>[] | null;

      try {
        result = await RollupService.inspect<Voucher[]>({
          endpoint: "get_vouchers",
          payload: {
            Voucher: {
              Get: {
                owner_address: walletStore.walletAddress,
              },
            },
          },
        });
      } catch (e) {
        console.error(e);
      }

      if (result === null) {
        return Promise.reject(false);
      }

      result.forEach((reports) => {
        if (
          reports === null ||
          reports.data === null ||
          reports.error !== null
        ) {
          console.error("back-end error: ", reports.error);

          return Promise.reject(false);
        }

        reports.data.forEach(async (voucherReport) => {
          try {
            const voucher = await this.checkVoucher(voucherReport);

            this.addVoucher(voucher);
          } catch (e) {            
            // ...
          }
        });
      });

      return Promise.resolve(true);
    },
    addVoucher(voucher: Voucher) {
      this.vouchers.push(voucher);
    },
    async withdrawFunds(zoneId: number, amount: string): Promise<void> {
      this.waitingForVoucher = true;

      RollupService.addInput<{
        errors: string[];
      }>({
        endpoint: "withdraw_funds",
        payload: {
          Balance: {
            Withdraw: {
              amount: amount,
              zone_id: zoneId,
            },
          },
        },
      }).then(() => {
        this.fetchVouchers(true);
      }).catch((error) => {
        console.log(error);
      }).finally(() => {
        this.waitingForVoucher = false;
      });
    },
    async checkVoucher(voucher_to_check: Voucher): Promise<Voucher> {
      const variables: VoucherByEpochInputAndVoucherIndexQueryVariables = {
        input_index: voucher_to_check.input_index,
        epoch_index: voucher_to_check.epoch_index,
        voucher_index: voucher_to_check.voucher_index,
      };

      voucher_to_check.status = "pending";

      return new Promise((resolve, reject) => {
        ApolloService.getClient().query<
          VoucherByEpochInputAndVoucherIndexQuery,
          VoucherByEpochInputAndVoucherIndexQueryVariables
        >({
          fetchPolicy: "no-cache",
          query: VoucherByEpochInputAndVoucherIndexDocument,
          variables,
        }).then((response) => {
          if (response?.data?.epoch?.input?.voucher) {
            voucher_to_check.generated_voucher = response?.data?.epoch?.input?.voucher;

            if (!voucher_to_check.generated_voucher) {
              reject("There is no voucher");
            }

            if (voucher_to_check.generated_voucher.proof && voucher_to_check.generated_voucher.proof?.machineStateHash != "0x") {
              voucher_to_check.status = "approved";
            }

            resolve(voucher_to_check);
          }
        }).catch((error: GraphQLError) => {
          reject(error);
        });
      });
    },
  },
});
