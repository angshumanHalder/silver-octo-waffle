use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, serde::Deserialize, serde::Serialize};
use near_sdk::{require, Timestamp};
use poll::{Ballot, Candidate, Poll, PollStatus, Signature};

mod poll;
pub(crate) mod prelude;
pub mod traits;
extern crate curve25519_dalek;
extern crate digest;
extern crate rand_core;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SerdePoll {
    pub poll_id: u32,
    pub created_at: Timestamp,
    pub poll_owner: String,
    pub poll_status: PollStatus,
    pub candidates: Vec<Candidate>,
    pub results: HashMap<String, u32>,
    pub shared_pk: [u8; 32],
    pub voters: usize,
}

// Define the contract structure
#[derive(BorshDeserialize, BorshSerialize)]
#[near_bindgen]
pub struct Contract {
    polls: Vec<Poll>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            polls: Vec::<Poll>::new(),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn create_poll(
        &mut self,
        candidates: Vec<Candidate>,
        shared_sk: [u8; 32],
        shared_pk: [u8; 32],
    ) -> u32 {
        let account_id = env::signer_account_id();
        let poll_id = self.polls.len() as u32;
        self.polls.push(Poll::new(
            poll_id, account_id, candidates, shared_sk, shared_pk,
        ));
        poll_id
    }

    pub fn get_polls(&self) -> Vec<SerdePoll> {
        self.polls
            .iter()
            .map(|poll| SerdePoll {
                poll_id: poll.poll_id,
                created_at: poll.created_at,
                poll_owner: poll.poll_owner.to_string().clone(),
                poll_status: poll.poll_status,
                candidates: poll.candidates.clone(),
                results: poll.results.clone(),
                voters: poll.voters_v.len(),
                shared_pk: poll.shared_pk,
            })
            .collect()
    }

    pub fn get_poll(&self, poll_id: usize) -> SerdePoll {
        let poll = &self.polls[poll_id];
        SerdePoll {
            poll_id: poll.poll_id,
            created_at: poll.created_at,
            poll_owner: poll.poll_owner.to_string().clone(),
            poll_status: poll.poll_status,
            candidates: poll.candidates.clone(),
            results: poll.results.clone(),
            voters: poll.voters_v.len(),
            shared_pk: poll.shared_pk,
        }
    }

    pub fn get_ring(&self, poll_id: usize, rng1: usize, rng2: usize, rng3: usize) -> Vec<[u8; 32]> {
        let mut ring = Vec::<[u8; 32]>::new();
        ring.push(self.polls[poll_id].get_voter(rng1));
        ring.push(self.polls[poll_id].get_voter(rng2));
        ring.push(self.polls[poll_id].get_voter(rng3));
        ring
    }

    pub fn start_poll(&mut self, poll_id: usize) {
        let poll = &mut self.polls[poll_id];
        require!(env::signer_account_id() == poll.poll_owner);
        poll.change_poll_status(poll::PollStatus::STARTED);
    }

    pub fn end_poll(&mut self, poll_id: usize) {
        let poll = &mut self.polls[poll_id];
        require!(env::signer_account_id() == poll.poll_owner);
        poll.change_poll_status(poll::PollStatus::ENDED);
    }

    pub fn get_poll_status(&mut self, poll_id: usize) -> PollStatus {
        self.polls[poll_id].poll_status
    }

    pub fn add_voter(&mut self, poll_id: usize, voter: [u8; 32]) {
        let poll = &mut self.polls[poll_id];
        poll.add_voter(voter);
    }

    pub fn vote(&mut self, poll_id: usize, ballot: Ballot, signature: Signature) -> bool {
        let poll = &mut self.polls[poll_id];
        require!(poll.poll_status == PollStatus::STARTED);
        require!(!poll.seen_key_images.contains(&signature.key_image));
        poll.vote(ballot, signature)
    }

