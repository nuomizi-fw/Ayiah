use axum::Router;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::models::{
    user::{CreateUserRequest, UpdateUserRequest},
    user_preferences::SetUserPreferenceRequest,
};

#[derive(OpenApi)]
#[openapi(
    info(title = "Ayiah API", version = "0.1.0", description = "Ayiah Media Server API"),
    components(
        schemas(
            // Auth schemas would go here

            // Entity schemas through aliasing
            // crate::entity::user::Model,
            // crate::entity::user_preferences::Model,

            // Request/response schemas for operations
            CreateUserRequest,
            UpdateUserRequest,
            SetUserPreferenceRequest,
        )
    ),
    tags(
        (name = "auth", description = "Authentication APIs"),
        (name = "users", description = "User management APIs"),
        (name = "libraries", description = "Media library management APIs"),
        (name = "media", description = "Media content management APIs"),
        (name = "playback", description = "Media playback and streaming APIs"),
        (name = "preferences", description = "User preferences APIs")
    )
)]
struct ApiDoc;

pub fn mount() -> Router {
    Router::new().merge(Scalar::with_url("/openapi", ApiDoc::openapi()))
}
