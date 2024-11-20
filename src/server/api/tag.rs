use super::{AuthState, ErrorMsg};
use crate::{app::AppState, field, model::Tag, svc::TagSvc};
use axum::{
    extract::{Json, Path},
    Extension,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize)]
pub struct TagResponse {
    pub id: field::Id,
    pub path: String,
    pub prefix: String,
    pub name: String,
    pub label: Option<String>,
    pub parent_id: Option<field::Id>,
    pub depth: usize,
    pub value_type: String,
    pub created_at: field::DateTime,
    pub updated_at: field::DateTime,
}

impl From<Tag> for TagResponse {
    fn from(tag: Tag) -> Self {
        let Tag {
            id,
            path,
            prefix,
            name,
            label,
            parent_id,
            depth,
            value_type,
            created_at,
            updated_at,
            ..
        } = tag;
        Self {
            id,
            path,
            prefix,
            name,
            label,
            parent_id,
            depth,
            value_type: value_type.to_string(),
            created_at,
            updated_at,
        }
    }
}

#[derive(Deserialize)]
pub struct TagRequest {
    pub path: Option<String>,
    pub label: Option<String>,
    pub value_type: Option<String>,
}

impl TagRequest {
    fn fill_into(self, tag: &mut Tag) -> Result<(), String> {
        if let Some(path) = self.path {
            tag.path = path;
        }
        if let Some(label) = self.label {
            tag.label = Some(label);
        }
        if let Some(value_type) = self.value_type {
            tag.value_type = value_type
                .parse()
                .map_err(|_| "invalid value type".to_string())?;
        }
        Ok(())
    }

    fn validate_create(&self) -> Result<(), String> {
        if self.path.is_none() {
            return Err("path is required".to_string());
        }
        Ok(())
    }
}

pub async fn list(
    Extension(mut state): Extension<AppState>,
    Extension(auth): Extension<AuthState>,
) -> Result<Json<Vec<TagResponse>>, ErrorMsg> {
    TagSvc::new()
        .list_by_user(&mut state.db, auth.user.id.clone())
        .await
        .map_err(|e| ErrorMsg(500, e.to_string()))
        .map(|ts| Json(ts.into_iter().map(TagResponse::from).collect()))
}

pub async fn find(
    Extension(mut state): Extension<AppState>,
    Extension(auth): Extension<AuthState>,
    Path(tag_id): Path<field::Id>,
) -> Result<Json<TagResponse>, ErrorMsg> {
    TagSvc::new()
        .find_by_user_and_id(&mut state.db, auth.user.id.clone(), tag_id)
        .await
        .map_err(|e| ErrorMsg(500, e.to_string()))
        .map(|t| Json(TagResponse::from(t)))
}

pub async fn create(
    Extension(mut state): Extension<AppState>,
    Extension(auth): Extension<AuthState>,
    Json(payload): Json<TagRequest>,
) -> Result<Json<TagResponse>, ErrorMsg> {
    if let Err(msg) = payload.validate_create() {
        return Err(ErrorMsg(422, msg));
    }
    let mut tag = Tag {
        user_id: auth.user.id.clone(),
        ..Tag::default()
    };
    if let Err(msg) = payload.fill_into(&mut tag) {
        return Err(ErrorMsg(422, msg));
    }
    TagSvc::new()
        .create(&mut state.db, tag)
        .await
        .map_err(|e| ErrorMsg(500, e.to_string()))
        .map(|t| Json(TagResponse::from(t)))
}

pub async fn update(
    Extension(mut state): Extension<AppState>,
    Extension(auth): Extension<AuthState>,
    Path(tag_id): Path<field::Id>,
    Json(payload): Json<TagRequest>,
) -> Result<Json<TagResponse>, ErrorMsg> {
    let svc = TagSvc::new();
    let mut tag = svc
        .find_by_user_and_id(&mut state.db, auth.user.id.clone(), tag_id)
        .await
        .map_err(|e| ErrorMsg(404, e.to_string()))?;
    if let Err(msg) = payload.fill_into(&mut tag) {
        return Err(ErrorMsg(422, msg));
    }
    svc.update(&mut state.db, tag)
        .await
        .map_err(|e| ErrorMsg(500, e.to_string()))
        .map(|t| Json(TagResponse::from(t)))
}

pub async fn delete(
    Extension(mut state): Extension<AppState>,
    Extension(auth): Extension<AuthState>,
    Path(tag_id): Path<field::Id>,
) -> Result<Json<TagResponse>, ErrorMsg> {
    let svc = TagSvc::new();
    let tag = svc
        .find_by_user_and_id(&mut state.db, auth.user.id.clone(), tag_id)
        .await
        .map_err(|e| ErrorMsg(404, e.to_string()))?;
    svc.delete(&mut state.db, tag.clone())
        .await
        .map_err(|e| ErrorMsg(500, e.to_string()))
        .map(|_| Json(TagResponse::from(tag)))
}
