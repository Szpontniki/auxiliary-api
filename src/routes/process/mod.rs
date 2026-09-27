use crate::routes::route::Route;

mod image;

pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();

    routes.push(image::route());

    routes
}
