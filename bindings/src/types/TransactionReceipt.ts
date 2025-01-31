// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Event } from "./Event";
import type { FeeReceipt } from "./FeeReceipt";
import type { Hash } from "./Hash";
import type { LogEntry } from "./LogEntry";

export interface TransactionReceipt {
  transaction_hash: Hash;
  events: Array<Event>;
  logs: Array<LogEntry>;
  fee_receipt: FeeReceipt;
}
