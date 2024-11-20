use super::ErrorMsg;
use crate::{app::AppState, database, field, model::User, svc::UserSvc};
use axum::{
    extract::{Json, Path, Request},
    middleware::Next,
    response::Response,
    Extension,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UserResponse {
    pub id: field::Id,
    pub username: String,
    pub email: Option<String>,
    pub role: String,
    pub created_at: field::DateTime,
    pub updated_at: field::DateTime,
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        let User {
            id,
            username,
            email,
            role,
            created_at,
            updated_at,
            ..
        } = value;
        Self {
            id,
            username,
            email,
            role: role.to_string(),
            created_at,
            updated_at,
        }
    }
}

#[derive(Deserialize)]
pub struct UserRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
}

impl UserRequest {
    fn fill_into(self, user: &mut User) -> Result<(), String> {
        if let Some(username) = self.username {
            user.username = username;
        }
        if let Some(password) = self.password {
            user.password = password;
        } else {
            user.password.clear();
        }
        if let Some(email) = self.email {
            user.email = Some(email);
        }
        if let Some(role) = self.role {
            user.role = role.parse().map_err(|_| "invalid user role".to_string())?;
        }
        Ok(())
    }

    async fn validate_create(&self, db: &mut impl database::Ops) -> Result<(), String> {
        let svc = UserSvc::new();

        if let Some(username) = &self.username {
            if let Ok(_) = svc.find_by_username(db, username.clone()).await {
                return Err("username has been taken".to_string());
            }
        } else {
            return Err("username is required".to_string());
        }

        if let Some(password) = &self.password {
            if password.len() == 0 {
                return Err("password cannot be empty".to_string());
            }
        } else {
            return Err("password is required".to_string());
        }

        Ok(())
    }

    async fn validate_update(
        &self,
        db: &mut impl database::Ops,
        user_id: field::Id,
    ) -> Result<(), String> {
        let svc = UserSvc::new();

        if let Some(username) = &self.username {
            if let Ok(user) = svc.find_by_username(db, username.clone()).await {
                if user.id != user_id {
                    return Err("username has been taken".to_string());
                }
            }
        };

        if let Some(password) = &self.password {
            if password.len() == 0 {
                return Err("password cannot be empty".to_string());
            }
        };

        Ok(())
    }

    async fn validate_login(&self, _db: &mut impl database::Ops) -> Result<(), String> {
        if self.username.is_none() || self.password.is_none() {
            return Err("username and password are required".to_string());
        }
        Ok(())
    }
}

pub async fn list(
    Extension(mut state): Extension<AppState>,
) -> Result<Json<Vec<UserResponse>>, ErrorMsg> {
    UserSvc::new()
        .list(&mut state.db)
        .await
        .map_err(|e| ErrorMsg(500, e.to_string()))
        .map(|u| Json(u.into_iter().map(UserResponse::from).collect()))
}

pub async fn find(
    Extension(mut state): Extension<AppState>,
    Path(user_id): Path<field::Id>,
) -> Result<Json<UserResponse>, ErrorMsg> {
    UserSvc::new()
        .find_by_id(&mut state.db, user_id)
        .await
        .map_err(|e| ErrorMsg(500, e.to_string()))
        .map(|u| Json(UserResponse::from(u)))
}

pub async fn create(
    Extension(mut state): Extension<AppState>,
    Json(payload): Json<UserRequest>,
) -> Result<Json<UserResponse>, ErrorMsg> {
    if let Err(msg) = payload.validate_create(&mut state.db).await {
        return Err(ErrorMsg(422, msg));
    }
    let mut user = User::default();
    if let Err(msg) = payload.fill_into(&mut user) {
        return Err(ErrorMsg(422, msg));
    }
    UserSvc::new()
        .create(&mut state.db, user)
        .await
        .map_err(|e| ErrorMsg(500, e.to_string()))
        .map(|u| Json(UserResponse::from(u)))
}

