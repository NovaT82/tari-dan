// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CommitteeShard } from "./CommitteeShard";
import type { Epoch } from "./Epoch";

export interface GetEpochManagerStatsResponse {
  current_epoch: Epoch;
  current_block_height: bigint;
  is_valid: boolean;
  committee_shard: CommitteeShard | null;
}
