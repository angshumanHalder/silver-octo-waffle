import { FinalExecutionOutcome } from "near-api-js/lib/providers";
import { call, view } from "../near-api";

export type SharedSK_PK = Array<number>;

export async function createPoll(args: { candidates: Array<Candidate>; shared_sk: SharedSK_PK; shared_pk: SharedSK_PK }): Promise<FinalExecutionOutcome> {
  return call("create_poll", args);
}

export async function getPolls(): Promise<Poll[]> {
  return view("get_polls", {}) as Promise<Poll[]>;
}
