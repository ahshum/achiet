use super::state::AuthenticationState;
use crate::{
    app::AppState,
    database::Connection,
    model::{Tag, TAG_TABLE},
    repo,
    taskqueue::Task,
};
use axum::{
    extract::{Extension, Json, Path, Query},
    http::StatusCode,
};
use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TagResponse {
    pub id: String,
    pub path: String,
    pub prefix: String,
    pub name: String,
    pub label: Option<String>,
    pub parent_id: Option<String>,
    pub depth: u32,
    pub value_type: Option<String>,
    pub user_id: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Tag> for TagResponse {
    fn from(tag: Tag) -> Self {
        Self {
            id: tag.id,
            path: tag.path,
            prefix: tag.prefix,
            name: tag.name,
            label: tag.label,
            parent_id: tag.parent_id,
            depth: tag.depth,
            value_type: tag.value_type,
            user_id: tag.user_id,
            created_at: tag.created_at,
            updated_at: tag.updated_at,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateTagRequest {
    pub path: String,
    pub label: Option<String>,
    pub value_type: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdatedTagRequest {
    pub path: Option<String>,
    pub label: Option<String>,
    pub value_type: Option<String>,
}

pub async fn list(
    Extension(app_state): Extension<AppState>,
    Extension(auth): Extension<AuthenticationState>,
    Query(search_params): Query<repo::tag::SearchTag>,
) -> Result<Json<Vec<TagResponse>>, (StatusCode, String)> {
    let tags = repo::tag::find_tags(
        &app_state,
        repo::tag::SearchTag {
            user_id: auth.user_id(),
            ..search_params
        },
    )
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?;

    Ok(Json(
        tags.into_iter().map(|t| TagResponse::from(t)).collect(),
    ))
}

pub async fn find(
    Extension(app_state): Extension<AppState>,
    Extension(auth): Extension<AuthenticationState>,
    Path(tag_id): Path<String>,
) -> Result<Json<TagResponse>, (StatusCode, String)> {
    let tag = repo::tag::find_tags(
        &app_state,
        repo::tag::SearchTag {
            id_vec: Some(vec![tag_id]),
            user_id: auth.user_id(),
            ..Default::default()
        },
    )
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?
    .pop()
    .ok_or((StatusCode::NOT_FOUND, "".to_string()))?;

    Ok(Json(TagResponse::from(tag)))
}

pub async fn create(
    Extension(app_state): Extension<AppState>,
    Extension(auth): Extension<AuthenticationState>,
    Json(payload): Json<CreateTagRequest>,
) -> Result<Json<TagResponse>, (StatusCode, String)> {
    let value = Tag {
        user_id: auth.user_id().unwrap(),
        label: payload.label,
        value_type: payload.value_type,
        ..Tag::from_path(payload.path)
    };

    let tag = repo::tag::create_tags(&app_state, vec![value])
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?
        .pop()
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?;

    Ok(Json(TagResponse::from(tag)))
}

pub async fn update(
    Extension(app_state): Extension<AppState>,
    Extension(auth): Extension<AuthenticationState>,
    Path(tag_id): Path<String>,
    Json(payload): Json<UpdatedTagRequest>,
) -> Result<Json<TagResponse>, (StatusCode, String)> {
    let mut tag = repo::tag::find_tags(
        &app_state,
        repo::tag::SearchTag {
            id_vec: Some(vec![tag_id.clone()]),
            user_id: auth.user_id(),
            ..Default::default()
        },
    )
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?
    .pop()
    .ok_or((StatusCode::NOT_FOUND, "".to_string()))?;

    if let Some(label) = payload.label.clone() {
        tag.label = Some(label.clone());
    }
    if let Some(value_type) = payload.value_type.clone() {
        tag.value_type = Some(value_type.clone());
    }

    let tag = repo::tag::update_tags(&app_state, vec![tag])
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?
        .pop()
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?;

    Ok(Json(TagResponse::from(tag)))
}

pub async fn delete(
    Extension(app_state): Extension<AppState>,
    Extension(auth): Extension<AuthenticationState>,
    Path(tag_id): Path<String>,
) -> Result<Json<TagResponse>, (StatusCode, String)> {
    let tag = repo::tag::find_tags(
        &app_state,
        repo::tag::SearchTag {
            id_vec: Some(vec![tag_id.clone()]),
            user_id: auth.user_id(),
            ..Default::default()
        },
    )
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?
    .pop()
    .ok_or((StatusCode::NOT_FOUND, "".to_string()))?;

    let tag = repo::tag::delete_tags(&app_state, vec![tag])
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?
        .pop()
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?;

    Ok(Json(TagResponse::from(tag)))
}
