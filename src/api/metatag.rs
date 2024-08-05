use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MetatagsRequest {
    pub url: String,
}

#[derive(Serialize, Default)]
pub struct Metatag {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

pub async fn fetch(
    Query(payload): Query<MetatagsRequest>,
) -> Result<Json<Metatag>, (StatusCode, String)> {
    let html = reqwest::get(payload.url.as_str())
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?
        .text()
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?;
    let document = scraper::Html::parse_document(html.as_str());

    match extract_from_plain(&document)
        .or_else(|| extract_from_opengraph(&document))
        .or_else(|| extract_from_twitter(&document))
    {
        Some(m) => Ok(Json(m)),
        None => Err((StatusCode::NOT_FOUND, "".to_string())),
    }
}

fn value_from_select<'a>(select: Option<scraper::ElementRef<'a>>, name: &'a str) -> &'a str {
    select
        .and_then(|e| e.value().attr(name))
        .unwrap_or_default()
}

fn extract_from_plain(document: &scraper::Html) -> Option<Metatag> {
    let title = document
        .select(&scraper::Selector::parse(r#"title"#).unwrap())
        .next();
    let description = document
        .select(&scraper::Selector::parse(r#"meta[name="description"]"#).unwrap())
        .next();

    if let None = title.or(description) {
        None
    } else {
        Some(Metatag {
            title: Some(
                title
                    .and_then(|e| e.text().next())
                    .map(|t| t.trim())
                    .unwrap_or_default()
                    .to_string(),
            ),
            description: Some(value_from_select(description, "content").to_string()),
            image_url: None,
        })
    }
}

fn extract_from_opengraph(document: &scraper::Html) -> Option<Metatag> {
    let title = document
        .select(&scraper::Selector::parse(r#"meta[property="og:title"]"#).unwrap())
        .next();
    let description = document
        .select(&scraper::Selector::parse(r#"meta[property="og:description"]"#).unwrap())
        .next();
    let image_url = document
        .select(&scraper::Selector::parse(r#"meta[property="og:image"]"#).unwrap())
        .next();

    if let None = title.or(description).or(image_url) {
        None
    } else {
        Some(Metatag {
            title: Some(value_from_select(title, "content").to_string()),
            description: Some(value_from_select(description, "content").to_string()),
            image_url: Some(value_from_select(image_url, "content").to_string()),
        })
    }
}

fn extract_from_twitter(document: &scraper::Html) -> Option<Metatag> {
    let title = document
        .select(&scraper::Selector::parse(r#"meta[name="twitter:title"]"#).unwrap())
        .next();
    let description = document
        .select(&scraper::Selector::parse(r#"meta[name="twitter:description"]"#).unwrap())
        .next();
    let image_url = document
        .select(&scraper::Selector::parse(r#"meta[name="twitter:image"]"#).unwrap())
        .next();

    if let None = title.or(description).or(image_url) {
        None
    } else {
        Some(Metatag {
            title: Some(value_from_select(title, "content").to_string()),
            description: Some(value_from_select(description, "content").to_string()),
            image_url: Some(value_from_select(image_url, "content").to_string()),
        })
    }
}
