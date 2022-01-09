use secp256k1::PublicKey;

pub type NodeId = PublicKey;
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[repr(transparent)]
pub struct EdgeId(u64);

impl EdgeId {
    pub fn new(id: u64) -> Self {
        EdgeId(id)
    }
}
