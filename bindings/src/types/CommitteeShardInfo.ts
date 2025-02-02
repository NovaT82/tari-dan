// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Shard } from "./Shard";
import type { SubstateAddress } from "./SubstateAddress";
import type { ValidatorNode } from "./ValidatorNode";

export interface CommitteeShardInfo<TAddr> {
  shard: Shard;
  substate_address_range: { start: SubstateAddress; end: SubstateAddress };
  validators: Array<ValidatorNode<TAddr>>;
}
