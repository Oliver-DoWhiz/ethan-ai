use std::net::SocketAddr;

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::domain::{generate, templates, GenerateRequest};

#[derive(Clone, Default)]
pub struct AppState;

#[derive(Debug, Serialize)]
struct ErrorPayload {
    error: String,
}

pub fn build_app() -> Router {
    Router::new()
        .route("/", get(landing))
        .route("/studio", get(studio))
        .route("/api/templates", get(template_catalog))
        .route("/api/plan", post(generate_plan))
        .nest_service(
            "/static",
            ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .with_state(AppState)
        .layer(TraceLayer::new_for_http())
}

pub async fn serve() {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ethan_ai=info,tower_http=info".into()),
        )
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    let app = build_app();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Ethan AI listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("listener should bind");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server should run");
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}

async fn landing() -> Html<&'static str> {
    Html(include_str!("../static/landing.html"))
}

async fn studio() -> Html<&'static str> {
    Html(include_str!("../static/studio.html"))
}

async fn template_catalog(
    State(_): State<AppState>,
) -> Json<&'static [crate::domain::TemplateDefinition]> {
    Json(templates())
}

async fn generate_plan(
    State(_): State<AppState>,
    Json(payload): Json<GenerateRequest>,
) -> Response {
    match generate(payload) {
        Ok(result) => Json(result).into_response(),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorPayload {
                error: err.message(),
            }),
        )
            .into_response(),
    }
}
