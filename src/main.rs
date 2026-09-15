use axum::{
    Json, Router, debug_handler,
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
struct Workout {
    workout_type: String,
    amount: i32,
    date: String,
}

#[tokio::main]
#[debug_handler]
async fn main() {
    let app = Router::new().route("/workout/v1", post(add_workout).get(previous_workout));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4985").await.unwrap();
}

#[debug_handler]
async fn add_workout(Json(workout): Json<Workout>) -> &'static str {
    let workout = workout;

    return "workout added";
}

#[debug_handler]
async fn previous_workout() {}
