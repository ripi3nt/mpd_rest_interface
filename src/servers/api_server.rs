use async_trait::async_trait;

use crate::util::mpd::MpdClient;

#[async_trait]
pub trait ApiServer {
    async fn run(&self, mpd_client : MpdClient) -> Result<(), Box<dyn std::error::Error>>;
}
