use crate::{
    api::{
        state::AuthenticationState,
        tag::{find_tagged_data_from_refs, sync_tagged_data_from_ref, TaggedData},
    },
    app::{util, AppState},
    database::Connection,
    model::{Bookmark, Tag, TaggedItem, TaggedType, BOOKMARK_TABLE},
};
use axum::{
    extract::{Extension, Json, Path, Query, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default)]
pub struct SearchBookmark {
    pub id: Option<String>,
    pub user_id: Option<String>,
}

pub async fn find_bookmarks(
    app_state: &AppState,
    search_params: SearchBookmark,
) -> Result<Vec<Bookmark>, ()> {
    let mut filters = app_state.new_query();
    filters.set_separator(" AND ");
    if let Some(id) = search_params.id.clone() {
        filters.push_str("id = ?").bind(id.into());
    }
    if let Some(user_id) = search_params.user_id.clone() {
        filters.push_str("user_id = ?").bind(user_id.into());
    }

    let mut query = app_state.new_query();
    query.push_str("SELECT * FROM ").push_str(BOOKMARK_TABLE);
    if !filters.is_empty() {
        query.push_str(" WHERE ").append(filters);
    }

    app_state
        .database()
        .connection()
        .fetch(query)
        .await
        .map_err(|_| ())?
        .into_iter()
        .map(|row| Bookmark::try_from(row))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())
}

pub async fn find_bookmark_one(
    app_state: &AppState,
    search_params: SearchBookmark,
) -> Result<Bookmark, ()> {
    find_bookmarks(app_state, search_params)
        .await?
        .pop()
        .ok_or(())
}

#[derive(Deserialize)]
pub struct BookmarkRequest {
    pub title: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct BookmarkResponse {
    pub id: String,
    pub title: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Bookmark> for BookmarkResponse {
    fn from(bookmark: Bookmark) -> Self {
        Self {
            id: bookmark.id,
            title: bookmark.title,
            url: bookmark.url,
            description: bookmark.description,
            tags: Vec::new(),
            created_at: bookmark.created_at,
            updated_at: bookmark.updated_at,
        }
    }
}

impl BookmarkResponse {
    fn with_tags(bookmark: Bookmark, tag_data: Vec<TaggedData>) -> Self {
        Self {
            id: bookmark.id,
            title: bookmark.title,
            url: bookmark.url,
            description: bookmark.description,
            tags: tag_data.iter().map(|data| data.0.path.clone()).collect(),
            created_at: bookmark.created_at,
            updated_at: bookmark.updated_at,
        }
    }
}

pub async fn list(
    Extension(app_state): Extension<AppState>,
    Extension(auth): Extension<AuthenticationState>,
    Query(search_params): Query<SearchBookmark>,
) -> Result<Json<Vec<BookmarkResponse>>, (StatusCode, String)> {
    let bookmarks = find_bookmarks(
        &app_state,
        SearchBookmark {
            user_id: auth.user_id(),
            ..search_params
        },
    )
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?;

    let tagged_result = find_tagged_data_from_refs(
        &app_state,
        TaggedType::Bookmark,
        bookmarks.clone().into_iter().map(|b| b.id).collect(),
    )
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?;

    Ok(Json(
        bookmarks
            .into_iter()
            .map(|bookmark| {
                let owned_tags = tagged_result.find_tags(bookmark.id.clone());
                BookmarkResponse::with_tags(bookmark, owned_tags)
            })
            .collect(),
    ))
}

pub async fn find(
    Extension(app_state): Extension<AppState>,
    Extension(auth): Extension<AuthenticationState>,
    Path(bookmark_id): Path<String>,
) -> Result<Json<BookmarkResponse>, (StatusCode, String)> {
    let bookmark = find_bookmark_one(
        &app_state,
        SearchBookmark {
            id: Some(bookmark_id),
            user_id: auth.user_id(),
        },
    )
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?;

    let owned_tags =
        find_tagged_data_from_refs(&app_state, TaggedType::Bookmark, vec![bookmark.id.clone()])
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))
            .map(|result| result.find_tags(bookmark.id.clone()))?;

    Ok(Json(BookmarkResponse::with_tags(bookmark, owned_tags)))
}

