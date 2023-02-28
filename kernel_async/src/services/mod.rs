use alloc::boxed::Box;

// pub mod handle_service;

#[async_trait]
pub trait KernelService {
    async fn run(self);
}
