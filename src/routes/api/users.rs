use axum::{
    Router,
    extract::{Extension, Json},
    http::StatusCode,
    routing::{get, post},
};
use chrono::Utc;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    ApiResponse, ApiResult, Ctx,
    db::entity::user,
    error::{ApiError, AyiahError},
    middleware::auth::JwtClaims,
    models::user::{AuthBody, CreateUserPayload},
    routes::service::{mutation::Mutation, query::Query},
    utils::crypto::{generate_salt, hash_password, verify_password},
};

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct LoginPayload {
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

pub fn mount() -> Router {
    Router::new().nest(
        "/users",
        Router::new()
            .route("/register", post(register))
            .route("/login", post(login))
            .route("/me", get(me)),
    )
}

/// Register a new user account
#[utoipa::path(
    post,
    operation_id = "register",
    path = "/api/users/register",
    tag = "Auth",
    request_body = CreateUserPayload,
    responses(
        (status = 200, description = "User registered successfully", body = ()),
        (status = 400, description = "Invalid input data", body = ()),
        (status = 409, description = "Username or email already exists", body = ()),
        (status = 500, description = "Internal server error", body = ()),
    ),
    params(),
    security()
)]
pub async fn register(
    Extension(ctx): Extension<Ctx>,
    Json(payload): Json<CreateUserPayload>,
) -> ApiResult<()> {
    let db = &ctx.db;

    // Check if username already exists
    let user_exists = Query::find_by_username(db, &payload.username).await?;
    if user_exists.is_some() {
        return Err(AyiahError::ApiError(ApiError::Conflict(
            "Username already taken".to_string(),
        )));
    }

    // Check if email already exists
    let email_exists = Query::find_by_email(db, &payload.email).await?;
    if email_exists.is_some() {
        return Err(AyiahError::ApiError(ApiError::Conflict(
            "Email already registered".to_string(),
        )));
    }

    // Check if this is the first user (will be admin)
    let is_first_user = Query::count_users(db).await? == 0;

    // Generate salt and hash password
    let salt = generate_salt();
    let hashed_password = hash_password(&payload.password, &salt);

    // Create new user
    let new_user = user::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        username: ActiveValue::Set(payload.username),
        email: ActiveValue::Set(Some(payload.email)),
        hashed_password: ActiveValue::Set(hashed_password),
        salt: ActiveValue::Set(salt),
        display_name: ActiveValue::Set(payload.display_name),
        avatar: ActiveValue::Set(payload.avatar),
        is_admin: ActiveValue::Set(is_first_user), // First user becomes admin
        created_at: ActiveValue::Set(Utc::now().naive_utc()),
        updated_at: ActiveValue::Set(Utc::now().naive_utc()),
        last_login_at: ActiveValue::Set(None),
    };

    Mutation::create_user(db, new_user).await?;

    Ok(ApiResponse {
        code: StatusCode::OK.into(),
        message: "User registered successfully".to_string(),
        data: None,
    })
}

/// Login to obtain an authentication token
#[utoipa::path(
    post,
    operation_id = "login",
    path = "/api/users/login",
    tag = "Auth",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<AuthBody>),
        (status = 400, description = "Invalid input data", body = ()),
        (status = 401, description = "Invalid username or password", body = ()),
        (status = 500, description = "Internal server error", body = ()),
    ),
    params(),
    security()
)]
pub async fn login(
    Extension(ctx): Extension<Ctx>,
    Json(payload): Json<LoginPayload>,
) -> ApiResult<AuthBody> {
    let db = &ctx.db;

    // Find user by username
    let user = Query::find_by_username(db, &payload.username)
        .await?
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
    user_active.last_login_at = ActiveValue::Set(Some(Utc::now().naive_utc()));
    user_active.updated_at = ActiveValue::Set(Utc::now().naive_utc());

    Mutation::update_user(db, user_active).await?;

    // Generate JWT token
    let claims = JwtClaims::new(user.id.to_string());
    let token = claims.encode_jwt().map_err(AyiahError::AuthError)?;

    Ok(ApiResponse {
        code: StatusCode::OK.into(),
        message: "Login successful".to_string(),
        data: Some(AuthBody::new(token)),
    })
}

/// Get the current authenticated user profile
#[utoipa::path(
    get,
    operation_id = "get_current_user",
    path = "/api/users/me",
    tag = "User",
    responses(
        (status = 200, description = "User profile retrieved", body = ApiResponse<user::Model>),
        (status = 401, description = "Not authenticated", body = ()),
        (status = 404, description = "User not found", body = ()),
    ),
    params(),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn me(Extension(ctx): Extension<Ctx>, claims: JwtClaims) -> ApiResult<user::Model> {
    let db = &ctx.db;

    let user_id = claims
        .sub
        .parse::<Uuid>()
        .map_err(|_| AyiahError::ApiError(ApiError::BadRequest("Invalid user ID".to_string())))?;

    let user = Query::find_by_id(db, user_id)
        .await?
        .ok_or_else(|| AyiahError::ApiError(ApiError::NotFound("User not found".to_string())))?;

    Ok(ApiResponse {
        code: StatusCode::OK.into(),
        message: "User profile retrieved".to_string(),
        data: Some(user),
    })
}
