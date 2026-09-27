pub mod route;

mod process;

pub fn get_routes() -> Vec<route::Route> {
    let mut routes = Vec::new();

    routes.extend(process::routes());

    routes
}
