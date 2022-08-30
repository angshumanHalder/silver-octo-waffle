import { FinalExecutionOutcome } from "near-api-js/lib/providers";
import { call, view } from "../near-api";

export type SharedSK_PK = Array<number>;

export async function createPoll(args: { candidates: Array<Candidate>; shared_sk: SharedSK_PK; shared_pk: SharedSK_PK }): Promise<FinalExecutionOutcome> {
  return call("create_poll", args);
}

export async function getPolls(): Promise<Poll[]> {
  return view("get_polls", {}) as Promise<Poll[]>;
}

export async function startPoll(args: { poll_id: number }): Promise<FinalExecutionOutcome> {
  return call("start_poll", args);
}

export async function endPoll(args: { poll_id: number }): Promise<FinalExecutionOutcome> {
  return call("end_poll", args);
}

export async function addVoter(args: { poll_id: number; voter: Array<number> }): Promise<FinalExecutionOutcome> {
  return call("add_voter", args);
}
