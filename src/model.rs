use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub price: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub price: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateItem {
    pub name: Option<String>,
    pub price: Option<i32>,
}
