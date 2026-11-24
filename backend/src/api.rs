mod models;

use sqlx::PgPool;
use axum::{
 Json, Router,
 extract::{
     Path, State,
     ws::{Message, WebSocket, WebSocketUpgrade}
  },
  response::IntoResponse, routing::{get, post}
};
use uuid::Uuid;
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
use chrono::{DateTime, Utc};
use dotenvy;


pub async fn post_log(
    State(db): State<PgPool>, Json(new_log): Json<NewLog>
) -> Json<Log> {
    let new_uuid = Uuid::new_v4();
    Json(sqlx::query_as(
        "INSERT INTO logs (id, datetime, name, calories, protein, grams)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *"
    ).bind(new_uuid).bind(new_log.datetime).bind(new_log.name)
    .bind(new_log.calories).bind(new_log.protein).bind(new_log.grams)
    .fetch_one(&db).await.unwrap())
}

pub async fn get_log(
    State(db): State<PgPool>,
    Path(id): Path<Uuid>
) -> Json<Log> {
    Json(sqlx::query_as("SELECT * FROM logs WHERE id=$1")
        .bind(id).fetch_one(&db).await.unwrap())
}

pub async fn get_logs(
    State(db): State<PgPool>,
    Json(datetime): Json<DateTime<Utc>>
) -> Json<Vec<Log>> {
    Json(sqlx::query_as("SELECT * FROM logs WHERE datetime=$1")
        .bind(datetime).fetch_all(&db).await.unwrap())
}

pub async fn put_log(
    State(db): State<PgPool>,
    Json(updated_log): Json<Log>
) -> Json<Log> {
    Json(sqlx::query_as(
        "INSERT INTO logs (id, datetime, name, calories, protein, grams)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *"
    ).bind(updated_log.id).bind(updated_log.datetime).bind(updated_log.name)
    .bind(updated_log.calories).bind(updated_log.protein).bind(updated_log.grams)
    .fetch_one(&db).await.unwrap())
}

pub async fn del_log(State(db): State<PgPool>, Path(id): Path<Uuid>) {
    sqlx::query("DELETE FROM logs WHERE id=$1")
        .bind(id).execute(&db).await.unwrap();
}

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
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPool::connect(&url).await.unwrap();

    let app = Router::new()
        .route("/logs", post(post_log).get(get_logs))
        .route("/logs/{id}", get(get_log).put(put_log).delete(del_log))
        .route("/audio", get(ws_upgrade))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
