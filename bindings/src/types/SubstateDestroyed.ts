// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { BlockId } from "./BlockId";
import type { Epoch } from "./Epoch";
import type { QcId } from "./QcId";
import type { TransactionId } from "./TransactionId";

export interface SubstateDestroyed {
  by_transaction: TransactionId;
  justify: QcId;
  by_block: BlockId;
  at_epoch: Epoch;
}