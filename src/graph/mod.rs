mod node;
mod primitives;

pub use self::{node::*, primitives::*};
use secp256k1::PublicKey;
use std::collections::HashMap;

pub struct BaseGraph {
    nodes: HashMap<NodeId, Node>,
    edges: HashMap<EdgeId, Edge>,
}

impl BaseGraph {
    pub fn new(nodes: impl IntoIterator<Item = Node>) -> Self {
        Self {
            nodes: HashMap::from_iter(nodes.into_iter().map(|n| (n.id, n))),
            edges: HashMap::new(),
        }
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }
}

impl Into<SimulatedGraph> for BaseGraph {
    fn into(self) -> SimulatedGraph {
        SimulatedGraph::new(self)
    }
}

pub struct SimulatedGraph {
    base: BaseGraph,
}

impl SimulatedGraph {
    fn new(base: BaseGraph) -> Self {
        Self { base }
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.base.nodes()
    }
}

#[derive(Debug)]
pub struct Edge {
    pub edge_id: EdgeId,
    pub node1_pub: PublicKey,
    pub node2_pub: PublicKey,
    pub capacity: u64,
}

impl Edge {
    fn payment_sucess_rate(&self, payment_amount: u64) -> f64 {
        let n_states = (self.capacity + 1) as f64;
        let n_success = n_states - (payment_amount as f64);
        f64::max(n_success / n_states, 0.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    fn edge_with_capacity(capacity: u64) -> Edge {
        let node1_pub = PublicKey::from_str(
            "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        )
        .unwrap();
        let node2_pub = PublicKey::from_str(
            "02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        )
        .unwrap();
        Edge {
            edge_id: EdgeId::new(0),
            node1_pub,
            node2_pub,
            capacity,
        }
    }

    #[test]
    fn payment_success_rate() {
        let edge = edge_with_capacity(1);
        assert_eq!(edge.payment_sucess_rate(1), 1.0 / 2.0);
        assert_eq!(edge.payment_sucess_rate(2), 0.0);
        let edge = edge_with_capacity(2);
        assert_eq!(edge.payment_sucess_rate(1), 2.0 / 3.0);
        assert_eq!(edge.payment_sucess_rate(2), 1.0 / 3.0);
        assert_eq!(edge.payment_sucess_rate(6), 0.0);
        assert_eq!(edge.payment_sucess_rate(0), 1.0);
    }
}
