pub mod client;
pub mod bucket;

use self::client::Storage;
use tokio::sync::OnceCell;

static STORAGE: OnceCell<Storage> = OnceCell::const_new();

pub async fn storage() -> &'static Storage {
    STORAGE
        .get_or_init(|| async {
            Storage::new().await
        })
        .await
}
