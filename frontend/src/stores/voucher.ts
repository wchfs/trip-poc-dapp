import { RollupService } from '@/services/rollup-service';
import type { InspectResponseDecodedPayload } from '@/interfaces/rollup-api';
import { defineStore } from 'pinia';
import type { Voucher } from '@/interfaces/voucher';
import { useWalletStore } from '@/stores/wallet';

const walletStore = useWalletStore();

export const useVoucherStore = defineStore('voucher', {
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
                            }
                        }
                    }
                });
            } catch (e) {
                console.error(e);
            }
            
            if (result === null) {
                return Promise.reject(false);
            }
            console.log(result);
            result.forEach(reports => {
                if(reports === null || reports.data === null || reports.error !== null) {
                    console.error("back-end error: ", reports.error);

                    return Promise.reject(false);
                }
                
                reports.data.forEach(voucherReport => {
                    this.addVoucher(voucherReport);
                })
            })
            console.log(this.vouchers);
            return Promise.resolve(true);
        },
        addVoucher(voucher: Voucher) {
            this.vouchers.push(voucher);
        }
    }
})