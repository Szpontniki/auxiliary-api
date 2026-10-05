mod routes;
mod app;
mod process;
mod database;

use app::App;

use crate::database::models::library_entry::NewLibraryEntry;

#[tokio::main]
async fn main() {
    let app = App::new(3001);
    app.run().await;

    NewLibraryEntry { };
}
