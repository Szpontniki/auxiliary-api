use crate::routes::{get_routes, route::Route};
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

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

    pub async fn run(mut self) {
        for route in &self.routes {
            self.router = self.router.route(&route.path, route.callback.clone());
        }

        self.router = self.router.layer(CorsLayer::permissive());

        let address = SocketAddr::from(([127, 0, 0, 1], self.port as u16));
        let listener = TcpListener::bind(address).await.expect("Port already in use!");

        axum::serve(listener, self.router).await.unwrap();
    }
}
