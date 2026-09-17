use std::{default, env};

use anyhow::Result;
use axum::{
    Json, Router, debug_handler,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Deserialize, Clone, Debug, PartialEq, Default, Serialize)]
pub struct Workout {
    workout_type: String,
    amount: i32,
    date: chrono::NaiveDate,
}

#[derive(Debug)]
struct AppError(sqlx::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError(e)
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let app = Router::new()
        .route("/workout/v1", post(add_workout).get(previous_workout))
        .route("/testing/v1", post(testing));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4985").await.unwrap();
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?)
        .await
        .unwrap();
    axum::serve(listener, app.with_state(pool)).await.unwrap();
    Ok(())
}

#[debug_handler]
async fn add_workout(State(pool): State<MySqlPool>, Json(workout): Json<Workout>) -> String {
    let _workout = workout;
    let _pool = pool;
    return "Success".to_string();
}

#[debug_handler]
async fn previous_workout() {}
#[debug_handler]
async fn testing(State(pool): State<MySqlPool>) -> Result<(StatusCode, Json<Workout>), AppError> {
    let workout = Workout {
        workout_type: "arms".to_string(),
        amount: 1,
        date: chrono::NaiveDate::from_ymd_opt(2026, 9, 15).unwrap(),
    };
    let pool = pool;
    let add_workout = sqlx::query!(
        r#"
        INSERT INTO workouts (workout_type, amount, date)
        VALUES (?, ?, ?)
        "#,
        workout.workout_type,
        workout.amount,
        workout.date,
    )
    .execute(&pool)
    .await
    .map_err(|e| sqlx::Error::from(e))?;
    Ok((StatusCode::OK, Json(workout)))
}
