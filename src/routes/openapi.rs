use axum::{Json, Router, routing::get};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
};
use utoipa_scalar::{Scalar, Servable};

use crate::{app::config::ScrapeConfig, entities::user};

use super::api::{provider::*, scrape::*, users::*};

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
    info(
        title = "Ayiah API",
        version = "0.1.0",
        description = "Ayiah Backend API Documentation",
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        ),
        contact(
            name = "Ns2Kracy",
            url = "https://github.com/Ns2Kracy",
            email = "ns2kracy@gmail.com"
        )
    ),
    paths(
        // Common operations
        openapi,

        // User operations
        register,
        login,
        me,

        // Scrape operations
        scrape,
        manual_match,
        get_scrape_config,
        update_scrape_config,

        // Provider operations
        get_supported_providers,
        test_provider_connection,
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

            // Scrape schemas
            ScrapePayload,
            ManualMatchPayload,
            ScrapeConfig,


            // Provider schemas
            ProviderConnectionTestPayload,
            ProvidersResponse,
            ProviderInfo,

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
