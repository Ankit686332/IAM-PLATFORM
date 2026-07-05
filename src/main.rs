mod app_state;
mod config;
mod errors;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod schemas;
mod services;
mod utils;
mod swagger;

use std::net::SocketAddr;

use app_state::AppState;
use axum::{
    extract::State,
    routing::get,
    Router,
};
use config::{
    database,
    settings::Settings,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::swagger::ApiDoc;

async fn bind_listener(host: &str, port: u16) -> Result<tokio::net::TcpListener, std::io::Error> {
    let candidate_ports = (port..=port + 10).collect::<Vec<_>>();

    for candidate_port in candidate_ports {
        let address = format!("{}:{}", host, candidate_port);

        match tokio::net::TcpListener::bind(&address).await {
            Ok(listener) => {
                if candidate_port != port {
                    println!("Configured port {port} was busy; using {candidate_port} instead.");
                }
                return Ok(listener);
            }
            Err(err) if err.kind() == std::io::ErrorKind::AddrInUse => continue,
            Err(err) => return Err(err),
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::AddrInUse,
        format!("Unable to bind to {host}:{port}"),
    ))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let settings = Settings::new();

    let db = database::connect(&settings.database_url)
        .await
        .expect("Failed to connect to database");

    println!("Database Connected Successfully");

    let state = AppState { db };

    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .nest("/auth", routes::auth_routes::auth_routes())
        .nest("/users", routes::user_routes::user_routes())
        .nest("/organizations", routes::organization_routes::organization_routes())
        .nest("/memberships", routes::membership_routes::membership_routes())
        .nest("/roles", routes::role_routes::role_routes())
        .nest("/permissions", routes::permission_routes::permission_routes())
        .nest("/role-permissions", routes::role_permission_routes::role_permission_routes())
        .nest("/member-roles", routes::member_role_routes::member_role_routes())
        .nest("/sessions", routes::session_routes::session_routes())
        .nest("/api-keys", routes::api_key_routes::api_key_routes())
        .merge(
           SwaggerUi::new("/swagger-ui")
           .url("/api-docs/openapi.json", ApiDoc::openapi()),
         )
        .with_state(state);

    let listener = bind_listener(&settings.host, settings.port)
        .await
        .expect("Failed to bind server");

    let address = listener
        .local_addr()
        .expect("Failed to read bound address");

    println!("Server running at http://{}", address);

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

async fn home() -> &'static str {
    "IAM Platform Backend is Running"
}

async fn health(
    State(state): State<AppState>,
) -> &'static str {
    let _ = &state.db;
    "Database Connected"
}