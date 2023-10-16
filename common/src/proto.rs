use serde::{
    Deserialize,
    Serialize,
};

use crate::challenge::Challenge;

pub const SIZE: usize = 32;

pub type ClientId = u64;

#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
pub struct InitMessage {
    pub client_id: ClientId,
    pub solution:  Option<[u8; SIZE]>,
}

impl InitMessage {
    pub fn new(client_id: ClientId, solution: Option<[u8; SIZE]>) -> Self {
        InitMessage {
            client_id,
            solution,
        }
    }
}

#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
pub struct ChallengeMessage {
    pub difficulty: u8,
    pub hash_seq:   [u8; SIZE],
}

impl<'a> From<&'a Challenge> for ChallengeMessage {
    fn from(challenge: &'a Challenge) -> Self {
        ChallengeMessage {
            difficulty: challenge.difficulty,
            hash_seq:   challenge.hash_seq,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct QuoteMessage {
    pub quote: String,
}
