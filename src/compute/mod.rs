mod tensor;
pub use tensor::*;

mod graph;
pub use graph::*;

mod non_numeric;
pub use non_numeric::*;

#[async_trait::async_trait]
pub trait Compute {
    type Input;
    type Output;

    async fn compute(&self, input: Self::Input) -> crate::Result<Self::Output>;
}
