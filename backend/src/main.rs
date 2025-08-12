use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};
use std::net::SocketAddr;
use anyhow::Result;

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:./data/app.db")
        .await?;

    init_db(&pool).await?;

    let state = AppState { pool };

    let app = Router::new().route("/login", post(login)).with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn init_db(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query("INSERT OR IGNORE INTO users (username, password) VALUES (?, ?)")
        .bind("admin")
        .bind("password")
        .execute(pool)
        .await?;

    Ok(())
}

#[derive(Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> StatusCode {
    match sqlx::query("SELECT password FROM users WHERE username = ?")
        .bind(&payload.username)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(Some(row)) => {
            let stored: String = row.get("password");
            if stored == payload.password {
                StatusCode::OK
            } else {
                StatusCode::UNAUTHORIZED
            }
        }
        _ => StatusCode::UNAUTHORIZED,
    }
}
