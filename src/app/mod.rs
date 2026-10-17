use crate::routes::{get_routes, route::Route};
use axum::Router;
use tracing::Level;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::{DefaultOnRequest, TraceLayer}};

pub struct App {
    port: i32,
    routes: Vec<Route>,
    router: Router,
}

impl App {
    pub fn new(port: i32) -> Self {
        let routes = get_routes();
        let router = Router::new();

        Self { port, routes, router }
    }

    async fn init_logging(&self) {
        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| EnvFilter::new("info,tower_http=info")),
            )
            .init();
    }

    pub async fn run(mut self) {
        self.init_logging().await;

        for route in &self.routes {
            self.router = self.router.route(&route.path, route.callback.clone());
        }

        self.router = self.router
            .layer(CorsLayer::permissive())
            .layer(
                TraceLayer::new_for_http()
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_request(DefaultOnRequest::new().level(Level::ERROR))
            );

        let address = SocketAddr::from(([127, 0, 0, 1], self.port as u16));
        let listener = match TcpListener::bind(address).await {
            Ok(listener) => listener,
            Err(err) => {
                tracing::error!(%address, error = %err, "Port is already in use!");
                return;
            }
        };

        tracing::info!(%address, "HTTP server listening.");

        if let Err(err) = axum::serve(listener, self.router).await {
            tracing::error!(error = %err, "HTTP server stopped with an error!");
        }
    }
}
