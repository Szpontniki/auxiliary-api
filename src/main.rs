mod routes;
mod app;

use app::App;

#[tokio::main]
async fn main() {
    let app = App::new(3000);
    app.run().await;
}
