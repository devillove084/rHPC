use async_trait::async_trait;

use crate::TensorComputationError;

#[async_trait]
pub trait TensorComputation {
    type Input;
    type Output;

    async fn add(&self, other: &Self) -> crate::Result<Self::Output, TensorComputationError>;
    async fn multiply(&self, other: &Self) -> crate::Result<Self::Output, TensorComputationError>;
    async fn transpose(&self) -> crate::Result<Self::Output, TensorComputationError>;
}
