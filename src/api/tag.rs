use crate::{
    api::state::AuthenticationState,
    app::{util, AppState},
    database::Connection,
    model::{Tag, TaggedItem, TaggedType, TAG_TABLE},
};
use axum::{
    extract::{Extension, Json, Path, Query},
    http::StatusCode,
};
use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Default, Deserialize)]
pub struct SearchTag {
    pub id_vec: Option<Vec<String>>,
    pub tag_path_vec: Option<Vec<String>>,
    pub user_id: Option<String>,
    pub parent_id: Option<String>,
    pub depth: Option<u32>,
}

pub async fn find_tags(app_state: &AppState, search_params: SearchTag) -> Result<Vec<Tag>, ()> {
    let mut filters = app_state.new_query();
    filters.set_separator(" AND ");
    if let Some(user_id) = search_params.user_id.clone() {
        filters.push_str("user_id = ?").bind(user_id.into());
    }
    if let Some(parent_id) = search_params.parent_id.clone() {
        filters.push_str("parent_id = ?").bind(parent_id.into());
    }
    if let Some(depth) = search_params.depth.clone() {
        filters.push_str("depth = ?").bind(depth.into());
    }
    if let Some(tag_path_vec) = search_params.tag_path_vec.clone() {
        filters.push_str(&format!(
            "path IN ({})",
            vec!["?"; tag_path_vec.len()].join(",")
        ));
        tag_path_vec.into_iter().for_each(|path| {
            filters.bind(path.into());
        });
    }
    if let Some(id_vec) = search_params.id_vec.clone() {
        filters.push_str(&format!("id IN ({})", vec!["?"; id_vec.len()].join(",")));
        id_vec.into_iter().for_each(|tag_id| {
            filters.bind(tag_id.into());
        });
    }

    let mut query = app_state.new_query();
    query.push_str("SELECT * FROM ").push_str(TAG_TABLE);
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
        .map(|row| Tag::try_from(row))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())
}

pub async fn create_tags(app_state: &AppState, tag_inputs: Vec<Tag>) -> Result<Vec<Tag>, ()> {
    let mut values = app_state.new_query();
    values.set_separator("), (");
    let tags = tag_inputs
        .into_iter()
        .map(|input| {
            let tag = Tag {
                id: util::new_uid(),
                created_at: Some(util::now()),
                updated_at: Some(util::now()),
                ..input
            };

            values
                .push_str(&vec!["?"; 10].join(","))
                .bind(tag.id.clone().into())
                .bind(tag.path.clone().into())
                .bind(tag.prefix.clone().into())
                .bind(tag.name.clone().into())
                .bind(tag.depth.clone().into())
                .bind(tag.parent_id.clone().into())
                .bind(tag.user_id.clone().into())
                .bind(tag.value_type.clone().into())
                .bind(tag.created_at.clone().into())
                .bind(tag.updated_at.clone().into());

            tag
        })
        .collect();

    let mut query = app_state.new_query();
    query
        .push_str("INSERT INTO ")
        .push_str(TAG_TABLE)
        .push_str(" (id, path, prefix, name, depth, parent_id, user_id, value_type, created_at, updated_at) VALUES (")
        .append(values)
        .push_str(")");
    app_state
        .database()
        .connection()
        .execute(query)
        .await
        .map_err(|_| ())?;

    Ok(tags)
}

pub async fn sync_tags(
    app_state: &AppState,
    user_id: String,
    tag_inputs: Vec<Tag>,
) -> Result<Vec<Tag>, ()> {
    let tag_paths_iter = tag_inputs.clone().into_iter().map(|t| t.path);

    let mut exist_tags = find_tags(
        app_state,
        SearchTag {
            tag_path_vec: Some(tag_paths_iter.clone().collect()),
            user_id: Some(user_id.clone()),
            ..Default::default()
        },
    )
    .await
    .map_err(|_| ())?;

    let exist_set: HashSet<_> = exist_tags.clone().into_iter().map(|t| t.path).collect();
    let input_set: HashSet<_> = tag_paths_iter.clone().collect();
    let new_paths_iter = input_set.difference(&exist_set);

    let mut new_tags = if new_paths_iter.clone().count() > 0 {
        create_tags(
            app_state,
            new_paths_iter
                .cloned()
                .map(|p| Tag {
                    user_id: user_id.clone(),
                    ..Tag::from_path(p)
                })
                .collect(),
        )
        .await
        .map_err(|_| ())?
    } else {
        Vec::new()
    };

    new_tags.append(&mut exist_tags);
    Ok(new_tags)
}

#[derive(Default)]
pub struct SearchTaggedItem {
    pub tag_id_vec: Option<Vec<String>>,
    pub ref_id_vec: Option<Vec<String>>,
}

