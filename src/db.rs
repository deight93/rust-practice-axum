use anyhow::Result;
use mongodb::{Client, Collection};
use crate::model::Item;
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub coll: Collection<Item>,
}

impl AppState {
    pub async fn init_from_env() -> Result<Self> {
        let uri = env::var("MONGODB_URI")?;
        let db_name = env::var("DB_NAME")?;
        let coll_name = env::var("COLL_NAME")?;

        let client = Client::with_uri_str(&uri).await?;
        let db = client.database(&db_name);
        let coll = db.collection::<Item>(&coll_name);

        Ok(Self { coll })
    }
}
