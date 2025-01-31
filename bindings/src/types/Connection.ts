// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ConnectionDirection } from "./ConnectionDirection";

export interface Connection {
  connection_id: string;
  peer_id: string;
  address: string;
  direction: ConnectionDirection;
  age: string;
  ping_latency: string | null;
}
