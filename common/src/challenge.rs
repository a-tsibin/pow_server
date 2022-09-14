use crate::errors::CommonErrors::InvalidSolution;
use crate::proto::{ChallengeMessage, SIZE};
use anyhow::{anyhow, Result};
use log::trace;
use rand::Rng;
use sha2::{Digest, Sha256};

pub struct Challenge {
    pub difficulty: u8,
    pub hash_seq: [u8; SIZE],
    pub hash: Sha256,
}

impl Challenge {
    pub fn new(difficulty: u8) -> Challenge {
        let seq = rand::thread_rng().gen::<[u8; SIZE]>();
        let mut hash = Sha256::new();
        hash.update(seq);
        Self {
            difficulty,
            hash_seq: seq,
            hash,
        }
    }

    pub fn check_solution(&self, solution: &[u8; SIZE]) -> Result<()> {
        let mut hash = self.hash.clone();
        hash.update(solution);
        let result = hash.finalize();
        let zeros = result
            .iter()
            .try_fold(0, |acc, elem_ref| {
                if *elem_ref == 0 {
                    Ok(acc + 1)
                } else {
                    Err(acc)
                }
            })
            .unwrap_or_else(|e| e);

        if zeros >= self.difficulty {
            return Ok(());
        }
        Err(anyhow!(InvalidSolution))
    }

    pub fn solve(&self) -> [u8; SIZE] {
        trace!("Trying to solve challenge");
        let mut rng = rand::thread_rng();
        let mut tries: u128 = 0;
        loop {
            let possible_solution = rng.gen::<[u8; SIZE]>();
            tries += 1;
            if let Ok(()) = self.check_solution(&possible_solution) {
                trace!("Challenge successfully solved after {} tries", tries);
                return possible_solution;
            }
        }
    }
}

impl From<ChallengeMessage> for Challenge {
    fn from(msg: ChallengeMessage) -> Self {
        let mut hash = Sha256::new();
        hash.update(msg.hash_seq);
        Challenge {
            difficulty: msg.difficulty,
            hash_seq: msg.hash_seq,
            hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::challenge::Challenge;
    use crate::proto::SIZE;
    use sha2::{Digest, Sha256};

    #[test]
    fn test_challenge_solution() {
        let challenge = Challenge::new(2);
        let solution = challenge.solve();
        assert!(challenge.check_solution(&solution).is_ok());
        let mut hash = Sha256::default();
        hash.update(&challenge.hash_seq);
        hash.update(&solution);
        let hash_hex = format!("{:x}", hash.finalize());
        assert!(hash_hex.starts_with("00"));
    }

    #[test]
    fn test_invalid_solution() {
        let challenge = Challenge::new(2);
        assert!(challenge.check_solution(&[0u8; SIZE]).is_err());
    }
}
