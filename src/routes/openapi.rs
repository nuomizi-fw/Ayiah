use axum::{Json, Router, routing::get};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::{
    entity::{user, user_preferences},
    models::{CreateUserPayload, SetUserPreferencePayload, UpdateUserPayload},
};

#[derive(OpenApi)]
#[openapi(
    info(title = "Ayiah API", version = "0.1.0", description = "Ayiah Media Server API"),
    paths(
        openapi
    ),
    components(
        schemas(
            // Auth schemas would go here

            // User schemas
            user::Model,
            CreateUserPayload,
            UpdateUserPayload,

            // User preference schemas
            user_preferences::Model,
            SetUserPreferencePayload,
        )
    ),
    tags(
        (name = "Auth", description = "Authentication and authorization"),
        (name = "User", description = "User management"),
        (name = "User Preference", description = "User preference management"),
    )
)]
struct ApiDoc;

/// Return JSON version of an OpenAPI schema
#[utoipa::path(
    get,
    path = "/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

pub fn mount() -> Router {
    Router::new()
        .route("/openapi.json", get(openapi))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
}