pub async fn create(
    Extension(app_state): Extension<AppState>,
    Extension(auth): Extension<AuthenticationState>,
    Json(payload): Json<BookmarkRequest>,
) -> Result<Json<BookmarkResponse>, (StatusCode, String)> {
    let bookmark = Bookmark {
        id: util::new_uid(),
        user_id: auth.user_id().unwrap(),
        title: payload.title.clone(),
        url: payload.url.clone(),
        description: payload.description.clone(),
        resource_id: None,
        created_at: Some(util::now()),
        updated_at: Some(util::now()),
    };
    let mut values = app_state.new_query();
    values.set_separator(", ");
    values
        .push_str("?")
        .bind(bookmark.id.clone().into())
        .push_str("?")
        .bind(bookmark.user_id.clone().into())
        .push_str("?")
        .bind(bookmark.title.clone().into())
        .push_str("?")
        .bind(bookmark.url.clone().into())
        .push_str("?")
        .bind(bookmark.description.clone().into())
        .push_str("?")
        .bind(bookmark.created_at.clone().into())
        .push_str("?")
        .bind(bookmark.updated_at.clone().into());
    let mut query = app_state.new_query();
    query
        .push_str("INSERT INTO ")
        .push_str(BOOKMARK_TABLE)
        .push_str(" (id, user_id, title, url, description, created_at, updated_at) VALUES (")
        .append(values)
        .push_str(")");
    app_state
        .database()
        .connection()
        .execute(query)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?;

    let owned_tags = if let Some(tag_inputs) = payload.tags {
        sync_tagged_data_from_ref(
            &app_state,
            auth.user_id().unwrap(),
            TaggedType::Bookmark,
            bookmark.id.clone(),
            tag_inputs
                .clone()
                .into_iter()
                .map(|s| {
                    TaggedData(
                        Tag::from_path(s),
                        TaggedItem {
                            ref_id: bookmark.id.clone(),
                            ..Default::default()
                        },
                    )
                })
                .collect(),
        )
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))
        .map(|result| result.find_tags(bookmark.id.clone()))?
    } else {
        Vec::new()
    };

    Ok(Json(BookmarkResponse::with_tags(bookmark, owned_tags)))
}

pub async fn update(
    Extension(app_state): Extension<AppState>,
    Extension(auth): Extension<AuthenticationState>,
    Path(bookmark_id): Path<String>,
    Json(payload): Json<BookmarkRequest>,
) -> Result<Json<BookmarkResponse>, (StatusCode, String)> {
    let mut bookmark = find_bookmark_one(
        &app_state,
        SearchBookmark {
            id: Some(bookmark_id.clone()),
            user_id: auth.user_id(),
        },
    )
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "".to_string()))?;

    bookmark.updated_at = Some(util::now());
    let mut values = app_state.new_query();
    values.set_separator(", ");
    values
        .push_str("updated_at = ?")
        .bind(bookmark.updated_at.clone().into());
    if let Some(url) = payload.url {
        bookmark.url = Some(url.clone());
        values.push_str("url = ?").bind(url.clone().into());
    }
    if let Some(title) = payload.title {
        bookmark.title = Some(title.clone());
        values.push_str("title = ?").bind(title.into());
    }
    if let Some(description) = payload.description {
        bookmark.description = Some(description.clone());
        values.push_str("description = ?").bind(description.into());
    }
    let mut query = app_state.new_query();
    query
        .push_str("UPDATE ")
        .push_str(BOOKMARK_TABLE)
        .push_str(" SET ")
        .append(values)
        .push_str(" WHERE id = ?")
        .bind(bookmark_id.into());
    app_state
        .database()
        .connection()
        .execute(query)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?;

    let owned_tags = if let Some(tag_inputs) = payload.tags {
        sync_tagged_data_from_ref(
            &app_state,
            auth.user_id().unwrap(),
            TaggedType::Bookmark,
            bookmark.id.clone(),
            tag_inputs
                .clone()
                .into_iter()
                .map(|s| {
                    TaggedData(
                        Tag::from_path(s),
                        TaggedItem {
                            ref_id: bookmark.id.clone(),
                            ..Default::default()
                        },
                    )
                })
                .collect(),
        )
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))
        .map(|result| result.find_tags(bookmark.id.clone()))?
    } else {
        Vec::new()
    };

    Ok(Json(BookmarkResponse::with_tags(bookmark, owned_tags)))
}

pub async fn delete(
    Extension(app_state): Extension<AppState>,
    Extension(auth): Extension<AuthenticationState>,
    Path(bookmark_id): Path<String>,
) -> Result<Json<BookmarkResponse>, (StatusCode, String)> {
    let bookmark = find_bookmark_one(
        &app_state,
        SearchBookmark {
            id: Some(bookmark_id.clone()),
            user_id: auth.user_id(),
        },
    )
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "".to_string()))?;

    let mut query = app_state.new_query();
    query
        .push_str("DELETE FROM ")
        .push_str(BOOKMARK_TABLE)
        .push_str(" WHERE id = ?")
        .bind(bookmark.id.clone().into());
    app_state
        .database()
        .connection()
        .execute(query)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?;

    Ok(Json(BookmarkResponse::from(bookmark)))
}
