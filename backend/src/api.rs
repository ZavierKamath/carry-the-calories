mod models;

use sqlx::PgPool;
use axum::{
 Json, Router, extract::{
     Path, State, ws::{
         Message, WebSocket, WebSocketUpgrade
     }
  }, response::IntoResponse, routing::{get, post}
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
    let item = Log {
        id: Uuid::new_v4(),
        datetime: new_log.datetime,
        name: new_log.name,
        calories: new_log.calories,
        protein: new_log.protein,
        grams: None
    };
    Json(item)
}

pub async fn get_log(
    State(db): State<PgPool>,
    Path(id): Path<Uuid>
) -> Json<Log> {
    let item = Log {
        id,
        datetime: Utc::now(),
        name: String::from("test"),
        calories: 1.2_f64,
        protein: 3.4_f64,
        grams: None
    };
    Json(item)
}

pub async fn get_logs(
    State(db): State<PgPool>,
    Json(datetime): Json<DateTime<Utc>>
) -> Json<Vec<Log>> {
    let item = Log {
        id: Uuid::new_v4(),
        datetime,
        name: String::from("test"),
        calories: 1.2_f64,
        protein: 3.4_f64,
        grams: None
    };
    Json(vec![item])
}

pub async fn put_log(
    State(db): State<PgPool>,
    Json(updated_log): Json<Log>
) -> Json<Log> {
    Json(updated_log)
}

pub async fn del_log(
    State(db): State<PgPool>,
    Path(id): Path<Uuid>
) -> Json<Log> {
    Json(Log {
        id,
        datetime: Utc::now(),
        name: String::from("test"),
        calories: 1.2_f64,
        protein: 3.4_f64,
        grams: None
    })
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
