use axum::routing::MethodRouter;

pub struct Route {
    pub path: String,
    pub callback: MethodRouter,
}

impl Route {
    pub fn new(path: String, callback: MethodRouter) -> Self {
        Self { path, callback }
    }
}