    pub fn tally(&mut self, poll_id: usize) -> HashMap<String, u32> {
        let poll = &mut self.polls[poll_id];
        require!(poll.poll_status == PollStatus::ENDED);
        poll.tally();
        return self.polls[poll_id].results.clone();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use near_sdk::{test_utils::VMContextBuilder, testing_env};
    use poll::Candidate;

    fn setup_poll() -> Contract {
        let mut contract = Contract::default();
        let c1 = Candidate {
            name: String::from("A"),
            party_name: String::from("A"),
            public_key: [
                44, 223, 138, 82, 165, 89, 220, 147, 72, 138, 213, 115, 133, 190, 143, 165, 31,
                146, 16, 217, 141, 141, 17, 83, 10, 45, 89, 235, 99, 76, 87, 110,
            ],
        };
        let c2 = Candidate {
            name: String::from("B"),
            party_name: String::from("B"),
            public_key: [
                252, 145, 39, 44, 85, 31, 147, 14, 76, 213, 1, 160, 52, 26, 144, 49, 253, 198, 31,
                117, 210, 8, 18, 240, 225, 110, 134, 137, 236, 105, 163, 75,
            ],
        };
        let v = vec![c1, c2];
        contract.create_poll(
            v,
            [
                39, 101, 89, 174, 61, 4, 207, 68, 42, 196, 206, 86, 9, 39, 145, 21, 65, 61, 213,
                176, 155, 254, 72, 143, 49, 249, 97, 51, 82, 247, 105, 10,
            ],
            [
                63, 127, 167, 182, 151, 241, 153, 188, 107, 136, 100, 200, 122, 234, 239, 63, 68,
                102, 177, 27, 102, 93, 185, 196, 101, 35, 134, 207, 96, 189, 63, 32,
            ],
        );
        contract.add_voter(
            0,
            [
                38, 4, 214, 231, 88, 0, 199, 46, 80, 31, 15, 151, 96, 178, 203, 151, 169, 247, 253,
                42, 182, 115, 125, 201, 182, 242, 212, 184, 225, 223, 17, 5,
            ],
        );
        contract.add_voter(
            0,
            [
                58, 244, 251, 44, 106, 220, 76, 215, 228, 58, 97, 165, 216, 152, 134, 176, 1, 23,
                192, 160, 2, 52, 238, 99, 137, 152, 9, 233, 134, 142, 3, 74,
            ],
        );
        contract.add_voter(
            0,
            [
                84, 77, 232, 38, 146, 174, 87, 90, 170, 231, 182, 124, 214, 248, 40, 246, 239, 255,
                64, 193, 116, 100, 39, 174, 218, 251, 252, 83, 190, 188, 238, 57,
            ],
        );
        contract.add_voter(
            0,
            [
                24, 79, 53, 83, 8, 35, 178, 17, 217, 141, 121, 103, 84, 196, 40, 192, 101, 137,
                228, 26, 15, 232, 91, 155, 219, 15, 183, 227, 124, 53, 204, 26,
            ],
        );
        contract.add_voter(
            0,
            [
                176, 238, 168, 150, 107, 200, 253, 8, 126, 253, 189, 89, 0, 148, 33, 98, 10, 23,
                229, 43, 135, 122, 130, 123, 147, 192, 82, 64, 87, 241, 211, 52,
            ],
        );
        contract.add_voter(
            0,
            [
                134, 183, 135, 242, 201, 15, 158, 245, 135, 37, 97, 160, 40, 130, 207, 231, 141,
                207, 180, 205, 201, 124, 200, 212, 30, 234, 191, 110, 74, 68, 197, 32,
            ],
        );
        contract.add_voter(
            0,
            [
                192, 194, 230, 82, 55, 15, 47, 5, 222, 168, 100, 111, 69, 26, 32, 103, 225, 234,
                216, 139, 147, 21, 188, 169, 164, 50, 107, 82, 84, 185, 70, 18,
            ],
        );
        contract.add_voter(
            0,
            [
                74, 127, 195, 178, 202, 234, 160, 106, 217, 121, 31, 35, 134, 63, 5, 191, 64, 166,
                129, 165, 222, 87, 93, 171, 236, 67, 214, 242, 165, 104, 168, 102,
            ],
        );
        contract.add_voter(
            0,
            [
                48, 22, 16, 65, 231, 252, 121, 157, 204, 229, 90, 197, 182, 224, 251, 247, 138, 43,
                115, 123, 189, 210, 84, 136, 189, 219, 119, 249, 249, 179, 97, 122,
            ],
        );
        contract.add_voter(
            0,
            [
                22, 74, 216, 49, 239, 109, 20, 123, 205, 15, 170, 10, 154, 107, 124, 229, 249, 185,
                16, 116, 244, 54, 133, 203, 39, 222, 231, 67, 138, 237, 10, 52,
            ],
        );
        contract
    }

    #[test]
    fn create_poll() {
        let contract = setup_poll();
        let context = VMContextBuilder::new()
            .signer_account_id("test_acc".parse().unwrap())
            .build();
        testing_env!(context);
        let poll_id = contract.polls[0 as usize].poll_id;
        assert_eq!(poll_id, 0);
    }

    #[test]
    fn start_poll() {
        let mut contract = setup_poll();
        contract.start_poll(0);

        let poll_status = contract.get_poll_status(0);
        assert_eq!(poll_status, PollStatus::STARTED);
    }

    #[test]
    fn end_poll() {
        let mut contract = setup_poll();
        contract.end_poll(0);

        let poll_status = contract.get_poll_status(0);
        assert_eq!(poll_status, PollStatus::ENDED);
    }

    #[test]
    fn vote() {
        let mut contract = setup_poll();
        contract.start_poll(0);

        let ballot: Ballot = Ballot {
            r: [
                242, 51, 92, 104, 45, 4, 223, 197, 3, 38, 61, 19, 30, 83, 120, 40, 252, 140, 15,
                233, 149, 164, 173, 95, 103, 210, 120, 49, 137, 187, 70, 127,
            ],
            sa: [
                147, 101, 117, 118, 250, 191, 213, 175, 122, 237, 244, 136, 160, 234, 177, 201, 47,
                208, 10, 95, 52, 93, 50, 213, 248, 235, 95, 172, 76, 169, 65, 90,
            ],
        };
        let signature = Signature {
            challenge: [
                78, 174, 115, 190, 189, 99, 154, 96, 220, 34, 213, 98, 227, 120, 183, 171, 214,
                152, 99, 163, 158, 227, 126, 79, 170, 245, 30, 41, 244, 175, 95, 8,
            ],
            responses: [
                [
                    54, 95, 167, 160, 94, 225, 41, 180, 56, 98, 244, 161, 191, 49, 49, 46, 208,
                    245, 40, 57, 112, 24, 91, 84, 230, 35, 179, 241, 51, 20, 80, 4,
                ],
                [
                    29, 136, 134, 19, 172, 24, 221, 125, 145, 233, 57, 112, 75, 164, 252, 47, 255,
                    46, 144, 243, 188, 160, 50, 184, 199, 75, 12, 22, 80, 135, 245, 0,
                ],
                [
                    54, 95, 167, 160, 94, 225, 41, 180, 56, 98, 244, 161, 191, 49, 49, 46, 208,
                    245, 40, 57, 112, 24, 91, 84, 230, 35, 179, 241, 51, 20, 80, 4,
                ],
                [
                    54, 95, 167, 160, 94, 225, 41, 180, 56, 98, 244, 161, 191, 49, 49, 46, 208,
                    245, 40, 57, 112, 24, 91, 84, 230, 35, 179, 241, 51, 20, 80, 4,
                ],
            ]
            .to_vec(),
            ring: [
                [
                    134, 183, 135, 242, 201, 15, 158, 245, 135, 37, 97, 160, 40, 130, 207, 231,
                    141, 207, 180, 205, 201, 124, 200, 212, 30, 234, 191, 110, 74, 68, 197, 32,
                ],
                [
                    38, 4, 214, 231, 88, 0, 199, 46, 80, 31, 15, 151, 96, 178, 203, 151, 169, 247,
                    253, 42, 182, 115, 125, 201, 182, 242, 212, 184, 225, 223, 17, 5,
                ],
                [
                    58, 244, 251, 44, 106, 220, 76, 215, 228, 58, 97, 165, 216, 152, 134, 176, 1,
                    23, 192, 160, 2, 52, 238, 99, 137, 152, 9, 233, 134, 142, 3, 74,
                ],
                [
                    38, 4, 214, 231, 88, 0, 199, 46, 80, 31, 15, 151, 96, 178, 203, 151, 169, 247,
                    253, 42, 182, 115, 125, 201, 182, 242, 212, 184, 225, 223, 17, 5,
                ],
            ]
            .to_vec(),
            key_image: [
                176, 232, 175, 1, 134, 33, 76, 46, 138, 249, 154, 193, 131, 62, 10, 97, 39, 47,
                235, 220, 25, 48, 58, 39, 114, 225, 244, 211, 222, 30, 128, 69,
            ],
        };
        let voted = contract.vote(0, ballot, signature);
        assert!(voted);
    }

    #[test]
    fn tally() {
        let mut contract = setup_poll();
        contract.start_poll(0);

        let ballot: Ballot = Ballot {
            r: [
                242, 51, 92, 104, 45, 4, 223, 197, 3, 38, 61, 19, 30, 83, 120, 40, 252, 140, 15,
                233, 149, 164, 173, 95, 103, 210, 120, 49, 137, 187, 70, 127,
            ],
            sa: [
                147, 101, 117, 118, 250, 191, 213, 175, 122, 237, 244, 136, 160, 234, 177, 201, 47,
                208, 10, 95, 52, 93, 50, 213, 248, 235, 95, 172, 76, 169, 65, 90,
            ],
        };
        let signature = Signature {
            challenge: [
                78, 174, 115, 190, 189, 99, 154, 96, 220, 34, 213, 98, 227, 120, 183, 171, 214,
                152, 99, 163, 158, 227, 126, 79, 170, 245, 30, 41, 244, 175, 95, 8,
            ],
            responses: [
                [
                    54, 95, 167, 160, 94, 225, 41, 180, 56, 98, 244, 161, 191, 49, 49, 46, 208,
                    245, 40, 57, 112, 24, 91, 84, 230, 35, 179, 241, 51, 20, 80, 4,
                ],
                [
                    29, 136, 134, 19, 172, 24, 221, 125, 145, 233, 57, 112, 75, 164, 252, 47, 255,
                    46, 144, 243, 188, 160, 50, 184, 199, 75, 12, 22, 80, 135, 245, 0,
                ],
                [
                    54, 95, 167, 160, 94, 225, 41, 180, 56, 98, 244, 161, 191, 49, 49, 46, 208,
                    245, 40, 57, 112, 24, 91, 84, 230, 35, 179, 241, 51, 20, 80, 4,
                ],
                [
                    54, 95, 167, 160, 94, 225, 41, 180, 56, 98, 244, 161, 191, 49, 49, 46, 208,
                    245, 40, 57, 112, 24, 91, 84, 230, 35, 179, 241, 51, 20, 80, 4,
                ],
            ]
            .to_vec(),
            ring: [
                [
                    134, 183, 135, 242, 201, 15, 158, 245, 135, 37, 97, 160, 40, 130, 207, 231,
                    141, 207, 180, 205, 201, 124, 200, 212, 30, 234, 191, 110, 74, 68, 197, 32,
                ],
                [
                    38, 4, 214, 231, 88, 0, 199, 46, 80, 31, 15, 151, 96, 178, 203, 151, 169, 247,
                    253, 42, 182, 115, 125, 201, 182, 242, 212, 184, 225, 223, 17, 5,
                ],
                [
                    58, 244, 251, 44, 106, 220, 76, 215, 228, 58, 97, 165, 216, 152, 134, 176, 1,
                    23, 192, 160, 2, 52, 238, 99, 137, 152, 9, 233, 134, 142, 3, 74,
                ],
                [
                    38, 4, 214, 231, 88, 0, 199, 46, 80, 31, 15, 151, 96, 178, 203, 151, 169, 247,
                    253, 42, 182, 115, 125, 201, 182, 242, 212, 184, 225, 223, 17, 5,
                ],
            ]
            .to_vec(),
            key_image: [
                176, 232, 175, 1, 134, 33, 76, 46, 138, 249, 154, 193, 131, 62, 10, 97, 39, 47,
                235, 220, 25, 48, 58, 39, 114, 225, 244, 211, 222, 30, 128, 69,
            ],
        };
        contract.vote(0, ballot, signature);
        contract.end_poll(0);
        let result = contract.tally(0);
        assert!(result.contains_key("A"));
        assert_eq!(result.get("A").unwrap(), &1);
    }
}
