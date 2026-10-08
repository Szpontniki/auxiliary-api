mod routes;
mod app;
mod process;
mod database;
mod library;
mod environment;

use app::App;

#[tokio::main]
async fn main() {
    let app = App::new(3001);
    app.run().await;
}
