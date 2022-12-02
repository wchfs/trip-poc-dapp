import type {
    VoucherByEpochInputAndVoucherIndexQuery,
    VoucherByEpochInputAndVoucherIndexQueryVariables,
} from "@/generated/graphql";
import { VoucherByEpochInputAndVoucherIndexDocument } from "@/generated/graphql";
import { RollupService } from "@/services/rollup-service";
import type { InspectResponseDecodedPayload } from "@/interfaces/rollup-api";
import { defineStore } from "pinia";
import type { Voucher } from "@/interfaces/voucher";
import { useWalletStore } from "@/stores/wallet";
import { ApolloService } from "@/services/apollo-service";
import type { GraphQLError } from "graphql";

const walletStore = useWalletStore();

export const useVoucherStore = defineStore("voucher", {
    state: () => ({
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

            let result = null as InspectResponseDecodedPayload<Voucher[]>[] | null;

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

                reports.data.forEach((voucherReport) =>
                    this.checkVoucher(voucherReport).then((voucher) => {
                        this.addVoucher(voucher);
                    })
                );
            });
            
            return Promise.resolve(true);
        },
        addVoucher(voucher: Voucher) {
            this.vouchers.push(voucher);
        },
        async checkVoucher(voucher_to_check: Voucher): Promise<Voucher> {
            const variables: VoucherByEpochInputAndVoucherIndexQueryVariables = {
                input_index: voucher_to_check.input_index,
                epoch_index: voucher_to_check.epoch_index,
                voucher_index: voucher_to_check.voucher_index,
            };

            voucher_to_check.status = "pending";

            return new Promise((resolve, reject) => {
                ApolloService.getClient()
                    .query<
                        VoucherByEpochInputAndVoucherIndexQuery,
                        VoucherByEpochInputAndVoucherIndexQueryVariables
                    >({
                        fetchPolicy: "no-cache",
                        query: VoucherByEpochInputAndVoucherIndexDocument,
                        variables,
                    })
                    .then((response) => {
                        if (response?.data?.epoch?.input?.voucher) {
                            const voucher = response.data.epoch.input.voucher;

                            if (!voucher) {
                                reject(voucher_to_check);
                            }
                            
                            if (voucher.proof && voucher.proof?.machineStateHash != "0x") {
                                voucher_to_check.status = "approved";
                            }
                            
                            resolve(voucher_to_check);
                        }
                    })
                    .catch((error: GraphQLError) => {
                        console.log(error.message);
                        reject(voucher_to_check);
                    });
            });
        },
    },
});
