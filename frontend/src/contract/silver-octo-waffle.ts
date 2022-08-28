import { call } from "../near-api";

export interface Candidate {
  name: String;
  party_name: String;
  public_key: Array<number>;
}

export type SharedSK_PK = Array<number>;

export async function createPoll(args: { candidates: Array<Candidate>; shared_sk: SharedSK_PK; shared_pk: SharedSK_PK }) {
  return call("create_poll", args);
}
