use axum::{Json, Router, routing::get};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
};
use utoipa_scalar::{Scalar, Servable};

use crate::{
    db::schema::user,
    models::user::{AuthBody, AuthPayload, CreateUserPayload, UpdateUserPayload},
};

pub use super::api::users::*;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // Add JWT bearer security scheme component
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    info(title = "Ayiah API", version = "0.1.0", description = "Ayiah Media Server API"),
    paths(
        // Common operations
        openapi,

        // User operations
        register,
        login,
        me,
    ),
    components(
        schemas(
            // Auth schemas
            LoginPayload,
            AuthBody,
            AuthPayload,

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
    ),
    modifiers(&SecurityAddon)
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
