use crate::challenge::Challenge;
use serde::{Deserialize, Serialize};

pub const SIZE: usize = 32;

#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
pub struct ChallengeMessage {
    pub difficulty: u8,
    pub hash_seq: [u8; SIZE],
}

impl<'a> From<&'a Challenge> for ChallengeMessage {
    fn from(challenge: &'a Challenge) -> Self {
        ChallengeMessage {
            difficulty: challenge.difficulty,
            hash_seq: challenge.hash_seq,
        }
    }
}

#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
pub struct SolutionMessage {
    pub solution: [u8; SIZE],
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct QuoteMessage {
    pub quote: String,
}
