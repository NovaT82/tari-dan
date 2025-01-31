// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { BlockId } from "./BlockId";
import type { Epoch } from "./Epoch";
import type { NodeHeight } from "./NodeHeight";
import type { QcId } from "./QcId";
import type { SubstateDestroyed } from "./SubstateDestroyed";
import type { SubstateId } from "./SubstateId";
import type { SubstateValue } from "./SubstateValue";
import type { TransactionId } from "./TransactionId";

export interface SubstateRecord {
  substate_id: SubstateId;
  version: number;
  substate_value: SubstateValue;
  state_hash: string;
  created_by_transaction: TransactionId;
  created_justify: QcId;
  created_block: BlockId;
  created_height: NodeHeight;
  created_at_epoch: Epoch;
  destroyed: SubstateDestroyed | null;
}
