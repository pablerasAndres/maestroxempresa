mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::mysql::MySqlPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL debe estar configurada en el archivo .env");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("No se pudo conectar a MySQL");

    println!("Conexión a la base de datos MySQL exitosa.");

    // Configuración de CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Servidor web + API
    let app = Router::new()
        // Servir los archivos web (HTML, CSS, JS) desde la carpeta /public
        .nest_service("/", ServeDir::new("public"))
        // Rutas API del backend
        .route("/api/registro", post(handlers::registrar_usuario))
        .route("/api/login", post(handlers::login_usuario))
        .route("/api/ofertas", post(handlers::crear_oferta))
        .route("/api/ofertas", get(handlers::obtener_ofertas))
        .route("/api/perfil/:id", get(handlers::obtener_perfil))
        .route("/api/postular", post(handlers::postular_a_oferta))
        .layer(cors)
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}