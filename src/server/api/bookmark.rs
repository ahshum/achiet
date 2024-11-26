use super::{AuthState, ErrorMsg};
use crate::{
    app::AppState,
    field,
    model::{Bookmark, Tag, Tagging},
    svc::BookmarkSvc,
};
use axum::{
    extract::{Json, Path},
    Extension,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkResponse {
    pub id: field::Id,
    pub url: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    pub created_at: field::DateTime,
    pub updated_at: field::DateTime,
}

impl BookmarkResponse {
    fn from_minimal(value: (Bookmark, Vec<(Tag, Tagging)>)) -> Self {
        let (
            Bookmark {
                id,
                url,
                title,
                created_at,
                updated_at,
                ..
            },
            tgs,
        ) = value;
        Self {
            id,
            url,
            title,
            created_at,
            updated_at,
            tags: Some(
                tgs.into_iter()
                    .map(|tg| TaggingResponse::from(tg).to_string())
                    .collect(),
            ),
            ..Self::default()
        }
    }
}

impl From<(Bookmark, Vec<(Tag, Tagging)>)> for BookmarkResponse {
    fn from(value: (Bookmark, Vec<(Tag, Tagging)>)) -> Self {
        let (
            Bookmark {
                id,
                url,
                title,
                description,
                created_at,
                updated_at,
                ..
            },
            tgs,
        ) = value;
        Self {
            id,
            url,
            title,
            description: Some(description),
            created_at,
            updated_at,
            tags: Some(
                tgs.into_iter()
                    .map(|tg| TaggingResponse::from(tg).to_string())
                    .collect(),
            ),
            ..Self::default()
        }
    }
}

struct TaggingResponse(Tag, Tagging);

impl From<(Tag, Tagging)> for TaggingResponse {
    fn from(tgs: (Tag, Tagging)) -> Self {
        Self(tgs.0, tgs.1)
    }
}

impl std::fmt::Display for TaggingResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.1.value {
            Some(value) => write!(f, "{}:{}", self.0.path, value),
            None => write!(f, "{}", self.0.path),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkRequest {
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl BookmarkRequest {
    fn fill_into(self, bm: &mut Bookmark) -> Result<(), String> {
        if let Some(url) = self.url {
            bm.url = url;
        }
        if let Some(title) = self.title {
            bm.title = title;
        }
        if let Some(description) = self.description {
            bm.description = description;
        }
        Ok(())
    }

    fn validate_create(&self) -> Result<(), String> {
        if self.url.is_none() && self.title.is_none() {
            return Err("url or title is required".to_string());
        }
        Ok(())
    }
}

pub async fn list(
    Extension(mut state): Extension<AppState>,
    Extension(auth): Extension<AuthState>,
) -> Result<Json<Vec<BookmarkResponse>>, ErrorMsg> {
    BookmarkSvc::new()
        .list_by_user(&mut state.db, auth.user.id.clone())
        .await
        .map_err(|e| ErrorMsg(500, e.to_string()))
        .map(|bs| Json(bs.into_iter().map(BookmarkResponse::from_minimal).collect()))
}

pub async fn find(
    Extension(mut state): Extension<AppState>,
    Extension(auth): Extension<AuthState>,
    Path(bm_id): Path<field::Id>,
) -> Result<Json<BookmarkResponse>, ErrorMsg> {
    BookmarkSvc::new()
        .find_by_user_and_id(&mut state.db, auth.user.id.clone(), bm_id)
        .await
        .map_err(|e| ErrorMsg(500, e.to_string()))
        .map(|bm_tgs| Json(BookmarkResponse::from(bm_tgs)))
}

pub async fn create(
    Extension(mut state): Extension<AppState>,
    Extension(auth): Extension<AuthState>,
    Json(mut payload): Json<BookmarkRequest>,
) -> Result<Json<BookmarkResponse>, ErrorMsg> {
    if let Err(msg) = payload.validate_create() {
        return Err(ErrorMsg(422, msg));
    }
    let mut bm = Bookmark {
        user_id: auth.user.id.clone(),
        ..Bookmark::default()
    };
    let tags_payload = payload.tags.take();
    if let Err(msg) = payload.fill_into(&mut bm) {
        return Err(ErrorMsg(422, msg));
    }

    let svc = BookmarkSvc::new();
    let bm = svc
        .create(&mut state.db, bm)
        .await
        .map_err(|e| ErrorMsg(500, e.to_string()))?;
    let tgs = if let Some(tag_strs) = tags_payload {
        svc.replace_tags(&mut state.db, bm.clone(), tag_strs)
            .await
            .map_err(|e| ErrorMsg(500, e.to_string()))?
    } else {
        Vec::new()
    };

    Ok(Json(BookmarkResponse::from((bm, tgs))))
}

pub async fn update(
    Extension(mut state): Extension<AppState>,
    Extension(auth): Extension<AuthState>,
    Path(bm_id): Path<field::Id>,
    Json(mut payload): Json<BookmarkRequest>,
) -> Result<Json<BookmarkResponse>, ErrorMsg> {
    let svc = BookmarkSvc::new();
    let (mut bm, _) = svc
        .find_by_user_and_id(&mut state.db, auth.user.id.clone(), bm_id)
        .await
        .map_err(|e| ErrorMsg(404, e.to_string()))?;
    let tags_payload = payload.tags.take();
    if let Err(msg) = payload.fill_into(&mut bm) {
        return Err(ErrorMsg(422, msg));
    }

    let bm = svc
        .update(&mut state.db, bm)
        .await
        .map_err(|e| ErrorMsg(500, e.to_string()))?;
    let tgs = if let Some(tag_strs) = tags_payload {
        svc.replace_tags(&mut state.db, bm.clone(), tag_strs)
            .await
            .map_err(|e| ErrorMsg(500, e.to_string()))?
    } else {
        Vec::new()
    };

    Ok(Json(BookmarkResponse::from((bm, tgs))))
}

pub async fn delete(
    Extension(mut state): Extension<AppState>,
    Extension(auth): Extension<AuthState>,
    Path(bm_id): Path<field::Id>,
) -> Result<Json<BookmarkResponse>, ErrorMsg> {
    let svc = BookmarkSvc::new();
    let bm_tgs = svc
        .find_by_user_and_id(&mut state.db, auth.user.id.clone(), bm_id)
        .await
        .map_err(|e| ErrorMsg(404, e.to_string()))?;
    svc.delete(&mut state.db, bm_tgs.0.clone())
        .await
        .map_err(|e| ErrorMsg(500, e.to_string()))
        .map(|_| Json(BookmarkResponse::from(bm_tgs)))
}
