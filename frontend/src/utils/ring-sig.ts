import { genSignature, genVoter, genCandidate, genBallot } from "ring-sig";

type Candidate = {
  secret: Array<number>;
  public: Array<number>;
};

type Voter = {
  secret: Array<number>;
  public: Array<number>;
  image: Array<number>;
};

type Signature = {
  challenge: Array<number>;
  responses: Array<Array<number>>;
  ring: Array<Array<number>>;
  key_image: Array<number>;
};

type Ballot = {
  sa: Array<number>;
  r: Array<number>;
};

export const generateCandidate = () => {
  return genCandidate() as Candidate;
};

export const generateVoter = () => {
  return genVoter() as Voter;
};

export const generateBallot = (shared_public: Array<number>, candidate_public: Array<number>) => {
  return genBallot(shared_public, candidate_public) as Ballot;
};

export const generateSignature = (
  private_key: Array<number>,
  public_key: Array<number>,
  key_image: Array<number>,
  ring: Array<Array<number>>,
  secret_index: number,
  ballot: Ballot
) => {
  return genSignature(private_key, public_key, key_image, ring, secret_index, ballot) as Signature;
};