pub async fn update(
    Extension(mut state): Extension<AppState>,
    Path(user_id): Path<field::Id>,
    Json(payload): Json<UserRequest>,
) -> Result<Json<UserResponse>, ErrorMsg> {
    if let Err(msg) = payload
        .validate_update(&mut state.db, user_id.clone())
        .await
    {
        return Err(ErrorMsg(422, msg));
    }
    let svc = UserSvc::new();
    let mut user = svc
        .find_by_id(&mut state.db, user_id)
        .await
        .map_err(|e| ErrorMsg(404, e.to_string()))?;
    if let Err(msg) = payload.fill_into(&mut user) {
        return Err(ErrorMsg(422, msg));
    }
    svc.update(&mut state.db, user)
        .await
        .map_err(|e| ErrorMsg(500, e.to_string()))
        .map(|u| Json(UserResponse::from(u)))
}

pub async fn delete(
    Extension(mut state): Extension<AppState>,
    Path(user_id): Path<field::Id>,
) -> Result<Json<UserResponse>, ErrorMsg> {
    let svc = UserSvc::new();
    let user = svc
        .find_by_id(&mut state.db, user_id)
        .await
        .map_err(|e| ErrorMsg(404, e.to_string()))?;
    svc.delete(&mut state.db, user.clone())
        .await
        .map_err(|e| ErrorMsg(500, e.to_string()))
        .map(|_| Json(UserResponse::from(user)))
}

/// claims
#[derive(Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    user_id: field::Id,
}

impl Claims {
    pub fn with_user(user_id: field::Id) -> Self {
        Self {
            exp: chrono::Utc::now()
                .checked_add_signed(chrono::TimeDelta::days(30))
                .map(|d| usize::try_from(d.timestamp()).unwrap())
                .unwrap(),
            user_id,
        }
    }
}

/// Jwt response
#[derive(Serialize)]
pub struct JwtResponse {
    pub access_token: String,
}

pub async fn create_jwt(
    Extension(mut state): Extension<AppState>,
    Json(payload): Json<UserRequest>,
) -> Result<Json<JwtResponse>, ErrorMsg> {
    if let Err(msg) = payload.validate_login(&mut state.db).await {
        return Err(ErrorMsg(422, msg));
    }
    let svc = UserSvc::new();
    let err_msg = "username or password is invalid";
    let user = svc
        .find_by_username(&mut state.db, payload.username.unwrap())
        .await
        .map_err(|_| ErrorMsg(400, err_msg.to_string()))?;

    svc.verify_password(&payload.password.unwrap(), &user.password)
        .map_err(|_| ErrorMsg(400, err_msg.to_string()))?;

    Ok(Json(JwtResponse {
        access_token: svc
            .jwt_encode(
                &Claims::with_user(user.id.clone()),
                state.jwt_secret.as_str(),
            )
            .map_err(|_| ErrorMsg(400, err_msg.to_string()))?,
    }))
}

#[derive(Clone)]
pub struct AuthState {
    pub user: User,
}

pub async fn authenticate(
    Extension(mut state): Extension<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, ErrorMsg> {
    let err_msg = "unauthorized";
    let token = req
        .headers()
        .get("authorization")
        .and_then(|hv| hv.to_str().ok())
        .and_then(|v| {
            if v.starts_with("Bearer ") {
                Some(v[7..].to_string())
            } else {
                None
            }
        })
        .ok_or(ErrorMsg(401, err_msg.to_string()))?;

    let svc = UserSvc::new();
    let claims = svc
        .jwt_decode::<Claims>(&token, state.jwt_secret.as_str())
        .map_err(|_| ErrorMsg(401, err_msg.to_string()))?;
    let user = svc
        .find_by_id(&mut state.db, claims.user_id)
        .await
        .map_err(|_| ErrorMsg(401, err_msg.to_string()))?;

    req.extensions_mut().insert(AuthState { user });
    Ok(next.run(req).await)
}
