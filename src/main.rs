use crate::{servers::{api_server::ApiServer, rest_server::RestServer}, util::mpd::MpdClient};
mod servers;
mod util;

#[tokio::main]
async fn main() {
    let rest: Box<dyn ApiServer> = Box::new(RestServer::new(6969));
    let mpd = MpdClient::new("localhost:6600").unwrap();

    rest.run(mpd).await;
}

