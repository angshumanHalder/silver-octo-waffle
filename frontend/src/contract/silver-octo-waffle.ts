import { FinalExecutionOutcome } from "near-api-js/lib/providers";
import { call, view } from "../near-api";
import { Gas } from "near-units";

export async function createPoll(args: { candidates: Array<Candidate>; shared_sk: SharedSK_PK; shared_pk: SharedSK_PK }): Promise<FinalExecutionOutcome> {
  return call("create_poll", args);
}

export async function getPolls(): Promise<Poll[]> {
  return view("get_polls", {}) as Promise<Poll[]>;
}

export async function getPoll(args: { poll_id: number }): Promise<Poll> {
  return view("get_poll", args) as Promise<Poll>;
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

export async function vote(args: { poll_id: number; ballot: Ballot; signature: Signature }): Promise<FinalExecutionOutcome> {
  return call("vote", args, { gas: Gas.parse("100000000000000") });
}

export async function tally(args: { poll_id: number }): Promise<FinalExecutionOutcome> {
  return call("tally", args, { gas: Gas.parse("300000000000000") });
}

export async function getRing(args: { poll_id: number; rng1: number; rng2: number; rng3: number }): Promise<Array<number[]>> {
  return view("get_ring", args) as Promise<Array<number[]>>;
}
