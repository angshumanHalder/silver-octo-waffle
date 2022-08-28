use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen};
use poll::{Ballot, Candidate, Poll, Signature};

mod poll;
pub(crate) mod prelude;
pub mod traits;
extern crate curve25519_dalek;
extern crate digest;
extern crate rand_core;

// Define the contract structure
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
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

    pub fn start_poll(&mut self, poll_id: usize) -> bool {
        let poll = &mut self.polls[poll_id];
        poll.change_poll_status(poll::PollStatus::STARTED);
        true
    }

    pub fn end_poll(&mut self, poll_id: usize) -> bool {
        if poll_id >= self.polls.len() {
            return false;
        }
        let poll = &mut self.polls[poll_id];
        poll.change_poll_status(poll::PollStatus::ENDED)
    }

    pub fn get_poll_status(&mut self, poll_id: usize) -> i32 {
        self.polls[poll_id].poll_status as i32
    }

    pub fn add_voter(&mut self, poll_id: usize, voter: [u8; 32]) -> bool {
        if poll_id >= self.polls.len() {
            return false;
        }
        let poll = &mut self.polls[poll_id];
        poll.add_voter(voter)
    }

    pub fn vote(&mut self, poll_id: usize, ballot: Ballot, signature: Signature) -> bool {
        if poll_id >= self.polls.len() {
            return false;
        }
        let poll = &mut self.polls[poll_id];
        poll.vote(ballot, signature)
    }

    pub fn tally(&mut self, poll_id: usize) -> HashMap<String, u32> {
        if poll_id >= self.polls.len() {
            return HashMap::new();
        }
        let _ = &self.polls[poll_id].tally();
        return self.polls[poll_id].results.clone();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use poll::Candidate;

    fn setup_poll() -> Contract {
        let mut contract = Contract::default();
        let canidate = Candidate {
            name: String::from("A"),
            party_name: String::from("A"),
            public_key: [
                5, 105, 125, 206, 75, 181, 93, 149, 139, 225, 206, 199, 11, 232, 234, 221, 176,
                208, 131, 203, 59, 93, 78, 206, 65, 26, 33, 102, 126, 189, 108, 238,
            ],
        };
        let v = vec![canidate];
        contract.create_poll(
            v,
            [
                29, 228, 250, 21, 48, 20, 21, 174, 71, 185, 41, 67, 36, 66, 13, 227, 172, 217, 166,
                72, 9, 81, 210, 76, 73, 177, 173, 248, 230, 207, 85, 1,
            ],
            [
                251, 67, 133, 123, 3, 17, 219, 92, 202, 96, 67, 93, 122, 136, 57, 73, 81, 122, 175,
                101, 190, 45, 235, 124, 192, 92, 169, 50, 34, 124, 4, 7,
            ],
        );
        contract.add_voter(
            0,
            [
                12, 122, 120, 115, 135, 140, 187, 37, 55, 16, 115, 178, 130, 243, 52, 78, 94, 151,
                98, 58, 209, 215, 200, 70, 199, 58, 110, 222, 235, 88, 78, 115,
            ],
        );
        let ballot: Ballot = Ballot {
            r: [
                208, 68, 145, 162, 42, 114, 107, 105, 169, 105, 176, 82, 152, 32, 55, 236, 205, 30,
                216, 199, 59, 155, 95, 42, 1, 45, 211, 15, 112, 96, 84, 5,
            ],
            sa: [
                168, 104, 2, 116, 77, 249, 239, 168, 98, 79, 80, 198, 217, 76, 24, 14, 99, 159,
                143, 177, 129, 110, 212, 138, 16, 231, 44, 202, 176, 47, 79, 162,
            ],
        };
        let signature = Signature {
            challenge: [
                174, 147, 150, 29, 24, 198, 13, 85, 18, 152, 213, 18, 67, 110, 226, 137, 14, 110,
                98, 29, 2, 162, 28, 52, 205, 222, 67, 141, 155, 198, 35, 5,
            ],
            responses: [
                [
                    189, 1, 240, 231, 142, 31, 172, 185, 5, 86, 7, 184, 93, 223, 10, 65, 52, 121,
                    2, 221, 197, 212, 123, 125, 194, 84, 151, 192, 220, 87, 22, 9,
                ],
                [
                    117, 99, 183, 55, 245, 212, 254, 151, 115, 153, 166, 226, 99, 73, 56, 45, 234,
                    80, 157, 190, 77, 235, 140, 167, 226, 13, 139, 176, 35, 59, 109, 11,
                ],
                [
                    245, 70, 57, 72, 70, 247, 238, 79, 87, 234, 74, 158, 177, 147, 68, 80, 216, 14,
                    47, 91, 116, 255, 7, 73, 156, 226, 116, 21, 79, 20, 199, 2,
                ],
                [
                    58, 238, 89, 180, 144, 7, 107, 232, 80, 106, 71, 33, 185, 225, 65, 244, 128,
                    155, 185, 151, 185, 218, 63, 136, 246, 157, 25, 221, 93, 55, 245, 0,
                ],
            ]
            .to_vec(),
            ring: [
                [
                    82, 45, 36, 53, 57, 140, 116, 29, 221, 240, 37, 209, 168, 56, 105, 110, 13, 73,
                    227, 218, 148, 245, 205, 111, 94, 72, 136, 175, 75, 190, 43, 42,
                ],
                [
                    178, 122, 122, 13, 180, 219, 25, 242, 30, 141, 41, 129, 202, 133, 7, 65, 71,
                    116, 204, 246, 183, 86, 19, 202, 70, 27, 156, 22, 120, 140, 182, 52,
                ],
                [
                    132, 118, 227, 237, 81, 109, 19, 204, 48, 28, 118, 94, 156, 200, 63, 28, 141,
                    101, 31, 149, 128, 23, 201, 211, 79, 138, 98, 173, 84, 176, 171, 110,
                ],
                [
                    240, 212, 39, 226, 164, 114, 89, 14, 94, 195, 137, 122, 75, 104, 49, 135, 136,
                    187, 35, 50, 239, 227, 127, 98, 110, 107, 56, 197, 22, 150, 77, 87,
                ],
            ]
            .to_vec(),
            key_image: [
                12, 122, 120, 115, 135, 140, 187, 37, 55, 16, 115, 178, 130, 243, 52, 78, 94, 151,
                98, 58, 209, 215, 200, 70, 199, 58, 110, 222, 235, 88, 78, 115,
            ],
        };
        contract.vote(0, ballot, signature);
        contract
    }

    #[test]
    fn create_poll() {
        let contract = setup_poll();
        let poll_id = contract.polls[0 as usize].poll_id;
        assert_eq!(poll_id, 0);
    }

    #[test]
    fn start_poll() {
        let mut contract = setup_poll();
        contract.start_poll(0);

        let poll_status = contract.get_poll_status(0);
        assert_eq!(poll_status, 1);
    }

    #[test]
    fn end_poll() {
        let mut contract = setup_poll();
        contract.end_poll(0);

        let poll_status = contract.get_poll_status(0);
        assert_eq!(poll_status, 2);
    }

    #[test]
    fn tally() {
        let mut contract = setup_poll();
        let result = contract.tally(0);
        println!("{:?}", result);
    }
}
