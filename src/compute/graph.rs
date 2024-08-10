use async_trait::async_trait;

use crate::GraphComputationError;

#[async_trait]
pub trait GraphComputation {
    type Node;
    type Edge;
    type Output;

    async fn add_node(&mut self, node: Self::Node) -> crate::Result<(), GraphComputationError>;

    async fn add_edge(
        &mut self,
        node1: Self::Node,
        node2: Self::Node,
    ) -> crate::Result<(), GraphComputationError>;

    async fn compute(&self) -> crate::Result<Self::Output, GraphComputationError>;
}
