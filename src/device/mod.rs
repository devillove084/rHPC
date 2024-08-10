mod gpu;
pub use gpu::*;

mod cpu;
pub use cpu::*;

#[async_trait::async_trait]
pub trait Device {
    async fn execute<F, R: Send>(&self, computation: F) -> crate::Result<R>
    where F: FnOnce() -> crate::Result<std::sync::Arc<R>>;
}
