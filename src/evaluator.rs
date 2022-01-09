use super::graph::*;
use super::path_finder::*;

pub struct ExhaustivePaymentEvaluator<'a> {
    graph: &'a SimulatedGraph,
    path_finder: &'a PathFinder<'a>,
}

impl<'a> ExhaustivePaymentEvaluator<'a> {
    pub fn new(path_finder: &'a PathFinder<'a>) -> Self {
        ExhaustivePaymentEvaluator {
            graph: &path_finder.graph,
            path_finder,
        }
    }

    pub fn determin_node_value(&self, node_id: NodeId) -> f64 {
        for node in self.graph.nodes() {
            if node.id == node_id {
                continue;
            }
        }
        return 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::*;

    #[test]
    fn test_determin_node_value() {
        let node_one = Node::new_random();
        let node_two = Node::new_random();
        let node_three = Node::new_random();

        let (node_one_id, node_two_id, node_three_id) = (node_one.id, node_two.id, node_three.id);
        let graph = BaseGraph::new(vec![node_one, node_two, node_three]).into();
        let path_finder = PathFinder::new(&graph);
        let evaluator = ExhaustivePaymentEvaluator::new(&path_finder);

        assert_eq!(evaluator.determin_node_value(node_one_id), 0.0);
    }
}
