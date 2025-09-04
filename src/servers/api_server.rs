use async_trait::async_trait;

#[async_trait]
pub trait ApiServer {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}
