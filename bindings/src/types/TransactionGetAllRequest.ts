// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ComponentAddress } from "./ComponentAddress";
import type { TransactionStatus } from "./TransactionStatus";

export interface TransactionGetAllRequest {
  status: TransactionStatus | null;
  component: ComponentAddress | null;
}
