pub mod route;

mod process;
mod library;

pub fn get_routes() -> Vec<route::Route> {
    let mut routes = Vec::new();

    routes.extend(process::routes());
    routes.extend(library::routes());

    routes
}
