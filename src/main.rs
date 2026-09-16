use std::env;

use anyhow::Result;
use axum::{
    Json, Router, debug_handler,
    routing::{get, post},
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
struct Workout {
    workout_type: String,
    amount: i32,
    date: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let app = Router::new().route("/workout/v1", post(add_workout).get(previous_workout));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4985").await.unwrap();
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?)
        .await
        .unwrap();
    axum::serve(listener, app.with_state(pool)).await.unwrap();
    Ok(())
}

#[debug_handler]
async fn add_workout(pool: MySqlPool, Json(workout): Json<Workout>) {
    let workout = workout;
}

#[debug_handler]
async fn previous_workout() {}
