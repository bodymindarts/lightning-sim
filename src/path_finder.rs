use crate::graph::*;

type Sats = u64;

pub struct PathFinder<'a> {
    pub graph: &'a SimulatedGraph,
}

impl<'a> PathFinder<'a> {
    pub fn new(graph: &'a SimulatedGraph) -> Self {
        PathFinder { graph }
    }

    pub fn first_try_success_probability(&self, start: NodeId, end: NodeId, sats: Sats) -> f64 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_try_success_probability() {
        let node_one = Node::new_random();
        let node_two = Node::new_random();
        let node_three = Node::new_random();

        let (node_one_id, node_two_id, node_three_id) = (node_one.id, node_two.id, node_three.id);
        let graph = BaseGraph::new(vec![node_one, node_two, node_three]).into();
        let path_finder = PathFinder::new(&graph);

        let prob = path_finder.first_try_success_probability(node_one_id, node_two_id, 1);

        assert_eq!(prob, 0.0);
    }
}
