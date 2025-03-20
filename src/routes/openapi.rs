use axum::{Json, Router, routing::get};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::{
    entity::user,
    models::{CreateUserPayload, UpdateUserPayload},
};

#[derive(OpenApi)]
#[openapi(
    info(title = "Ayiah API", version = "0.1.0", description = "Ayiah Media Server API"),
    paths(
        openapi,
    ),
    components(
        schemas(
            // Auth schemas would go here

            // User schemas
            user::Model,
            CreateUserPayload,
            UpdateUserPayload,
        )
    ),
    tags(
        (name = "Common", description = "Common operations"),
        (name = "Auth", description = "Authentication and authorization"),
        (name = "User", description = "User management"),
    )
)]
struct ApiDoc;

/// Return JSON version of an OpenAPI schema
#[utoipa::path(
    get,
    operation_id = "openapi",
    path = "/openapi.json",
    tag = "Common",
    request_body(),
    responses(
        (status = 200, description = "OpenAPI JSON Schema", body = ()),
    ),
    params(),
    security()
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

pub fn mount() -> Router {
    Router::new()
        .route("/openapi.json", get(openapi))
        .merge(Scalar::with_url("/openapi", ApiDoc::openapi()))
}
