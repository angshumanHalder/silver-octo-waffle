import { genSignature, genVoter, genCandidate, genBallot, genVoterFromSecret } from "ring-sig";

export const generateCandidate = () => {
  return genCandidate() as CandidateKey;
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

export const generateKeysFromSecret = (private_key: Array<number>) => {
  return genVoterFromSecret(private_key) as Voter;
};
