use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, DateTime, Document};
use crate::{
    db::AppState,
    error::AppError,
    model::{Item, CreateItem, UpdateItem},
};

pub fn router(state: AppState) -> Router {
    Router::new()
        // 동일 경로에 메서드별로 따로 등록 (모두 free function 사용)
        .route("/items", get(list_items))
        .route("/items", post(create_item))
        .route("/items/{id}", get(get_item))
        .route("/items/{id}", put(update_item))
        .route("/items/{id}", delete(delete_item))
        .with_state(state)
}

// GET /items
async fn list_items(State(state): State<AppState>) -> Result<Json<Vec<Item>>, AppError> {
    let mut cursor = state.coll.find(doc! {}).await?;
    let mut out = Vec::new();
    while let Some(item) = cursor.try_next().await? {
        out.push(item);
    }
    Ok(Json(out))
}

// GET /items/{id}
async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Item>, AppError> {
    let oid = ObjectId::parse_str(&id).map_err(|_| AppError::BadRequest("invalid id".into()))?;
    let found = state
        .coll
        .find_one(doc! {"_id": oid})
        .await?
        .ok_or_else(|| AppError::NotFound("item not found".into()))?;
    Ok(Json(found))
}

// POST /items
async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateItem>,
) -> Result<(StatusCode, Json<Item>), AppError> {
    let now = DateTime::now();
    let item = Item {
        id: ObjectId::new(),
        name: payload.name,
        price: payload.price,
        created_at: now,
        updated_at: now,
    };
    state.coll.insert_one(&item).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

// PUT /items/{id}
async fn update_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(update): Json<UpdateItem>,
) -> Result<Json<Item>, AppError> {
    let oid = ObjectId::parse_str(&id).map_err(|_| AppError::BadRequest("invalid id".into()))?;
    let update_doc = to_update_doc(update);
    let result = state
        .coll
        .update_one(doc! {"_id": oid}, update_doc)
        .await?;
    if result.matched_count == 0 {
        return Err(AppError::NotFound("item not found".into()));
    }
    let updated = state
        .coll
        .find_one(doc! {"_id": oid})
        .await?
        .ok_or_else(|| AppError::NotFound("item not found".into()))?;
    Ok(Json(updated))
}

// DELETE /items/{id}
async fn delete_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    let oid = ObjectId::parse_str(&id).map_err(|_| AppError::BadRequest("invalid id".into()))?;
    let res = state.coll.delete_one(doc! {"_id": oid}).await?;
    if res.deleted_count == 0 {
        return Err(AppError::NotFound("item not found".into()));
    }
    Ok(StatusCode::NO_CONTENT)
}

fn to_update_doc(u: UpdateItem) -> Document {
    let mut set = Document::new();
    if let Some(name) = u.name { set.insert("name", name); }
    if let Some(price) = u.price { set.insert("price", price); }
    set.insert("updated_at", DateTime::now());
    doc! { "$set": set }
}
