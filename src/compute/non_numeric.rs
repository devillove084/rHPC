use async_trait::async_trait;

use crate::NonNumericComputationError;

#[async_trait]
pub trait NonNumericComputation {
    type Input;
    type Output;

    async fn parse(
        &self,
        input: Self::Input,
    ) -> crate::Result<Self::Output, NonNumericComputationError>;

    async fn match_pattern(
        &self,
        input: Self::Input,
        pattern: Self::Input,
    ) -> crate::Result<Self::Output, NonNumericComputationError>;

    async fn query(
        &self,
        input: Self::Input,
        query: Self::Input,
    ) -> crate::Result<Self::Output, NonNumericComputationError>;
}