pub async fn find_tagged_items(
    app_state: &AppState,
    tagged_type: TaggedType,
    search_params: SearchTaggedItem,
) -> Result<Vec<TaggedItem>, ()> {
    let mut filters = app_state.new_query();
    filters.set_separator(" AND ");
    if let Some(tag_id_vec) = search_params.tag_id_vec.clone() {
        filters.push_str(&format!(
            "tag_id IN ({})",
            vec!["?"; tag_id_vec.len()].join(",")
        ));
        tag_id_vec.into_iter().for_each(|tag_id| {
            filters.bind(tag_id.into());
        });
    }
    if let Some(ref_id_vec) = search_params.ref_id_vec.clone() {
        filters.push_str(&format!(
            "ref_id IN ({})",
            vec!["?"; ref_id_vec.len()].join(",")
        ));
        ref_id_vec.into_iter().for_each(|ref_id| {
            filters.bind(ref_id.into());
        });
    }

    let mut query = app_state.new_query();
    query
        .push_str("SELECT * FROM ")
        .push_str(tagged_type.table());
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
        .map(|row| TaggedItem::try_from(row))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())
}

pub async fn create_tagged_items(
    app_state: &AppState,
    tagged_type: TaggedType,
    item_inputs: Vec<TaggedItem>,
) -> Result<Vec<TaggedItem>, ()> {
    let mut values = app_state.new_query();
    values.set_separator("), (");
    let tagged_items = item_inputs
        .into_iter()
        .map(|input| {
            let item = TaggedItem {
                id: util::new_uid(),
                ..input
            };

            values
                .push_str(&vec!["?"; 4].join(","))
                .bind(item.id.clone().into())
                .bind(item.ref_id.clone().into())
                .bind(item.tag_id.clone().into())
                .bind(item.value.clone().into());

            item
        })
        .collect();

    let mut query = app_state.new_query();
    query
        .push_str("INSERT INTO ")
        .push_str(tagged_type.table())
        .push_str(" (id, ref_id, tag_id, value) VALUES (")
        .append(values)
        .push_str(")");
    app_state
        .database()
        .connection()
        .execute(query)
        .await
        .map_err(|_| ())?;

    Ok(tagged_items)
}

pub async fn update_tagged_items(
    app_state: &AppState,
    tagged_type: TaggedType,
    item_inputs: Vec<TaggedItem>,
) -> Result<Vec<TaggedItem>, ()> {
    let mut values = app_state.new_query();
    values.set_separator("), (");
    let tagged_items = item_inputs
        .into_iter()
        .map(|input| {
            let item = input.clone();
            values
                .push_str(&vec!["?"; 4].join(","))
                .bind(input.id.clone().into())
                .bind(input.ref_id.clone().into())
                .bind(input.tag_id.clone().into())
                .bind(input.value.clone().into());
            item
        })
        .collect();

    let mut query = app_state.new_query();
    query
        .push_str("WITH _data (id, ref_id, tag_id, value) AS ( VALUES (")
        .append(values)
        .push_str(")) UPDATE ")
        .push_str(tagged_type.table())
        .push_str(" AS tagged SET value = _data.value")
        .push_str(" FROM _data")
        .push_str(" WHERE tagged.id = _data.id");
    app_state
        .database()
        .connection()
        .execute(query)
        .await
        .map_err(|_| ())?;

    Ok(tagged_items)
}

pub async fn delete_tagged_items(
    app_state: &AppState,
    tagged_type: TaggedType,
    item_inputs: Vec<TaggedItem>,
) -> Result<Vec<TaggedItem>, ()> {
    let mut values = app_state.new_query();
    values.set_separator("), (");
    let tagged_items = item_inputs
        .into_iter()
        .map(|input| {
            let item = input.clone();
            values
                .push_str(&vec!["?"; 3].join(","))
                .bind(input.id.clone().into())
                .bind(input.ref_id.clone().into())
                .bind(input.tag_id.clone().into());
            item
        })
        .collect();

    let mut query = app_state.new_query();
    query
        .push_str("WITH _data (id, ref_id, tag_id) AS ( VALUES (")
        .append(values)
        .push_str(")) DELETE FROM ")
        .push_str(tagged_type.table())
        .push_str(" AS tagged")
        .push_str(" WHERE EXISTS")
        .push_str(" (SELECT 1 FROM _data WHERE tagged.id = _data.id)");
    app_state
        .database()
        .connection()
        .execute(query)
        .await
        .map_err(|_| ())?;

    Ok(tagged_items)
}

#[derive(Clone)]
struct TaggedItemCmp {
    id: String,
    ref_id: String,
    tag_id: String,
    value: Option<String>,
    order: usize,
}

