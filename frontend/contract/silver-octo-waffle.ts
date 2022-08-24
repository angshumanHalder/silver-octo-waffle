import { call } from "../near-api";

export interface Candidate {
  name: String;
  party_name: String;
  public_key: Array<number>;
}

export type SharedSK_PK = Array<number>;

export async function createPoll(args: { candidates: Array<Candidate>; shared_secret: SharedSK_PK; shared_public: SharedSK_PK }): Promise<any> {
  return call("create_poll", args);
}
