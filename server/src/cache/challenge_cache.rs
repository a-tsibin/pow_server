use common::{
    challenge::Challenge,
    proto::ClientId,
};

pub trait ChallengeCache: Clone + Send + Sync {
    fn get(&self, client_id: &ClientId) -> Option<Challenge>;
    fn set(&self, client_id: ClientId, challenge: Challenge);
}