impl PartialEq for TaggedItemCmp {
    fn eq(&self, other: &Self) -> bool {
        self.tag_id == other.tag_id && self.ref_id == other.ref_id
    }
}

impl Eq for TaggedItemCmp {}

impl std::hash::Hash for TaggedItemCmp {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.tag_id.hash(state);
        self.ref_id.hash(state);
    }
}

impl TaggedItemCmp {
    fn with_order(item: TaggedItem, order: usize) -> Self {
        Self {
            id: item.id,
            ref_id: item.ref_id,
            tag_id: item.tag_id,
            value: item.value,
            order,
        }
    }
}

impl From<TaggedItem> for TaggedItemCmp {
    fn from(item: TaggedItem) -> Self {
        Self {
            id: item.id,
            ref_id: item.ref_id,
            tag_id: item.tag_id,
            value: item.value,
            order: 0,
        }
    }
}

impl Into<TaggedItem> for TaggedItemCmp {
    fn into(self) -> TaggedItem {
        TaggedItem {
            id: self.id,
            ref_id: self.ref_id,
            tag_id: self.tag_id,
            value: self.value,
        }
    }
}

pub async fn sync_tagged_items(
    app_state: &AppState,
    tagged_type: TaggedType,
    item_inputs: Vec<TaggedItem>,
) -> Result<Vec<TaggedItem>, ()> {
    use itertools::Itertools;

    let exist_items = find_tagged_items(
        app_state,
        tagged_type.clone(),
        SearchTaggedItem {
            ref_id_vec: Some(
                item_inputs
                    .clone()
                    .into_iter()
                    .map(|item| item.ref_id)
                    .collect(),
            ),
            ..Default::default()
        },
    )
    .await
    .map_err(|_| ())?;

    let input_set = item_inputs
        .clone()
        .into_iter()
        .enumerate()
        .map(|(index, item)| TaggedItemCmp::with_order(item, index))
        .collect::<HashSet<_>>();
    let exist_set = exist_items
        .clone()
        .into_iter()
        .map(|item| TaggedItemCmp::from(item))
        .collect::<HashSet<_>>();
    let create_items_iter = input_set.difference(&exist_set);
    let update_items_iter = input_set.intersection(&exist_set);
    let delete_items_iter = exist_set.difference(&input_set);

    if log::log_enabled!(log::Level::Debug) {
        let map_item = |t: &TaggedItemCmp| t.tag_id.clone();
        log::debug!(
            "sync tagged items\ncreated {:?}\nupdated {:?}\ndeleted {:?}",
            create_items_iter.clone().map(map_item).collect::<Vec<_>>(),
            update_items_iter.clone().map(map_item).collect::<Vec<_>>(),
            delete_items_iter.clone().map(map_item).collect::<Vec<_>>()
        );
    }

    let mut result: Vec<TaggedItem> = Vec::new();
    if create_items_iter.clone().count() > 0 {
        let mut created_items = create_tagged_items(
            app_state,
            tagged_type.clone(),
            create_items_iter
                .cloned()
                .sorted_by_key(|cmp| cmp.order)
                .map(|cmp| cmp.into())
                .collect::<Vec<_>>(),
        )
        .await
        .map_err(|_| ())?;
        result.append(&mut created_items);
    }
    if update_items_iter.clone().count() > 0 {
        let mut updated_items = &mut update_tagged_items(
            app_state,
            tagged_type.clone(),
            update_items_iter
                .cloned()
                .map(|cmp| cmp.into())
                .collect::<Vec<_>>(),
        )
        .await
        .map_err(|_| ())?;
        result.append(&mut updated_items);
    }
    if delete_items_iter.clone().count() > 0 {
        delete_tagged_items(
            app_state,
            tagged_type.clone(),
            delete_items_iter
                .cloned()
                .map(|cmp| cmp.into())
                .collect::<Vec<_>>(),
        )
        .await
        .map_err(|_| ())?;
    }
    Ok(result)
}

pub struct TaggedResult {
    pub tags: Vec<Tag>,
    pub tagged_type: TaggedType,
    pub tagged_items: Vec<TaggedItem>,
}

impl TaggedResult {
    pub fn find_tags(&self, ref_id: String) -> Vec<TaggedData> {
        self.tagged_items
            .iter()
            .filter(|ti| ti.ref_id == ref_id)
            .map(|ti| {
                TaggedData(
                    self.tags
                        .iter()
                        .find(|t| t.id == ti.tag_id)
                        .unwrap()
                        .clone(),
                    ti.clone(),
                )
            })
            .collect()
    }
}

#[derive(Clone)]
pub struct TaggedData(pub Tag, pub TaggedItem);

