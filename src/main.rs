mod routes;
mod app;
mod process;

use app::App;

#[tokio::main]
async fn main() {
    let app = App::new(3001);
    app.run().await;
}
