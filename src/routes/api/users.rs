use axum::{
    Router,
    extract::{Extension, Json},
    http::StatusCode,
    routing::{get, post},
};
use chrono::Utc;
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    ApiResponse, ApiResult,
    app::state::AppState,
    entity::{
        prelude::*,
        user::{self},
    },
    error::{ApiError, AyiahError},
    middleware::auth::{AuthBody, JwtClaims},
    models::user::CreateUserPayload,
    utils::crypto::{generate_salt, hash_password, verify_password},
};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginPayload {
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

pub fn mount() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/me", get(me))
}

async fn register(
    state: Extension<Arc<AppState>>,
    Json(payload): Json<CreateUserPayload>,
) -> ApiResult<()> {
    let db = &*state.db;

    // Check if username already exists
    let user_exists = User::find()
        .filter(user::Column::Username.eq(&payload.username))
        .one(db)
        .await
        .map_err(AyiahError::from)?;

    if user_exists.is_some() {
        return Err(AyiahError::ApiError(ApiError::Conflict(
            "Username already taken".to_string(),
        )));
    }

    // Check if email already exists
    let email_exists = User::find()
        .filter(user::Column::Email.eq(&payload.email))
        .one(db)
        .await
        .map_err(AyiahError::from)?;

    if email_exists.is_some() {
        return Err(AyiahError::ApiError(ApiError::Conflict(
            "Email already registered".to_string(),
        )));
    }

    // Generate salt and hash password
    let salt = generate_salt();
    let hashed_password = hash_password(&payload.password, &salt);

    // Create new user
    let new_user = user::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        username: ActiveValue::Set(payload.username),
        email: ActiveValue::Set(payload.email),
        hashed_password: ActiveValue::Set(hashed_password),
        salt: ActiveValue::Set(salt),
        display_name: ActiveValue::Set(payload.display_name),
        avatar: ActiveValue::Set(payload.avatar),
        is_admin: ActiveValue::Set(false),
        created_at: ActiveValue::Set(Utc::now()),
        updated_at: ActiveValue::Set(Utc::now()),
        last_login_at: ActiveValue::Set(None),
    };

    User::insert(new_user)
        .exec_with_returning(db)
        .await
        .map_err(AyiahError::from)?;

    Ok(ApiResponse {
        code: StatusCode::CREATED.as_u16(),
        message: "User registered successfully".to_string(),
        data: None,
    })
}

async fn login(
    state: Extension<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> ApiResult<AuthBody> {
    let db = &*state.db;

    // Find user by username
    let user = User::find()
        .filter(user::Column::Username.eq(&payload.username))
        .one(db)
        .await
        .map_err(AyiahError::from)?
        .ok_or_else(|| {
            AyiahError::ApiError(ApiError::Unauthorized(
                "Invalid username or password".to_string(),
            ))
        })?;

    // Verify password
    if !verify_password(&payload.password, &user.hashed_password, &user.salt) {
        return Err(AyiahError::ApiError(ApiError::Unauthorized(
            "Invalid username or password".to_string(),
        )));
    }

    // Update last login time
    let mut user_active: user::ActiveModel = user.clone().into();
    user_active.last_login_at = ActiveValue::Set(Some(Utc::now()));
    user_active.updated_at = ActiveValue::Set(Utc::now());

    User::update(user_active)
        .exec(db)
        .await
        .map_err(AyiahError::from)?;

    // Generate JWT token
    let claims = JwtClaims::new(user.id.to_string());
    let token = claims.encode_jwt().map_err(AyiahError::AuthError)?;

    Ok(ApiResponse {
        code: StatusCode::OK.as_u16(),
        message: "Login successful".to_string(),
        data: Some(AuthBody::new(token)),
    })
}

async fn me(state: Extension<Arc<AppState>>, claims: JwtClaims) -> ApiResult<user::Model> {
    let db = &*state.db;

    let user_id = claims
        .sub
        .parse::<Uuid>()
        .map_err(|_| AyiahError::ApiError(ApiError::BadRequest("Invalid user ID".to_string())))?;

    let user = User::find_by_id(user_id)
        .one(db)
        .await
        .map_err(AyiahError::from)?
        .ok_or_else(|| AyiahError::ApiError(ApiError::NotFound("User not found".to_string())))?;

    Ok(ApiResponse {
        code: StatusCode::OK.as_u16(),
        message: "User profile retrieved".to_string(),
        data: Some(user),
    })
}