pub async fn find_tagged_data_from_refs(
    app_state: &AppState,
    tagged_type: TaggedType,
    ref_id_vec: Vec<String>,
) -> Result<TaggedResult, ()> {
    let tagged_items = find_tagged_items(
        app_state,
        TaggedType::Bookmark,
        SearchTaggedItem {
            ref_id_vec: Some(ref_id_vec),
            ..Default::default()
        },
    )
    .await
    .map_err(|_| ())?;

    let tags = find_tags(
        app_state,
        SearchTag {
            id_vec: Some(tagged_items.clone().into_iter().map(|t| t.tag_id).collect()),
            ..Default::default()
        },
    )
    .await
    .map_err(|_| ())?;

    Ok(TaggedResult {
        tags,
        tagged_type,
        tagged_items,
    })
}

pub async fn sync_tagged_data_from_ref(
    app_state: &AppState,
    user_id: String,
    tagged_type: TaggedType,
    ref_id: String,
    inputs: Vec<TaggedData>,
) -> Result<TaggedResult, ()> {
    let tags = sync_tags(
        app_state,
        user_id,
        inputs.clone().into_iter().map(|data| data.0).collect(),
    )
    .await
    .map_err(|_| ())?;

    let tagged_items = sync_tagged_items(
        app_state,
        tagged_type.clone(),
        inputs
            .clone()
            .into_iter()
            .map(|data| {
                let tag = tags.iter().find(|t| t.path == data.0.path).unwrap();
                TaggedItem {
                    tag_id: tag.id.clone(),
                    ..data.1
                }
            })
            .collect(),
    )
    .await
    .map_err(|_| ())?;

    Ok(TaggedResult {
        tags,
        tagged_type,
        tagged_items,
    })
}

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
    Query(search_params): Query<SearchTag>,
) -> Result<Json<Vec<TagResponse>>, (StatusCode, String)> {
    let tags = find_tags(
        &app_state,
        SearchTag {
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
    let tag = find_tags(
        &app_state,
        SearchTag {
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

    let tag = create_tags(&app_state, vec![value])
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
    let mut tag = find_tags(
        &app_state,
        SearchTag {
            id_vec: Some(vec![tag_id.clone()]),
            user_id: auth.user_id(),
            ..Default::default()
        },
    )
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?
    .pop()
    .ok_or((StatusCode::NOT_FOUND, "".to_string()))?;

    tag.updated_at = Some(util::now());
    let mut values = app_state.new_query();
    values.set_separator(" AND ");
    values
        .push_str("updated_at = ?")
        .bind(tag.updated_at.clone().into());
    if let Some(path) = payload.path {
        let Tag {
            path,
            prefix,
            name,
            depth,
            ..
        } = Tag::from_path(path);
        tag.path = path.clone();
        tag.prefix = prefix.clone();
        tag.name = name.clone();
        tag.depth = depth.clone();
        values
            .push_str("path = ?")
            .bind(path.into())
            .push_str("prefix = ?")
            .bind(prefix.into())
            .push_str("name = ?")
            .bind(name.into())
            .push_str("depth = ?")
            .bind(depth.into());
    }
    if let Some(label) = payload.label.clone() {
        tag.label = Some(label.clone());
        values.push_str("label = ?").bind(label.into());
    }
    if let Some(value_type) = payload.value_type.clone() {
        tag.value_type = Some(value_type.clone());
        values.push_str("value_type = ?").bind(value_type.into());
    }
    let mut query = app_state.new_query();
    query
        .push_str("UPDATE ")
        .push_str(TAG_TABLE)
        .push_str(" SET ")
        .append(values)
        .push_str(" WHERE id = ?")
        .bind(tag_id.into());
    app_state
        .database()
        .connection()
        .execute(query)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?;

    Ok(Json(TagResponse::from(tag)))
}

pub async fn delete(
    Extension(app_state): Extension<AppState>,
    Extension(auth): Extension<AuthenticationState>,
    Path(tag_id): Path<String>,
) -> Result<Json<TagResponse>, (StatusCode, String)> {
    let tag = find_tags(
        &app_state,
        SearchTag {
            id_vec: Some(vec![tag_id.clone()]),
            user_id: auth.user_id(),
            ..Default::default()
        },
    )
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?
    .pop()
    .ok_or((StatusCode::NOT_FOUND, "".to_string()))?;

    let mut query = app_state.new_query();
    query
        .push_str("DELETE FROM ")
        .push_str(TAG_TABLE)
        .push_str(" WHERE id = ?")
        .bind(tag_id.into());
    app_state
        .database()
        .connection()
        .execute(query)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?;

    Ok(Json(TagResponse::from(tag)))
}
