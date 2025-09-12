use std::sync::Arc;
use async_trait::async_trait;
use axum::{ body::Body, extract::State, http::{header, Response}, response::IntoResponse, routing::{get, post}, Json, Router };
use mpd::{Id, Song, Status};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use crate::{servers::api_server::ApiServer, util::mpd::{MpdClient, SongData}};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct FileList {
    files : Vec<String>
}

struct AppState {
    mpd_client : Mutex<MpdClient>
}

#[derive(Deserialize)]
struct SearchParams{
    value: String
}

#[derive(Deserialize)]
struct AddQueueParams{
    file: String
}

#[derive(Deserialize)]
struct RemoveQueueParams{
    pos: u32
}

#[derive(Deserialize)]
struct SeekCurrParams {
    time: f64
}

pub struct RestServer {
    port: i32,
}

impl RestServer {
     pub fn new(port: i32) -> Self {
        RestServer { port }
     }
 }

async fn search_mpd(state: State<Arc<AppState>>, Json(payload): Json<SearchParams>) -> Json<Vec<Song>>{
    let mut client = state.mpd_client.lock().await;
    Json(client.search(&payload.value).unwrap())
}

async fn play_mpd(state: State<Arc<AppState>>) -> (){
    let mut client = state.mpd_client.lock().await;
    client.play().unwrap();
}

async fn stop_mpd(state: State<Arc<AppState>>) -> (){
    let mut client = state.mpd_client.lock().await;
    client.stop().unwrap();
}

async fn toggle_play_mpd(state: State<Arc<AppState>>) -> () {
    let mut client = state.mpd_client.lock().await;
    client.toggle_play().unwrap();
}

async fn playback_status_mpd(state: State<Arc<AppState>>) -> Json<Status> {
    let mut client = state.mpd_client.lock().await;
    Json(client.playback_status().unwrap())
}

async fn add_song_queue_mpd(state: State<Arc<AppState>>, Json(payload) : Json<AddQueueParams>) -> Json<Id> {
    let mut client = state.mpd_client.lock().await;
    let mut song = Song::default();
    song.file = payload.file;
    Json(client.add_to_queue(song).unwrap())
}


async fn remove_song_queue_mpd(state: State<Arc<AppState>>, Json(payload) : Json<RemoveQueueParams>) -> () {
    let mut client = state.mpd_client.lock().await;
    client.remove_from_queue(payload.pos).unwrap();
}

async fn list_queue_mpd(state: State<Arc<AppState>>) -> Json<Vec<Song>> {
    let mut client = state.mpd_client.lock().await;
    Json(client.list_queue().unwrap())
}

async fn seek_curr_mpd(state: State<Arc<AppState>>, Json(payload): Json<SeekCurrParams>) -> () {
    let mut client = state.mpd_client.lock().await;
    client.seek_curr(payload.time).unwrap()
}

async fn get_album_art_mpd(state: State<Arc<AppState>>, Json(payload) : Json<AddQueueParams>) -> impl IntoResponse {
    let mut song = Song::default();
    song.file = payload.file;
    let mut client = state.mpd_client.lock().await;
    let image = client.get_album_art(song).unwrap();
    Response::builder()
        .header(header::CONTENT_TYPE, "image/jpg")
        .body(Body::from(image))
        .unwrap()
}

#[async_trait]
impl ApiServer for RestServer {
    async fn run(&self, mpd_client : MpdClient) -> Result<(), Box<dyn std::error::Error>> {
        let cors : CorsLayer = CorsLayer::new().allow_origin(Any).allow_headers(Any).allow_methods(Any);
        let state = Arc::new(AppState { mpd_client : Mutex::new(mpd_client)});
        let app = Router::new()
            .route("/search", post(search_mpd))
            .route("/play", get(play_mpd))
            .route("/stop", get(stop_mpd))
            .route("/togglePlay", get(toggle_play_mpd))
            .route("/playbackStatus", get(playback_status_mpd))
            .route("/addSongQueue", post(add_song_queue_mpd))
            .route("/listQueue", get(list_queue_mpd))
            .route("/removeSongQueue", post(remove_song_queue_mpd))
            .route("/seekCurr", post(seek_curr_mpd))
            .route("/getAlbumArt", post(get_album_art_mpd))
            .layer(cors)
            .with_state(state);


        let listener = tokio::net::TcpListener::bind(format!("localhost:{}", self.port)).await.unwrap();
        axum::serve( listener, app.into_make_service()).await.unwrap();

        Ok(())
    }

}
