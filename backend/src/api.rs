use sqlx::PgPool;
use axum::{
 Json, Router, extract::{
     Path, State, ws::{
         Message, WebSocket, WebSocketUpgrade
     }
  }, response::IntoResponse, routing::{get, post}, serve::Listener
};
use uuid::Uuid;

mod models;

use models::{
    NewLog,
    Log,
    NewDay,
    Day,
    NewRecipe,
    Recipe,
    Ingredient,
    NewIngredient
};

pub async fn post_log(
    State(db): State<PgPool>, Json(new_log): Json<NewLog>
) -> Json<Log> {pass}

pub async fn get_logs(
    State(db): State<PgPool>,
    Json(datetime): Json<DateTime>
) -> Json<Vec<Log>> {pass}

pub async fn put_log(
    State(db): State<PgPool>,
    Json(updated_log): Json<Log>
) -> Json<Log> {pass}

pub async fn del_log(
    State(db): State<PgPool>,
    id: Uuid
) -> Json<Log> {pass}

pub async fn ws_upgrade(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(audio)
}

pub async fn audio(mut sock: WebSocket) {
    while let Some(Ok(msg)) = sock.recv().await {
        if let Message::Binary(bytes) = msg {
            println!("got {} audio bytes", bytes.len());
        }
    }
}

#[tokio::main]
async fn main() {
    // serve the api
    let db = PgPool::connect(
        &std::env::var("DATABASE_URL").unwrap()
    ).await.unwrap();

    let app = Router::new()
        .route("/logs", post(post_log).get(get_logs))
        .route("/logs/:id", get(get_log).put(put_log).delete(del_log))
        .route("/audio", get(ws_upgrade))
        .with_state(db);

    let listener = toko::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
