use async_trait::async_trait;
use axum::{ Router };
use axum::routing::get;
use tower_http::cors::{Any, CorsLayer};


use crate::servers::api_server::ApiServer;

pub struct RestServer {
    port: i32
}

impl RestServer {
     pub fn new(port: i32) -> Self {
        RestServer { port}
     }
 }

impl RestServer {
    fn get_metadata(&self) -> String{
        "Hello world".to_string()
    }
}

#[async_trait]
impl ApiServer for RestServer {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cors : CorsLayer = CorsLayer::new().allow_origin(Any).allow_headers(Any).allow_methods(Any);
        let app : Router = Router::new()
            .layer(cors)
            .route("/getMetadata", get(self.get_metadata()));


        let listener = tokio::net::TcpListener::bind(format!("localhost:{}", self.port)).await.unwrap();
        axum::serve( listener, app).await.unwrap();

        Ok(())
    }

}
