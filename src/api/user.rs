use super::{router::Config, state::AuthenticationState};
use crate::{
    app::{util, AppState},
    database::Connection,
    hash::{hash_password, verify_password},
    model::{User, USER_TABLE},
};
use axum::{
    extract::{Extension, Json, Path, Query, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use chrono::{offset::Utc, DateTime, Duration};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub role: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub role: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct SearchUser {
    pub id: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
}

pub async fn find_users(app_state: &AppState, search_params: SearchUser) -> Result<Vec<User>, ()> {
    let mut filters = app_state.new_query();
    filters.set_separator(" AND ");
    if let Some(id) = search_params.id.clone() {
        filters.push_str("id IN (?)").bind(id.into());
    }
    if let Some(username) = search_params.username.clone() {
        filters.push_str("username IN (?)").bind(username.into());
    }
    if let Some(email) = search_params.email.clone() {
        filters.push_str("email IN (?)").bind(email.into());
    }
    if let Some(role) = search_params.role.clone() {
        filters.push_str("role IN (?)").bind(role.into());
    }

    let mut query = app_state.new_query();
    query.push_str("SELECT * FROM ").push_str(USER_TABLE);
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
        .map(|row| User::try_from(row))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())
}

pub async fn list(
    Extension(app_state): Extension<AppState>,
    Query(search_params): Query<SearchUser>,
) -> Result<Json<Vec<UserResponse>>, (StatusCode, String)> {
    let users = find_users(&app_state, search_params)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?;
    Ok(Json(
        users
            .into_iter()
            .map(|user| UserResponse::from(user))
            .collect(),
    ))
}

pub async fn find_user_by_id(app_state: &AppState, user_id: String) -> Result<User, ()> {
    find_users(
        app_state,
        SearchUser {
            id: Some(user_id),
            ..Default::default()
        },
    )
    .await?
    .pop()
    .ok_or(())
}

pub async fn find(
    Extension(app_state): Extension<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = find_user_by_id(&app_state, user_id)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "".to_string()))?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn create(
    Extension(app_state): Extension<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = User {
        id: util::new_uid(),
        username: payload.username,
        password: hash_password(payload.password)
            .map_err(|_| (StatusCode::BAD_REQUEST, "malformed password".to_string()))?,
        email: payload.email,
        role: payload.role.unwrap_or("user".to_string()),
        created_at: Some(util::now()),
        updated_at: Some(util::now()),
    };
    let mut values = app_state.new_query();
    values.set_separator(", ");
    values
        .push_str("?")
        .bind(user.id.clone().into())
        .push_str(", ?")
        .bind(user.username.clone().into())
        .push_str(", ?")
        .bind(user.password.clone().into())
        .push_str(", ?")
        .bind(user.email.clone().into())
        .push_str(", ?")
        .bind(user.role.clone().into())
        .push_str(", ?")
        .bind(user.created_at.clone().into())
        .push_str(", ?")
        .bind(user.updated_at.clone().into());
    let mut query = app_state.new_query();
    query
        .push_str("INSERT INTO ")
        .push_str(USER_TABLE)
        .push_str(" (id, username, password, email, role, created_at, updated_at) VALUES (")
        .append(values)
        .push_str(")");
    app_state
        .database()
        .connection()
        .execute(query)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(UserResponse::from(user)))
}

pub async fn update(
    Extension(app_state): Extension<AppState>,
    Path(user_id): Path<String>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let mut user = find_user_by_id(&app_state, user_id.clone())
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "".to_string()))?;

    user.updated_at = Some(util::now());
    let mut values = app_state.new_query();
    values.set_separator(", ");
    values
        .push_str("updated_at = ?")
        .bind(user.updated_at.clone().into());
    if let Some(username) = payload.username.clone() {
        user.username = username.clone();
        values.push_str("username = ?").bind(username.into());
    }
    if let Some(password) = payload.password.clone() {
        user.password = hash_password(password)
            .map_err(|_| (StatusCode::BAD_REQUEST, "malformed password".to_string()))?;
        values
            .push_str("password = ?")
            .bind(user.password.clone().into());
    }
    if let Some(email) = payload.email.clone() {
        user.email = Some(email.clone());
        values.push_str("email = ?").bind(email.into());
    }
    if let Some(role) = payload.role.clone() {
        user.role = role.clone();
        values.push_str("role = ?").bind(role.into());
    }
    let mut query = app_state.new_query();
    query
        .push_str("UPDATE ")
        .push_str(USER_TABLE)
        .push_str(" SET ")
        .append(values)
        .push_str(" WHERE id = ?")
        .bind(user_id.clone().into());
    app_state
        .database()
        .connection()
        .execute(query)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(UserResponse::from(user)))
}

pub async fn delete(
    Extension(app_state): Extension<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = find_user_by_id(&app_state, user_id.clone())
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "".to_string()))?;

    let mut query = app_state.new_query();
    query
        .push_str("DELETE FROM ")
        .push_str(USER_TABLE)
        .push_str(" WHERE id = ?")
        .bind(user_id.into());
    app_state
        .database()
        .connection()
        .execute(query)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(UserResponse::from(user)))
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub user_id: String,
}

impl From<User> for Claims {
    fn from(user: User) -> Self {
        Self {
            user_id: user.id.clone(),
            exp: usize::try_from(
                util::now()
                    .checked_add_signed(Duration::days(90))
                    .unwrap()
                    .timestamp(),
            )
            .unwrap(),
        }
    }
}

pub async fn create_token(
    Extension(app_state): Extension<AppState>,
    Extension(config): Extension<Config>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<TokenResponse>, (StatusCode, String)> {
    use jsonwebtoken::{encode, EncodingKey, Header};

    let general_error = (
        StatusCode::BAD_REQUEST,
        "username or password incorrect".to_string(),
    );

    let user = find_users(
        &app_state,
        SearchUser {
            username: Some(payload.username.clone()),
            ..Default::default()
        },
    )
    .await
    .map_err(|_| general_error.clone())?
    .pop()
    .ok_or(general_error.clone())?;

    verify_password(payload.password.clone(), user.password.clone())
        .map_err(|_| general_error.clone())?;

    let claims = Claims::from(user);
    let access_token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.clone().as_bytes()),
    )
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?;
    Ok(Json(TokenResponse { access_token }))
}

pub async fn authenticate(
    Extension(app_state): Extension<AppState>,
    Extension(config): Extension<Config>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    use jsonwebtoken::{decode, DecodingKey, Validation};

    let access_token = request
        .headers()
        .get("authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)
        .and_then(|s| match s.split_once(" ") {
            Some((scheme, token)) => match scheme {
                ref s if s.to_lowercase() == "bearer" => Ok(token),
                _ => Err(StatusCode::UNAUTHORIZED),
            },
            _ => Err(StatusCode::UNAUTHORIZED),
        })?;

    let claims = decode::<Claims>(
        &access_token,
        &DecodingKey::from_secret(config.jwt_secret.clone().as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)
    .map(|d| d.claims)?;

    let user = find_user_by_id(&app_state, claims.user_id)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    request
        .extensions_mut()
        .insert(AuthenticationState::User(user));
    Ok(next.run(request).await)
}
