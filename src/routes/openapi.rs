use axum::Router;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::entity::{
    user, user_preferences
};

#[derive(OpenApi)]
#[openapi(
    paths(),
    components(
        schemas(
            // Auth schemas would go here

            // User schemas
            user::Model,
            user::UserResponse,
            user::CreateUserRequest,
            user::UpdateUserRequest,

            // User preference schemas
            user_preferences::Model,
            user_preferences::UserPreferenceResponse,
            user_preferences::SetUserPreferenceRequest,
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
