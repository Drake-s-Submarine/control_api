use axum::{
    routing::{get, post},
    response::IntoResponse,
    http::StatusCode,
    Router,
    Json
};
use common::commands::{
    BallastCommand,
    LightCommand,
    PropulsionCommand,
    Vector2d,
    serde::Serde,
};
use serde::Deserialize;
use std::os::unix::net::UnixStream;
use std::io::Write;
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize)]
struct BallastRequest {
    desired_state: DesiredBallastState,
}

#[derive(Debug, Deserialize)]
enum DesiredBallastState {
    Idle,
    Discharge,
    Intake,
}

#[derive(Debug, Deserialize)]
struct PropRequest {
    x: f32,
    y: f32,
}

#[derive(Debug, Deserialize)]
struct LightRequest {
    desired_state: DesiredLightState,
}

#[derive(Debug, Deserialize)]
enum DesiredLightState {
    On,
    Off,
    Blink
}

fn get_unix_stream() -> UnixStream {
    // TODO: Move this path to common
    UnixStream::connect("/tmp/sub_cmd_socket").unwrap()
}

#[tokio::main]
async fn main() {

    let api_router = Router::new()
        .route("/test", get(test))
        .route("/ballast", post(set_ballast_state))
        .route("/propulsion", post(set_propulsion_state))
        .route("/light", post(set_light_state));
    let app = Router::new()
        .nest("/api", api_router)
        .nest_service("/", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn test() -> impl IntoResponse {
    "Hello World"
}

async fn set_ballast_state(Json(req): Json<BallastRequest>) -> impl IntoResponse {
    let command = match req.desired_state {
        DesiredBallastState::Idle => BallastCommand::Idle,
        DesiredBallastState::Discharge => BallastCommand::Discharge,
        DesiredBallastState::Intake => BallastCommand::Intake,
    };

    let mut stream = get_unix_stream();

    let _ = stream.write_all(&command.serialize());
    
    StatusCode::OK
}

async fn set_propulsion_state(Json(req): Json<PropRequest>) -> impl IntoResponse {
    let command = PropulsionCommand::SetThrust(Vector2d{x: req.x, y: req.y});

    println!("{:?}", command);

    let mut stream = get_unix_stream();

    let _ = stream.write_all(&command.serialize());

    StatusCode::OK
}

async fn set_light_state(Json(req): Json<LightRequest>) -> impl IntoResponse {
    let command = match req.desired_state {
        DesiredLightState::On => LightCommand::On,
        DesiredLightState::Off => LightCommand::Off,
        DesiredLightState::Blink => LightCommand::Blink
    };

    let mut stream = get_unix_stream();

    let _ = stream.write_all(&command.serialize());

    StatusCode::OK
}
