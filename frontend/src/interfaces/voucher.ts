import type { PartialVoucher } from '@/services/rollup-service';

export interface Voucher {
    id: number;
    epoch_index: number;
    input_index: number;
    voucher_index: number;
    owner_address: string;
    status?: "pending" | "approved";
    generated_voucher?: PartialVoucher
}