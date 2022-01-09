use super::primitives::*;
use rand::thread_rng;
use secp256k1::Secp256k1;

#[derive(Debug)]
pub struct Node {
    pub id: NodeId,
    edges: Vec<EdgeId>,
}

impl Node {
    pub fn new_random() -> Self {
        let (_, id) = Secp256k1::new().generate_keypair(&mut thread_rng());
        Self::new(id)
    }
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            edges: Vec::new(),
        }
    }
}
